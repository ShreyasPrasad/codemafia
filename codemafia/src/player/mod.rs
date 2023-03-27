/* 
    Player
 
    Encapsulates basic active player fields with several generic fields
    for custom player implementations.
*/

use std::hash::{Hash, Hasher};
use uuid::Uuid;

pub mod role;

use crate::events::EventSender;

use self::role::Role;

/* Convenient alias for the player ID. */
pub type PlayerId = Uuid;

pub struct Player {
    /* The player's unique ID. */
    pub player_id: PlayerId,
    /* The channel used to communicate with the player's websocket sender. */
    pub channel: PlayerChannel,
    /* The player's role. */
    pub role: Option<Box<dyn Role + Send + Sync>>,
    /* The player's self-assigned name. */
    pub name: Option<String>
}

pub struct PlayerChannel {
    pub event_sender: EventSender,
}

impl Player {
    pub fn new(name: String, event_sender: EventSender) -> Self {
        Player {
            channel: PlayerChannel { event_sender },
            role: None,
            name: Some(name),
            player_id: Uuid::new_v4()
        }
    }
}

/* Blanket Eq impl for Player. */
impl Eq for Player {}

/* Allow Player to be used in hash-based data structures like HashSet, using the 
underlying player ID UUID as the key. */ 
impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.player_id == other.player_id
    }
}

impl Hash for Player {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.player_id.hash(state)
    }
}
