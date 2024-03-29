use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        ConnectInfo, Path, State,
    },
    http::StatusCode,
    response::IntoResponse,
};

use axum_extra::extract::cookie::CookieJar;
use shared::{
    events::{game::RoomCode, EventContent},
    player::{PlayerId, PlayerMetadata},
};
use tokio::sync::{mpsc, oneshot};

//allows to split the websocket stream into separate TX and RX branches

use std::{net::SocketAddr, str::FromStr, sync::Arc};

use crate::{
    manager::{controllers::internal::InternalSender, room::MessageSender},
    misc::internal::InternalMessage,
    routes::AppState,
};

use super::{
    util::{get_handles, init_socket, PLAYER_MSPC_BUFFER_SIZE},
    PLAYER_ID_COOKIE_KEY,
};

pub async fn session_route_handler(
    Path(code): Path<RoomCode>, /* The room code the client is attempting to connect to. */
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> impl IntoResponse {
    // check if we have a cookie containing a player ID in the request
    let player_id_str: Option<String> = jar
        .get(PLAYER_ID_COOKIE_KEY)
        .map(|cookie| cookie.value().to_owned());

    match player_id_str {
        Some(player_id) => {
            // check if the game exists
            let handles_opt: Option<(MessageSender, InternalSender)> = get_handles(state, code);

            /* Check if the room exists. */
            match handles_opt {
                /* If the room exists, upgrade the websocket connection. */
                Some(handles) => {
                    let player_id_uuid = PlayerId::from_str(&player_id).unwrap();
                    match check_if_player_exists_in_room(&handles.1, player_id_uuid).await {
                        Some(..) => ws.on_upgrade(move |socket| {
                            handle_socket(socket, addr, handles, player_id_uuid)
                        }),
                        None => (StatusCode::NOT_FOUND, "Player not found.").into_response(),
                    }
                }
                None => (StatusCode::NOT_FOUND, "Room not found.").into_response(),
            }
        }
        None => (StatusCode::NOT_FOUND, "Player ID not found.").into_response(),
    }
}

async fn check_if_player_exists_in_room(
    room_sender: &InternalSender,
    player_id: PlayerId,
) -> Option<PlayerMetadata> {
    let (tx, rx) = oneshot::channel::<Option<PlayerMetadata>>();
    let message_send = room_sender
        .send(InternalMessage::SessionConnection(player_id, tx))
        .await;

    match message_send {
        Ok(..) => match rx.await {
            Ok(player_meta) => player_meta,
            Err(err) => {
                println!("Oneshot channel was cancelled: {}", err);
                None
            }
        },
        Err(err) => {
            println!("Could not send player session connection message: {}", err);
            None
        }
    }
}

async fn handle_socket(
    socket: WebSocket,
    who: SocketAddr,
    handles: (MessageSender, InternalSender),
    player_id: PlayerId,
) {
    // pass the current game state to the player, including existing player state if they are reconnecting
    let (tx, rx) = mpsc::channel::<EventContent>(PLAYER_MSPC_BUFFER_SIZE);
    let player_creation_result = handles
        .1
        .send(InternalMessage::UpdatePlayer(player_id, tx))
        .await;

    init_socket(socket, who, handles, rx, player_creation_result, player_id).await;
}
