/* 
    Server
 
    Contains the Tokio task that runs the primary game loop.
*/

use tokio::sync::mpsc::{Receiver};
use tokio::sync::oneshot;

use crate::wordbank::Game;

use super::Message;
use super::event::Event;
use tokio::sync::mpsc;

pub struct GameServer {
    /* The owned, generated game. */
    game: Game 
}

impl GameServer {
    pub async fn game_loop(&mut self, mut rx: Receiver<Message>) {
        while let Some(cmd) = rx.recv().await {
            
        }
    }
}
