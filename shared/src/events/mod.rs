/*
    Event

    Represents an event that is distributed to one or more players that are in the game.
*/

use self::{chat::ChatEvents, game::GameEvents, player::PlayerEvents, room::RoomEvents};
use serde::Serialize;

pub mod chat;
pub mod game;
pub mod player;
pub mod room;

#[derive(Debug, Clone, Serialize)]
pub enum EventContent {
    Chat(ChatEvents),
    Game(GameEvents),
    Room(RoomEvents),
    Player(PlayerEvents),
}
