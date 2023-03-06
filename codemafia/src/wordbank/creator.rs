/* Creator
  
 This module reads word lists to assemble an in memory list of words that can be used in 
 new codenames games. It creates a buffer of games that is replenished when it is emptied
 by the caller.

 TODO: Make the replenishment of this game buffer asynchronous after a certain threshold number
 of games is reached. Otherwise, the caller that obtains the last buffered Game incurs a penalty
 to calculate the next buffer for other callers; this is not ideal.
 
 */


use super::*;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashSet;
use std::fmt;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use rand::prelude::*;

const GAME_BUFFER_SIZE: usize = 5;
const MINIMUM_WORDBANK_SIZE: usize = 200;
const NUMBER_OF_WORDS_IN_GAME: usize = 25;
const WORD_FILES: [&'static str; 1] = ["wordlist"];

pub struct Creator {
    all_words: Vec<String>,
    game_buffer: VecDeque<Game>,
    rng: ChaCha20Rng  // use ChaCha20Rng since it implements Send+Sync
}

// Define an error type for errors that occur when the creator is instantiated
#[derive(Debug, Clone)]
pub enum CreatorError {
    NotEnoughWords
}

impl fmt::Display for CreatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotEnoughWords => write!(f, "Not enough words were read.")
        }   
    }
}

impl Creator {
    pub fn new() -> Result<Self, CreatorError> {
        // get the list of words and initialize the buffer with GAME_BUFFER_SIZE games
       match Self::get_all_words() {
            Ok(word_list) => 
                Ok(Creator {
                    all_words: word_list,
                    game_buffer: VecDeque::new(),
                    rng: ChaCha20Rng::from_entropy()
                }
            ),
            Err(err) => Err(err)
        }
    }

    pub fn get_game (&mut self) -> Game {
        if self.game_buffer.is_empty() {
            self.populate_buffer();
        }
        self.game_buffer.pop_front().unwrap()
    }

    fn populate_buffer(&mut self) {
        for _i in 1..GAME_BUFFER_SIZE {
            self.add_game_to_buffer();
        }
    }

    fn add_game_to_buffer (&mut self) {
        let sample: Vec<&String> = self.all_words.choose_multiple (
            &mut self.rng, 
            NUMBER_OF_WORDS_IN_GAME
        ).collect();

        let mut game_words: Vec<Word> = vec![];

        for i in 1..NUMBER_OF_WORDS_IN_GAME {
            // push each selected word, with a default type of WordType::Normal
            game_words.push(
                Word { text: sample.get(i).unwrap().to_string(), word_type: WordType::Normal }
            );
        }

        // now randomly assign the chosen words to blue, red, blackblue, or blackred
        let word_indices: Vec<usize> = (0..24).collect();
        let rand_word_type_indices: Vec<usize> = word_indices.choose_multiple(
            &mut self.rng, 
            20
        ).cloned().collect();

        for (index, value) in rand_word_type_indices.into_iter().enumerate() {
            let word: &mut Word = game_words.get_mut(value).unwrap();
            // Rust doesn't allow dynamic arm expressions as of now. Unfortunately, can't use the constants defined in mod.rs.
            // TODO: refactor when possible.
            match index + 1 {
                1..=8 => word.word_type = WordType::Blue,
                9..=18 => word.word_type = WordType::Red,
                19..=19 => word.word_type = WordType::BlackBlue,
                20..=20 => word.word_type = WordType::Blue,
                _ => ()
            }
        }
        
        // add the newly created game to the buffer
        self.game_buffer.push_back(Game { board: Board { words: game_words } });
    }

    fn get_all_words() -> Result<Vec<String>, CreatorError> {
        // collect all words into map to prevent accepting duplicates
        let mut word_map: HashSet<String> = HashSet::new();

        for file in WORD_FILES {
            let file = File::open(file);
            match file {
                Ok(file_contents) => {
                    // attempt to add the contents of the file to the map of words
                    let reader = BufReader::new(file_contents);
                    for line in reader.lines() {
                        if let Ok(word) = line {
                            word_map.insert(word);
                        }
                    }
                },
                Err(error) => {
                    // silently ignore file opening failures for now
                    println!("{}", error);
                }
            }
        }

        if word_map.len() < MINIMUM_WORDBANK_SIZE {
            Err(CreatorError::NotEnoughWords)
        } else {
            // collect all words into the list
            let mut all_words_list: Vec<String> = vec![];
            for word in word_map.into_iter() {
                all_words_list.push(word);
            }
            Ok(all_words_list)
        }
    }
}
