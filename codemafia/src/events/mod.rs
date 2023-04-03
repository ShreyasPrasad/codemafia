/* 
    Event
    
    Represents an event that is distributed to one or more players that are in the game.
*/

use serde::Serialize;
use tokio::sync::mpsc;

use crate::player::{role::CodeMafiaRole, PlayerId};

use self::{chat::ChatEvents, game::GameEvents, room::RoomEvents, player::PlayerEvents};

pub mod chat;
pub mod game;
pub mod room;
pub mod player;

pub type EventSender = mpsc::Sender<EventContent>;

pub struct Event {
    pub recipient: Recipient,
    pub content: EventContent
}

/* Defines the different recipients of events. */
pub enum Recipient {
    /* Specify the recipients by their game role. */
    SingleRoleList(Vec<CodeMafiaRole>),
    /* Specify the recipients by their player ID. */
    SinglePlayerList(Vec<PlayerId>),
    /* Send to all active players. */
    All
}

#[derive(Clone, Serialize)]
pub enum EventContent {
    Chat(ChatEvents),
    Game(GameEvents),
    Room(RoomEvents),
    Player(PlayerEvents)
}
