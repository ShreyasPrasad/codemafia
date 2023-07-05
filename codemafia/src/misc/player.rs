use shared::player::PlayerMetadata;
use uuid::Uuid;

use super::events::EventSender;

/*
   This struct represents an active player, which comprises of the basic player struct,
   along with a stateful player connection.
*/
pub struct ActivePlayer {
    pub meta: PlayerMetadata,
    pub connection: PlayerConnection,
}

pub struct PlayerConnection {
    /* The player's connection status. */
    pub status: PlayerStatus,
    /* The channel used to communicate with the player's websocket sender. */
    pub event_sender: EventSender,
}

pub enum PlayerStatus {
    Connected,
    Disconnected,
}

impl ActivePlayer {
    pub fn new(name: String, event_sender: EventSender) -> Self {
        ActivePlayer {
            meta: PlayerMetadata {
                role: None,
                name: Some(name),
                player_id: Uuid::new_v4(),
            },
            connection: PlayerConnection {
                status: PlayerStatus::Connected,
                event_sender,
            },
        }
    }
}
