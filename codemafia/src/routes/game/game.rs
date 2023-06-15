use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        Path, Query, State,
    },
    http::StatusCode,
    response::IntoResponse,
};

//allows to extract the IP of connecting user
use axum::extract::connect_info::ConnectInfo;

use crate::{manager::controllers::internal::InternalSender, misc::internal::InternalMessage};
use shared::events::{game::RoomCode, EventContent};
//allows to split the websocket stream into separate TX and RX branches
use tokio::sync::mpsc;

use serde::Deserialize;
use std::{net::SocketAddr, sync::Arc};

use super::util::{init_socket, PLAYER_MSPC_BUFFER_SIZE};
use crate::{manager::room::MessageSender, routes::AppState};

#[derive(Deserialize)]
pub struct NewPlayerFields {
    pub name: String,
}

pub async fn game_route_handler(
    Path(code): Path<RoomCode>, /* The room code the client is attempting to connect to. */
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Query(new_player_fields): Query<NewPlayerFields>,
) -> impl IntoResponse {
    // check if the game exists
    let mut handles_option: Option<(MessageSender, InternalSender)> = None;
    {
        match state.manager.read() {
            Ok(manager_lock) => {
                handles_option = manager_lock.get_handles(code);
            }
            Err(err) => {
                println!(
                    "Error encountered when acquiring manager RwLock in read mode: {}",
                    err
                );
            }
        }
    }

    /* Check if the room exists. */
    match handles_option {
        /* If the room exists, upgrade the websocket connection. */
        Some(handles) => ws
            .on_upgrade(move |socket| handle_socket(socket, addr, handles, new_player_fields.name)),
        None => (StatusCode::NOT_FOUND, "Room not found.").into_response(),
    }
}

/// Actual websocket statemachine (one will be spawned per connection)
async fn handle_socket(
    socket: WebSocket,
    who: SocketAddr,
    handles: (MessageSender, InternalSender),
    player_name: String,
) {
    // pass the current game state to the player, including existing player state if they are reconnecting
    let (tx, rx) = mpsc::channel::<EventContent>(PLAYER_MSPC_BUFFER_SIZE);
    let player_creation_result = handles
        .1
        .send(InternalMessage::NewPlayer(player_name, tx))
        .await;

    init_socket(socket, who, handles.0, rx, player_creation_result).await;
}
