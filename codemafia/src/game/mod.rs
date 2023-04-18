/* 
    Game

    This module contains the logic for game completion itself, using the actions and
    events defined in mod actions and mod events respectively.
*/

use std::sync::Arc;

use dashmap::DashMap;

use crate::manager::bridge::RoomToGameBridge;
use crate::messages::game::GameMessageAction;
use crate::player::{PlayerId, Player};
use crate::wordbank::Game;

use self::turn::TurnStateMachine;

mod teams;
mod turn;
mod board;

pub struct GameServer {
    /* The owned, generated game. */
    game: Game,
    bridge: RoomToGameBridge,
    players: Arc<DashMap<PlayerId, Player>>,
    turn_state: Option<TurnStateMachine> /* The coordinator ordering, using player ID strings. */
}

/* Contains message handling corresponding to game actions. */
impl GameServer {

    pub fn new(game: Game, bridge: RoomToGameBridge, players: Arc<DashMap<PlayerId, Player>>) -> Self {
        GameServer { game, bridge, players, turn_state: None }
    }

    pub async fn start_game_loop(&mut self) {
        while let Some(cmd) = self.bridge.game_channel_rx.recv().await {
            match cmd.action {
                /* teams.rs */
                GameMessageAction::MakeTeams => {
                    self.complete_teams().await;
                    self.send_initial_game_state().await;
                    self.set_coordinators().await;
                    self.advance_turn().await;
                },
                _ => ()
            }
        }
    }
}