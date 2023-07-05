use shared::{
    events::EventContent,
    player::{role::CodeMafiaRole, PlayerId, PlayerMetadata},
};
use tokio::sync::mpsc;

pub type EventSender = mpsc::Sender<EventContent>;
pub const SEND_ERROR_MSG: &'static str = "Failed to send channel message";

#[derive(Debug, Clone)]
pub struct Event {
    pub recipient: Recipient,
    pub content: EventContent,
}

#[derive(Debug, Clone)]
/* Defines the different recipients of events. */
pub enum Recipient {
    /* Specify the recipients by their game role. */
    SingleRoleList(Vec<CodeMafiaRole>),
    /* Specify the recipients by their player ID. */
    SinglePlayerList(Vec<PlayerId>),
    /* Send to all active players. */
    All,
}

/* Function that determines if the given event is meant to be received by the player with the given metadata. */
pub fn is_event_for_player_with_role(event: &Event, player_meta: &PlayerMetadata) -> bool {
    match &event.recipient {
        Recipient::All => true,
        Recipient::SinglePlayerList(players) => players.contains(&player_meta.player_id),
        Recipient::SingleRoleList(roles) => {
            if let Some(player_role) = &player_meta.role {
                return roles.contains(&player_role);
            }
            false
        }
    }
}
