/* Defines a room message and its different actions. */
use serde::Deserialize;

use super::game::Team;

#[derive(Debug, Deserialize)]
pub struct RoomMessage {
    pub action: RoomMessageAction
}

#[derive(Debug, Deserialize)]
pub enum RoomMessageAction {
    /* Sent by a connected player when they wish to join a team. */
    JoinTeam(String /* PlayerName */, Team, /* Whether or not the player selected the spymaster role */ bool),
    /* Sent by the game owner when they decide to start the game. */
    StartGame
}