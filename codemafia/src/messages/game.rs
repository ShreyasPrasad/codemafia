/* Defines a game message and its different actions. */

use std::fmt;

use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize)]
pub struct GameMessage {
    pub action: GameMessageAction,
}

#[derive(Debug, Deserialize)]
pub enum GameMessageAction {
    WordSuggested(String, /* The ID of the player suggesting the word. */ u8 /* The word that was suggested by an ally. */),
    WordClicked(String, /* The ID of the player suggesting the word. */ u8 /* The index of the word that was clicked */),
    WordHint(String, /* The ID of the player suggesting the word. */ String /* The word hint provided by the Spymaster at the start of their turn. */),
    EndTurn /* Done by the coodinator for the current turn. */,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum Team {
    Blue,
    Red
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}