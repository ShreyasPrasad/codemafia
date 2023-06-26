/*
    Messages

    Defines the messages that are passed to the room as valid input from any connected player.
*/

use self::{chat::ChatMessage, game::GameMessage, room::RoomMessage};
use serde::Deserialize;

pub mod chat;
pub mod game;
pub mod room;

#[derive(Debug, Deserialize)]
pub enum Message {
    Chat(ChatMessage),
    Room(RoomMessage),
    Game(GameMessage),
}
