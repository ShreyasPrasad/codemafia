use std::vec;

use crate::{events::{game::{OpaqueBoard, OpaqueWord, GameEvents}, Event, EventContent, SEND_ERROR_MSG}, player::role::{CodeMafiaRole, CodeMafiaRoleTitle}, messages::game::Team};
use crate::events::Recipient;

use super::GameServer;

/* Game board specific event handling. */
impl GameServer {
    pub async fn send_initial_game_state(&self) {
        /* Construct the default board. */
        let hidden_board: OpaqueBoard = OpaqueBoard {
            words: self.game.board.words.iter().map(|word| {
                OpaqueWord {
                    text: word.text.to_string(),
                    color: None
                }
            }).collect::<Vec<OpaqueWord>>()
        };
        /* Send the board to each player, but for special roles, reveal word colours. */
        self.send_initial_game_state_hidden(&hidden_board).await;

        let visible_board: OpaqueBoard = OpaqueBoard {
            words: self.game.board.words.iter().map(|word| {
                OpaqueWord {
                    text: word.text.to_string(),
                    color: Some(word.word_type)
                }
            }).collect::<Vec<OpaqueWord>>()
        };
        
        self.send_initial_game_state_visible(&visible_board).await;
    }

    async fn send_initial_game_state_hidden(&self, board: &OpaqueBoard){
        /* Send to each team, their spymasters, and the undercover operatives. */
        self.bridge.room_channel_tx.send(
            Event {
                recipient: Recipient::SingleRoleList(
                    vec![CodeMafiaRole{team: Team::Blue, role_title: Some(CodeMafiaRoleTitle::Ally)},
                         CodeMafiaRole{team: Team::Red, role_title: Some(CodeMafiaRoleTitle::Ally)}]),
                content: EventContent::Game(GameEvents::Board(board.clone()))
            }
        ).await.expect(SEND_ERROR_MSG);
    }

    async fn send_initial_game_state_visible(&self, board: &OpaqueBoard){
        /* Send to each team, their spymasters, and the undercover operatives. */
        self.bridge.room_channel_tx.send(
            Event {
                recipient: Recipient::SingleRoleList(
                    vec![CodeMafiaRole{team: Team::Blue, role_title: Some(CodeMafiaRoleTitle::Undercover)},
                         CodeMafiaRole{team: Team::Red, role_title: Some(CodeMafiaRoleTitle::Undercover)},
                         CodeMafiaRole{team: Team::Blue, role_title: Some(CodeMafiaRoleTitle::SpyMaster)},
                         CodeMafiaRole{team: Team::Red, role_title: Some(CodeMafiaRoleTitle::SpyMaster)}]),
                content: EventContent::Game(GameEvents::Board(board.clone()))
            }
        ).await.expect(SEND_ERROR_MSG);
    }
}