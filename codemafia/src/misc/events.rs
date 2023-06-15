use shared::{
    events::EventContent,
    player::{role::CodeMafiaRole, PlayerId},
};
use tokio::sync::mpsc;

pub type EventSender = mpsc::Sender<EventContent>;
pub const SEND_ERROR_MSG: &'static str = "Failed to send channel message";

#[derive(Debug)]
pub struct Event {
    pub recipient: Recipient,
    pub content: EventContent,
}

#[derive(Debug)]
/* Defines the different recipients of events. */
pub enum Recipient {
    /* Specify the recipients by their game role. */
    SingleRoleList(Vec<CodeMafiaRole>),
    /* Specify the recipients by their player ID. */
    SinglePlayerList(Vec<PlayerId>),
    /* Send to all active players. */
    All,
}
