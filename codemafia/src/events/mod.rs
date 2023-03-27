/* 
    Event
    
    Represents an event that is distributed to one or more players that are in the game.
*/

use tokio::sync::oneshot;

use crate::player::role::CodeMafiaRole;

use self::{chat::ChatEvents, game::GameEvents, room::RoomEvents};

pub type EventSender = oneshot::Sender<EventContent>;

pub struct Event {
    pub recipient: Recipient,
    pub content: EventContent
}

/* Defines the different recipients of events, in the context of the game.  */
pub enum Recipient {
    SingleRoleList(Vec<CodeMafiaRole>),
    All
}

pub enum EventContent {
    Chat(ChatEvents),
    Game(GameEvents),
    Room(RoomEvents)
}

pub mod chat;
pub mod game;
pub mod room;
