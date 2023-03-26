/* Defines a room message and its different actions. */

use super::game::Team;

#[derive(Debug)]
pub struct RoomMessage {
    action: RoomMessageAction
}

#[derive(Debug)]
pub enum RoomMessageAction {
    /* This message is sent by the websocket statemachine upon receiving a player connection. */
    InitialConnection,
    /* Sent by a connected player when they wish to join a team. */
    JoinTeam(Team),
    /* Sent by the game creator when they decide to start the game. */
    StartGame
}