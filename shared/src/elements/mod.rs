use serde::Serialize;

pub const NUM_BLUE_WORDS: usize = 8;
pub const NUM_RED_WORDS: usize = 9;
pub const NUM_BLACK_WORDS: usize = 1;

/* Defines the possible types of a word in the game. */
#[derive(Debug, Copy, Clone, Serialize)]
pub enum WordType {
    Black,
    Normal,
    Blue,
    Red,
}

#[derive(Clone, Debug, Serialize)]
pub struct Word {
    pub text: String,
    pub word_type: WordType,
    pub clicked: bool,
}

#[derive(Debug)]
pub struct Board {
    pub words: Vec<Word>,
}

/* The complete definition of a codenames game, accessible to callers, such as the gameserver. */
#[derive(Debug)]
pub struct Game {
    pub board: Board,
}
