/* Defines a game message and its different actions. */

use std::fmt;

#[derive(Debug)]
pub struct GameMessage {
    action: GameMessageAction,
}

#[derive(Debug)]
pub enum GameMessageAction {
    WordSuggested(String /* The word that was suggested */),
    WordClicked(u8 /* The index of the word that was clicked */),
    SpyMasterVoteInitiated,
    EndTurn,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Team {
    Blue,
    Red
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}