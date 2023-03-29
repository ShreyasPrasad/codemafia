/* Defines a room message and its different actions. */

use crate::player::PlayerId;

use super::game::Team;

#[derive(Debug)]
pub struct RoomMessage {
    pub action: RoomMessageAction
}

#[derive(Debug)]
pub enum RoomMessageAction {
    /* This message is sent by the websocket statemachine upon receiving a player connection. */
    InitialConnection(PlayerId),
    /* Sent by a connected player when they wish to join a team. */
    JoinTeam(String, Team),
    /* Sent by the game owner when they decide to start the game. */
    StartGame
}