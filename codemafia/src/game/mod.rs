/* 
    Game

    This module contains the logic for game completion itself, using the actions and
    events defined in mod actions and mod events respectively.
*/

use tokio::sync::mpsc::{Receiver};
use tokio::sync::oneshot;

use crate::manager::room::RoomToGameBridge;
use crate::wordbank::Game;

pub struct GameServer {
    /* The owned, generated game. */
    game: Game,
    bridge: RoomToGameBridge
}

impl GameServer {

    pub fn new(game: Game, bridge: RoomToGameBridge) -> Self {
        GameServer { game, bridge }
    }

    pub async fn start_game_loop(&mut self) {
        while let Some(cmd) = self.bridge.game_channel_rx.recv().await {
            
        }
    }
}