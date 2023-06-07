/* 
    Event
    
    Represents an event that is distributed to one or more players that are in the game.
*/

use serde::Serialize;
use self::{chat::ChatEvents, game::GameEvents, room::RoomEvents, player::PlayerEvents};

pub mod chat;
pub mod game;
pub mod room;
pub mod player;

#[derive(Debug, Clone, Serialize)]
pub enum EventContent {
    Chat(ChatEvents),
    Game(GameEvents),
    Room(RoomEvents),
    Player(PlayerEvents)
}
