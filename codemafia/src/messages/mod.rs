/* 
    Messages
    
    Defines the messages that are passed to the room as valid input from any connected player.
*/

use serde::Deserialize;

use self::{chat::ChatMessage, room::RoomMessage, game::GameMessage, internal::InternalMessage};

pub mod chat;
pub mod room;
pub mod game;
pub mod internal;

#[derive(Debug)]
pub enum Message {
    Internal(InternalMessage),
    Client(ClientMessage)
}

#[derive(Debug, Deserialize)]
pub enum ClientMessage {
    Chat(ChatMessage),
    Room(RoomMessage),
    Game(GameMessage)
}

