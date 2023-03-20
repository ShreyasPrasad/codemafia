/*
    Connection

    This module contains a struct with metadata with clients (such as players) that initiate
    connections to the server. This is useful for identifying unique players and enabling
    client reconnections based on existing Connection entries.

    To allow for the use of a Connection as a hash key, the struct derives the Eq trait, with custom
    implementations of PartialEq and Hash, which depend solely on the 
*/

use std::net::SocketAddr;
use std::time::Instant;

use std::hash::{Hash, Hasher};

#[derive(Eq)]
pub struct Connection {
    addr: SocketAddr,
    init_time: Instant
}

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.addr == other.addr
    }
}

impl Hash for Connection {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.addr.hash(state)
    }
}

