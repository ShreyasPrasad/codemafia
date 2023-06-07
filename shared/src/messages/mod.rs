/* 
    Messages
    
    Defines the messages that are passed to the room as valid input from any connected player.
*/

use serde::Deserialize;
use self::{chat::ChatMessage, room::RoomMessage, game::GameMessage};

pub mod chat;
pub mod room;
pub mod game;

#[derive(Debug, Deserialize)]
pub enum Message {
    Chat(ChatMessage),
    Room(RoomMessage),
    Game(GameMessage)
}
