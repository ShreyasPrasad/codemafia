/* Game
 
 Contains the Tokio task that runs the primary game loop.

 */

use tokio::sync::mpsc::{Receiver};
use tokio::sync::oneshot;

use crate::wordbank::Game;

use super::Action;
use super::event::Event;
use tokio::sync::mpsc;

pub struct GameServer {
    /* The owned, generated game. */
    game: Game 
}

impl GameServer {
    pub async fn game_loop(&mut self, mut rx: Receiver<Action>) {
        while let Some(cmd) = rx.recv().await {
            
        }
    }
}
