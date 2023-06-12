use axum::{
    extract::{
        ws::{Message as AxumMessage, WebSocket},
    },
};

use shared::{events::{game::{RoomCode, GameEvents}, EventContent}, messages::Message};
//allows to split the websocket stream into separate TX and RX branches
use futures::{stream::StreamExt, SinkExt};
use tokio::sync::mpsc::{Receiver, error::SendError};

use std::{net::SocketAddr, sync::Arc};

use crate::{manager::{room::MessageSender, controllers::internal::InternalSender}, routes::AppState, misc::internal::InternalMessage};

pub const PLAYER_MSPC_BUFFER_SIZE: usize = 4;

pub fn get_handles(state: Arc<AppState>, code: RoomCode) -> Option<(MessageSender, InternalSender)> {
    // check if the game exists
    let mut handles_opt: Option<(MessageSender, InternalSender)> = None;
    {
        match state.manager.read() {
            Ok(manager_lock) => {
                handles_opt = manager_lock.get_handles(code);
            },
            Err(err) => {
                println!("Error encountered when acquiring manager RwLock in read mode: {}", err);
            } 
        }
    }
    handles_opt
}

pub async fn spawn_game_connection(socket: WebSocket, who: SocketAddr, event_sender: MessageSender, mut rx: Receiver<EventContent>) {
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
                            if let Err(err) = sender.send(AxumMessage::Text(parsed_content)).await{
                                println!("Error sending event to player: {}", err);
                            }
                            /* We are done if we read a GameEnded message; exit the send task. */
                            if let EventContent::Game(GameEvents::GameEnded(_)) = content {
                                break;
                            }
                        }, 
                        Err(err) => {
                            cnt = cnt + 1;
                            println!("Error serializing event content to string {}", err);
                        }
                    }
                },
                None => break
            }
        }
        cnt
    });

    // This second task will receive messages from client and print them on server console
    let mut recv_task = tokio::spawn(async move {
        let mut cnt = 0;
        while let Some(Ok(msg)) = receiver.next().await {
            // deserialize the message and forward it to the room so it can be routed appropriately
            match msg.to_text() {
                Ok(msg_text) => {
                    cnt = cnt + 1;
                    match serde_json::from_str::<Message>(msg_text) {
                        Ok(msg_struct) => {
                            if let Err(err) = event_sender.send(msg_struct).await {
                                println!("Error sending client message to room: {}", err);
                            }
                        },
                        Err(err) => {
                            println!("Error deserializing message from websocket string {}", err);
                        }
                    }
                },
                Err(err) => {
                    cnt = cnt + 1;
                    println!("Unexpected error decoding client websocket message: {}", err);
                }
            }
        }
        /* if the socket is closed, mark the player as disconnected. */
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
        },
        rv_b = (&mut recv_task) => {
            match rv_b {
                Ok(b) => println!("Received {} messages", b),
                Err(b) => println!("Error receiving messages {:?}", b)
            }
            send_task.abort();
        }
    }

    // returning from the handler closes the websocket connection
    println!("Websocket context {} destroyed", who);
}

/// Actual websocket statemachine (one will be spawned per connection)
pub async fn init_socket(socket: WebSocket, who: SocketAddr, msg_sender: MessageSender, rx: Receiver<EventContent>, create_result: Result<(), SendError<InternalMessage>>) {
    if let Err(err) = create_result {
        println!("Error creating player: {}. Closing socket.", err);
        if let Err(s_err) = socket.close().await {
            println!("Error closing socket: {}", s_err);
        }
    } else {
        spawn_game_connection(socket, who, msg_sender, rx).await;
    }    
}
