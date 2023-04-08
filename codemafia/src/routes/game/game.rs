use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        TypedHeader, Path, State, Query
    },
    response::IntoResponse, http::StatusCode,
};

//allows to extract the IP of connecting user
use axum::extract::connect_info::ConnectInfo;

//allows to split the websocket stream into separate TX and RX branches
use tokio::sync::mpsc;

use std::{net::SocketAddr, sync::Arc};
use serde::Deserialize;

use crate::{manager::{RoomCode, room::MessageSender}, routes::AppState, messages::{Message::Internal, internal::InternalMessage}, events::EventContent};
use super::util::spawn_game_connection;

pub const PLAYER_MSPC_BUFFER_SIZE: usize = 4;

#[derive(Deserialize)]
pub struct NewPlayerFields {
    pub name: String
}

pub async fn game_route_handler(
    Path(code): Path<RoomCode>, /* The room code the client is attempting to connect to. */
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Query(new_player_fields): Query<NewPlayerFields>
) -> impl IntoResponse {

    // check if the game exists
    let mut room_handle: Option<MessageSender> = None;
    {
        match state.manager.read() {
            Ok(manager_lock) => {
                room_handle = manager_lock.get_room_handle(code)
            },
            Err(err) => {
                println!("Error encountered when acquiring manager RwLock in read mode: {}", err);
            } 
        }
    }

    /* Check if the room exists. */
    match room_handle {
        /* If the room exists, upgrade the websocket connection. */
        Some(sender) => ws.on_upgrade(
            move |socket| handle_socket(socket, addr, sender, new_player_fields.name)),
        None => (StatusCode::NOT_FOUND, "Room not found.").into_response()
    }
}

/// Actual websocket statemachine (one will be spawned per connection)
async fn handle_socket(socket: WebSocket, who: SocketAddr, msg_sender: MessageSender, player_name: String) {
    // pass the current game state to the player, including existing player state if they are reconnecting
    let (tx, rx) = mpsc::channel::<EventContent>(PLAYER_MSPC_BUFFER_SIZE);
    let player_creation_result  = msg_sender.send(Internal(InternalMessage::NewPlayer(player_name, tx)))
    .await;

    if let Err(err) = player_creation_result {
        socket.close();
    } else {
        spawn_game_connection(socket, who, msg_sender, rx);
    }    
}
