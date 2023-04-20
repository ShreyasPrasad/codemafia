/* 
    Game

    This module contains the logic for game completion itself, using the actions and
    events defined in mod actions and mod events respectively.
*/

use std::str::FromStr;
use std::sync::Arc;

use dashmap::DashMap;
use futures::Future;

use crate::manager::bridge::RoomToGameBridge;
use crate::messages::game::GameMessageAction;
use crate::player::{PlayerId, Player};
use crate::wordbank::Game;

use self::turn::TurnStateMachine;
use self::word::GameState;

mod teams;
mod turn;
mod board;
mod word;
mod cache;

pub struct GameServer {
    /* The owned, generated game. */
    game: Game,
    bridge: RoomToGameBridge,
    players: Arc<DashMap<PlayerId, Player>>,
    turn_state: TurnStateMachine, /* The coordinator ordering, using player ID strings. */
    game_state: GameState
}

/* Contains message handling corresponding to game actions. */
impl GameServer {

    pub fn new(game: Game, bridge: RoomToGameBridge, players: Arc<DashMap<PlayerId, Player>>) -> Self {
        GameServer { 
            game, 
            bridge, 
            players: players.clone(), 
            turn_state: GameServer::get_turn_state_machine(players.clone()),
            game_state: GameState::default()
        }
    }

    /* Called once at the start of the game. */
    pub async fn init_game(&mut self) {
        self.complete_teams().await;
        self.send_initial_game_state().await;
    }

    pub async fn start_game_loop(&mut self) {
        while let Some(cmd) = self.bridge.game_channel_rx.recv().await {
            match cmd.action {
                GameMessageAction::EndTurn => {
                    self.advance_turn().await;
                },
                GameMessageAction::WordClicked(player_id, index) => {
                    if let Ok(player_id) = uuid::Uuid::from_str(&player_id) {
                        self.handle_word_click(player_id, index).await;
                    }
                },
                GameMessageAction::WordSuggested(player_id, index) => {
                    let fut = |id| self.handle_word_suggested(id, index);
                    Self::proceed_with_valid_player_id(player_id, fut).await;
                },
                GameMessageAction::WordHint(player_id, hint) => {
                    let fut = |id| self.handle_word_hint(id, hint.clone());
                    Self::proceed_with_valid_player_id(player_id, fut).await;
                }
            }
        }
    }

    async fn proceed_with_valid_player_id<F, Fut>(player_id: String, f: F)
    where
        F: FnOnce(PlayerId) -> Fut, 
        Fut: Future,
    {
        if let Ok(player_id) = uuid::Uuid::from_str(&player_id) {
            f(player_id).await;
        }
    }
}