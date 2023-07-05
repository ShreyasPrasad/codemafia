/*
    Player

    Encapsulates basic active player fields with several generic fields
    for custom player implementations.
*/

use std::{
    fmt,
    hash::{Hash, Hasher},
};
use uuid::Uuid;

pub mod role;
use self::role::CodeMafiaRole;

/* Convenient alias for the player ID. */
pub type PlayerId = Uuid;

#[derive(Debug, Clone)]
pub struct PlayerMetadata {
    /* The player's unique ID. */
    pub player_id: PlayerId,
    /* The player's role. */
    pub role: Option<CodeMafiaRole>,
    /* The player's self-assigned name. */
    pub name: Option<String>,
}

impl PlayerMetadata {
    pub fn new(name: String) -> Self {
        PlayerMetadata {
            role: None,
            name: Some(name),
            player_id: Uuid::new_v4(),
        }
    }
}

/* Blanket Eq impl for Player. */
impl Eq for PlayerMetadata {}

/* Allow Player to be used in hash-based data structures like HashSet, using the
underlying player ID UUID as the key. */
impl PartialEq for PlayerMetadata {
    fn eq(&self, other: &Self) -> bool {
        self.player_id == other.player_id
    }
}

impl Hash for PlayerMetadata {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.player_id.hash(state)
    }
}

/* Declare error types for the player */

// Define an error type for errors that occur when the creator is instantiated
#[derive(Debug, Clone)]
pub enum PlayerError {
    DoesNotExist,
}

impl fmt::Display for PlayerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::DoesNotExist => write!(f, "The player does not exist."),
        }
    }
}
