/* Manager

 This module contains the logic that handles game creation and routing. Newly created
 games are stored in memory, accessible to clients by a generated game code that
 the original game creator distributes.

 */

use std::collections::HashMap;
use tokio::sync::mpsc;
use super::server::Action;
use rand::{Rng, rngs::ThreadRng}; // 0.8

/* A message buffer size of 64 should be more than sufficient as messages are handled as soon as they 
   appear from, from at most 10-12 players. */
const MSPC_BUFFER_SIZE: usize = 64;

/* Constants used to define the RNG that generates game codes that are distributed to player. */
const GAME_CODE_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const GAME_CODE_LEN: usize = 4;

pub struct GameManager {
    games: HashMap<String, GameChannel>
}

type GameChannel = mpsc::Sender<Action>;

impl GameManager {
    /* Invoked when a game creation request is made. */
    pub fn create_game(&mut self){
        let (tx, mut rx) = mpsc::channel::<Action>(MSPC_BUFFER_SIZE);
        /* Add the game to the manager so at least one reference to the Sender exists (and thus receiver is not dropped). */
        self.games.insert(self.get_game_code(), tx);
        tokio::spawn(async move {
            
        });
    }

    pub fn add_player(self){
        
    }

    /* Invoked by a game when it is completed and should be cleaned up from within the manager. */
    pub fn remove_game(&mut self, game_code: String){
        /* Removing the game from the map kills the Tokio game server task since mspc::Sender ref count reaches 0. */
        self.games.remove(&game_code);
    }

    fn get_game_code(&self) -> String {
        let mut rng: ThreadRng = rand::thread_rng();

        let game_code: String = (0..GAME_CODE_LEN)
            .map(|_| {
                let idx: usize = rng.gen_range(0..GAME_CODE_CHARSET.len());
                GAME_CODE_CHARSET[idx] as char
            })
            .collect();

        game_code
    }
}