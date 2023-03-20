/* 
    Player
 
    Encapsulates basic active player fields with several generic fields
    for custom player implementations.

*/

use std::hash::{Hash, Hasher};

pub mod connection;

use axum::extract::ws::WebSocket;
use crate::player::connection::Connection;

pub struct Player {
    /* The player's communication channel.  */
    channel: PlayerChannel,
    /* The player's role. */
    role: Option<Box<dyn Role>>,
    /* The player's self-assigned name. */
    name: Option<String>
}

pub struct PlayerChannel {
    socket: WebSocket,
    connection: Connection
}

/* Trait that designates a player's role; this can be used for whatever 
purpose the game requires; in our case it will store the game's roles. */
pub trait Role {
    fn get_role_str(&self) -> String;
}

impl Player {
    pub fn new(name: String, socket: WebSocket, connection: Connection) -> Self {
        Player {
            channel: PlayerChannel { socket, connection },
            role: None,
            name: Some(name)
        }
    }
}

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
