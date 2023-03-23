/* 
    Player
 
    Encapsulates basic active player fields with several generic fields
    for custom player implementations.
*/

use std::hash::{Hash, Hasher};

pub mod connection;

use crate::{player::connection::Connection, game::event::EventSender};

pub struct Player {
    /* The player's communication channel.  */
    channel: PlayerChannel,
    /* The player's role. */
    profile: Option<Box<dyn Profile + Send + Sync>>,
    /* The player's self-assigned name. */
    name: Option<String>
}

pub struct PlayerChannel {
    event_sender: EventSender,
    connection: Connection
}

/* Trait that designates a player's profile; this can be used for whatever 
purpose the game requires; in our case it will store the player's name and role. */
pub trait Profile {
    fn get_role_str(&self) -> String;
    fn get_name_str(&self) -> String;
}

impl Player {
    pub fn new(name: String, event_sender: EventSender, connection: Connection) -> Self {
        Player {
            channel: PlayerChannel { event_sender, connection },
            profile: None,
            name: Some(name)
        }
    }
}

/* Blanket Eq impl for Player. */
impl Eq for Player {}

/* Allow Player to be used in hash-based datastructures like HashSet, using the 
underlying Connection. */ 
impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.channel.connection == other.channel.connection
    }
}

impl Hash for Player {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.channel.connection.hash(state)
    }
}
