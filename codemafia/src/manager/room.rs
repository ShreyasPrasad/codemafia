/*
    Room

    This module contains the struct that organizes players (see mod player) into a single group, allowing
    broadcast communication. Access to this struct is not thread-safe and should be made synchronous
    using some concurrency primitive (see the use of Dashmap in mod,rs).
*/

use crate::{player::Player, game::server::GameServer};
use std::collections::HashSet;

pub struct Room {
    /* The active players in the room. */
    players: HashSet<Player>,
    game: Option<GameServer>
}

impl Room {
    pub fn new() -> Self {
        Room { players: HashSet::new(), game: None }
    }
}
