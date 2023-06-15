use axum::{routing::get, Router};

use tower_http::trace::{DefaultMakeSpan, TraceLayer};

use std::sync::RwLock;

use crate::manager::RoomManager;

use std::sync::Arc;

use self::{
    create::create_route_handler,
    game::{game::game_route_handler, session::session_route_handler},
};

/* Declare the shared state for routing games. */

pub struct AppState {
    manager: RwLock<RoomManager>,
}

/* Create: used to create a new CodeMafia game, and obtain the corresponding game code. */
pub mod create;
/* Game: allows a player to initiate a connection to the game room and server. */
pub mod game;

/* Function that builds all the app's public routes. */
pub fn build_routes() -> Router {
    // initialize our shared state
    let shared_state = Arc::new(AppState {
        manager: RwLock::new(RoomManager::new()),
    });

    let create_state = shared_state.clone();
    let game_session_state = shared_state.clone();
    // build our application with some routes
    Router::new()
        .route(
            "/game/join/:code",
            get(game_route_handler).with_state(shared_state),
        )
        .route(
            "/game/session/:code",
            get(session_route_handler).with_state(game_session_state),
        )
        .route(
            "/create",
            get(create_route_handler).with_state(create_state),
        )
        // logging so we can see whats going on
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
}
