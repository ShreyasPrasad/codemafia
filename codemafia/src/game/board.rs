use std::vec;

use shared::{
    elements::WordType,
    events::{
        game::{GameEvents, OpaqueBoard, OpaqueWord},
        EventContent,
    },
    messages::game::Team,
    player::{
        role::{CodeMafiaRole, CodeMafiaRoleTitle},
        PlayerId,
    },
};

use crate::misc::events::{Event, Recipient, SEND_ERROR_MSG};

use super::GameServer;

/* Game board specific event handling. */
impl GameServer {
    pub async fn send_initial_game_state(&self) {
        /* Construct the default, hidden board. */
        let hidden_board: OpaqueBoard = self.construct_hidden_board();
        /* Send the hidden board to the corresponding players. */
        self.send_initial_game_state_hidden(&hidden_board).await;
        /* Construct the fully visible board. */
        let visible_board: OpaqueBoard = self.construct_visible_board();
        /* Send the visible board to the corresponding players. */
        self.send_initial_game_state_visible(&visible_board).await;
    }

    async fn send_initial_game_state_hidden(&self, board: &OpaqueBoard) {
        /* Send to each team, their spymasters, and the undercover operatives. */
        self.bridge
            .room_channel_tx
            .send(Event {
                recipient: Recipient::SingleRoleList(vec![
                    CodeMafiaRole {
                        team: Team::Blue,
                        role_title: Some(CodeMafiaRoleTitle::Ally),
                    },
                    CodeMafiaRole {
                        team: Team::Red,
                        role_title: Some(CodeMafiaRoleTitle::Ally),
                    },
                ]),
                content: EventContent::Game(GameEvents::Board(board.clone())),
            })
            .await
            .expect(SEND_ERROR_MSG);
    }

    async fn send_initial_game_state_visible(&self, board: &OpaqueBoard) {
        /* Send to each team, their spymasters, and the undercover operatives. */
        self.bridge
            .room_channel_tx
            .send(Event {
                recipient: Recipient::SingleRoleList(vec![
                    CodeMafiaRole {
                        team: Team::Blue,
                        role_title: Some(CodeMafiaRoleTitle::Undercover),
                    },
                    CodeMafiaRole {
                        team: Team::Red,
                        role_title: Some(CodeMafiaRoleTitle::Undercover),
                    },
                    CodeMafiaRole {
                        team: Team::Blue,
                        role_title: Some(CodeMafiaRoleTitle::SpyMaster),
                    },
                    CodeMafiaRole {
                        team: Team::Red,
                        role_title: Some(CodeMafiaRoleTitle::SpyMaster),
                    },
                ]),
                content: EventContent::Game(GameEvents::Board(board.clone())),
            })
            .await
            .expect(SEND_ERROR_MSG);
    }

    /* Function that returns the board visible to the player with the given player Id. */
    fn get_current_board_state_for_player(&self, player_id: PlayerId) -> OpaqueBoard {
        /* Get the player's role and construct the board accordingly. */
        let player = &self
            .players
            .get(&player_id)
            .expect("Unexpected invalid player Id.");
        let player_role = player.meta.role.as_ref().unwrap();
        match player_role.role_title.unwrap() {
            CodeMafiaRoleTitle::SpyMaster => self.construct_visible_board(),
            CodeMafiaRoleTitle::Undercover => self.construct_visible_board(),
            CodeMafiaRoleTitle::Ally => self.construct_hidden_board(),
        }
    }

    /* Constructs an owned visible board from the generated board words. */
    fn construct_visible_board(&self) -> OpaqueBoard {
        OpaqueBoard {
            words: self
                .game
                .board
                .words
                .iter()
                .map(|word| OpaqueWord {
                    text: word.text.to_string(),
                    color: Some(word.word_type),
                })
                .collect::<Vec<OpaqueWord>>(),
        }
    }

    /* Constructs an owned hidden board from the generated board words. */
    fn construct_hidden_board(&self) -> OpaqueBoard {
        OpaqueBoard {
            words: self
                .game
                .board
                .words
                .iter()
                .map(|word| OpaqueWord {
                    text: word.text.to_string(),
                    color: if word.clicked {
                        Some(word.word_type)
                    } else {
                        Some(WordType::Normal)
                    },
                })
                .collect::<Vec<OpaqueWord>>(),
        }
    }
}
