/* 
    Event
    
    Represents an event that is distributed to one or more players that are in the game.
*/

use tokio::sync::oneshot;

use self::{chat::ChatEvents, game::GameEvents, room::RoomEvents};

pub type EventSender = oneshot::Sender<Event>;

pub struct Event {
    recipient: Recipient,
    content: EventContent
}

/* Defines the different recipients of events, in the context of the game.  */
pub enum Recipient {
    RedSpymaster,
    BlueSpymaster,
    RedUndercover,
    BlueUnderCover,
    Red,
    Blue,
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
