use axum::{
    extract::State,
    response::IntoResponse, http::StatusCode,
};

use std::sync::Arc;

use crate::manager::RoomCode;

use super::AppState;

pub async fn create_route_handler(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let mut room_code: Option<RoomCode> = None;
    {
        match state.manager.write() {
            Ok(mut manager_lock) => {
                room_code = Some(manager_lock.create_room());
            },
            Err(err) => {
                println!("Error encountered when acquiring manager RwLock in read mode: {}", err);
            } 
        }
    }

    /* Check if we have a valid new RoomCode, corresponding to the new game. */
    match room_code {
        Some(room_code) => (StatusCode::OK, room_code).into_response(),
        None => (StatusCode::INTERNAL_SERVER_ERROR, "Could not create new game.").into_response()
    }
}