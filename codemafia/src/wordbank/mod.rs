/* WordBank
  
 The WordBank module is reponsible for generating 25 words that can constitute a valid codenames game.
 It assembles a complete list of words in memory and uses them to assemble valid codenames games upon
 request. The data structures that make up a game are defined in this module, along with the logic to
 populate them.

 */

use serde::Serialize;

pub mod creator;

pub const NUM_BLUE_WORDS: usize = 8;
pub const NUM_RED_WORDS: usize = 9;
pub const NUM_BLACK_WORDS: usize = 1;

/* Defines the possible types of a word in the game. */
#[derive(Debug, Clone, Serialize)]
pub enum WordType {
    Black,
    Normal,
    Blue, 
    Red
}

#[derive(Debug)]
pub struct Word {
    text: String,
    word_type: WordType
}

#[derive(Debug)]
pub struct Board {
    words: Vec<Word>
}

/* The complete definition of a codenames game, accessible to callers, such as the gameserver. */
#[derive(Debug)]
pub struct Game {
    board: Board
}
