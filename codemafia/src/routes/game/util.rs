use axum::extract::ws::{Message as AxumMessage, WebSocket};

use shared::{
    events::{
        game::{GameEvents, RoomCode},
        EventContent,
    },
    messages::Message,
    player::PlayerId,
};
//allows to split the websocket stream into separate TX and RX branches
use futures::{stream::StreamExt, SinkExt};
use tokio::sync::mpsc::{error::SendError, Receiver};

use std::{error::Error, net::SocketAddr, sync::Arc};

use crate::{
    manager::{controllers::internal::InternalSender, room::MessageSender},
    misc::{events::SEND_ERROR_MSG, internal::InternalMessage},
    routes::AppState,
};

pub const PLAYER_MSPC_BUFFER_SIZE: usize = 4;

pub fn get_handles(
    state: Arc<AppState>,
    code: RoomCode,
) -> Option<(MessageSender, InternalSender)> {
    // check if the game exists
    let mut handles_opt: Option<(MessageSender, InternalSender)> = None;
    {
        match state.manager.read() {
            Ok(manager_lock) => {
                handles_opt = manager_lock.get_handles(code);
            }
            Err(err) => {
                println!(
                    "Error encountered when acquiring manager RwLock in read mode: {}",
                    err
                );
            }
        }
    }
    handles_opt
}

pub async fn spawn_game_connection(
    socket: WebSocket,
    who: SocketAddr,
    player_id: PlayerId,
    message_sender: MessageSender,
    internal_sender: InternalSender,
    mut rx: Receiver<EventContent>,
) {
    // By splitting socket we can send and receive at the same time. In this example we will send
    // unsolicited messages to client based on some sort of server's internal event (i.e .timer).
    let (mut sender, mut receiver) = socket.split();

    // Spawn a task that will push several messages to the client (does not matter what client does)
    let mut send_task = tokio::spawn(async move {
        let mut cnt = 0;
        loop {
            /* Use a blocking recv here to avoid cluttering Tokio runtime scheduler with too many idle listeners. */
            let event_content = rx.blocking_recv();
            match event_content {
                Some(content) => {
                    match serde_json::to_string(&content) {
                        Ok(parsed_content) => {
                            cnt = cnt + 1;
                            if let Err(err) = sender.send(AxumMessage::Text(parsed_content)).await {
                                println!("Error sending event to player: {}", err);
                            }
                            /* We are done if we read a GameEnded message; exit the send task. */
                            if let EventContent::Game(GameEvents::GameEnded(_)) = content {
                                break;
                            }
                        }
                        Err(err) => {
                            cnt = cnt + 1;
                            println!("Error serializing event content to string {}", err);
                        }
                    }
                }
                None => break,
            }
        }
        cnt
    });

    // This second task will receive messages from the client
    let mut recv_task = tokio::spawn(async move {
        let mut cnt = 0;
        while let Some(Ok(msg)) = receiver.next().await {
            // deserialize the message and forward it to the room so it can be routed appropriately
            match msg.to_text() {
                Ok(msg_text) => {
                    cnt = cnt + 1;
                    match serde_json::from_str::<Message>(msg_text) {
                        Ok(msg_struct) => {
                            if let Err(err) = message_sender.send(msg_struct).await {
                                println!("Error sending client message to room: {}", err);
                            }
                        }
                        Err(err) => {
                            println!("Error deserializing message from websocket string {}", err);
                        }
                    }
                }
                Err(err) => {
                    cnt = cnt + 1;
                    println!(
                        "Unexpected error decoding client websocket message: {}",
                        err
                    );
                }
            }
        }
        cnt
    });

    // If any one of the tasks exit, abort the other.
    tokio::select! {
        rv_a = (&mut send_task) => {
            match rv_a {
                Ok(a) => println!("{} messages sent to {}", a, who),
                Err(a) => println!("Error sending messages {:?}", a)
            }
            recv_task.abort();
            mark_player_as_disconnected(player_id, internal_sender).await;
        },
        rv_b = (&mut recv_task) => {
            match rv_b {
                Ok(b) => println!("Received {} messages", b),
                Err(b) => println!("Error receiving messages {:?}", b)
            }
            send_task.abort();
            mark_player_as_disconnected(player_id, internal_sender).await;
        }
    }

    // returning from the handler closes the websocket connection
    println!("Websocket context {} destroyed", who);
}

/// Actual websocket statemachine (one will be spawned per connection)
pub async fn init_socket(
    socket: WebSocket,
    who: SocketAddr,
    handles: (MessageSender, InternalSender),
    rx: Receiver<EventContent>,
    create_result: Result<(), SendError<InternalMessage>>,
    player_id: PlayerId,
) {
    if let Err(err) = create_result {
        close_socket_after_unrecoverable_error(socket, err.into()).await;
    } else {
        spawn_game_connection(socket, who, player_id, handles.0, handles.1, rx).await;
    }
}

pub async fn close_socket_after_unrecoverable_error(
    socket: WebSocket,
    err: Box<dyn Error + Send + Sync>,
) {
    println!("Error creating player: {}. Closing socket.", err);
    if let Err(s_err) = socket.close().await {
        println!("Error closing socket: {}", s_err);
    }
}

/* Sends an internal message to mark a player as disconnected. */
async fn mark_player_as_disconnected(player_id: PlayerId, internal_sender: InternalSender) {
    internal_sender
        .send(InternalMessage::PlayerDisconnected(player_id))
        .await
        .expect(SEND_ERROR_MSG)
}

/* Sends an internal message to mark a player as connected, if they were previously disconnected. */
async fn mark_player_as_connected(player_id: PlayerId, internal_sender: InternalSender) {
    internal_sender
        .send(InternalMessage::PlayerReconnected(player_id))
        .await
        .expect(SEND_ERROR_MSG)
}
