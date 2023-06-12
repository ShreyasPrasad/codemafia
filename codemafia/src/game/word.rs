use shared::{player::{PlayerId, role::CodeMafiaRoleTitle}, events::{EventContent, game::{GameEvents, GameOutcome, WinCondition}}, messages::game::Team, elements::{WordType, NUM_BLUE_WORDS, NUM_RED_WORDS}};
use crate::misc::events::{Event, Recipient, SEND_ERROR_MSG};
use super::GameServer;

#[derive(Default)]
pub struct GameState {
    pub num_blue_words_clicked: usize,
    pub num_red_words_clicked: usize
}

/*  Event handling for messages involving word clicks, suggestions, and hints. */
impl GameServer {
    pub async fn handle_word_click(&mut self, player_id: PlayerId, word_index: u8){
        // Make sure we have a valid word_index
        if word_index >= 25 {
            println!("Received a word click message with an invalid word_index: {}", word_index);
            return;
        }

        // Make sure it's the current coordinator making the click.
        let (team, coord) = self.turn_state.get_current_turn();

        if coord != player_id.to_string() {
            println!("Received a word click message for an incorrect player with ID: {}", player_id);
            return;
        }

        // Make sure the word hasn't been clicked already.
        let word_type: WordType;
        if let Some(word) = self.game.board.words.get_mut(word_index as usize){
            if word.clicked {
                println!("Received a word click message from player with ID: {} in which the word has already been clicked: {}", player_id, word.text);
                return;
            }
            // Set the word as clicked and record its word type.
            word.clicked = true;
            word_type = word.word_type;
        } else {
            println!("Received a word click message from player with ID {} in which the word index is invalid: {}", player_id, word_index);
            return;
        }

        // Send the click event to all the players.
        self.bridge.room_channel_tx.send(
            Event {
                recipient: Recipient::All,
                content: EventContent::Game(GameEvents::WordClicked(word_index, word_type))
            }
        ).await.expect(SEND_ERROR_MSG);

        // Check if a team has won as a result.
        self.check_win_condition(team, word_type).await;
    }

    pub async fn handle_word_suggested(&self, player_id: PlayerId, word_index: u8){
        // Make sure the correct team is sending a word
        let (team, _) = self.turn_state.get_current_turn();

        let player = &self.players.get(&player_id).unwrap();
        match &player.meta.role {
            Some(player_role_op) => {
                if player_role_op.team != team {
                    println!("Received a word hint message for an incorrect player with ID: {}", player_id);
                }
                /* Send the word suggestion to all players. */
                self.bridge.room_channel_tx.send(
                    Event {
                        recipient: Recipient::All,
                        content: EventContent::Game(GameEvents::WordSuggested(player.meta.name.clone().unwrap(), word_index))
                    }
                ).await.expect(SEND_ERROR_MSG);
            },
            None => {
                println!("Unexpected error in handle_word_hint: player with ID {} does not have a role.", player_id);
            }
        }
    }

    pub async fn handle_word_hint(&self, player_id: PlayerId, hint: String){
        // Make sure the correct spymaster is sending a hint.
        let (team, _) = self.turn_state.get_current_turn();

        let player_role_op = &self.players.get(&player_id).unwrap().meta.role;

        match player_role_op {
            Some(player_role) => {
                if player_role.team != team || player_role.role_title != Some(CodeMafiaRoleTitle::SpyMaster){
                    println!("Received a word hint message for an incorrect player with ID: {}", player_id);
                }
                /* Send the word hint to all players. */
                self.bridge.room_channel_tx.send(
                    Event {
                        recipient: Recipient::All,
                        content: EventContent::Game(GameEvents::WordHint(team, hint))
                    }
                ).await.expect(SEND_ERROR_MSG);
            },
            None => {
                println!("Unexpected error in handle_word_hint: player with ID {} does not have a role.", player_id);
            }
        }
    } 

    async fn check_win_condition(&mut self, team: Team, word_clicked_type: WordType){
        match word_clicked_type {
            WordType::Black => {
                let mut winner: Team = Team::Blue;
                if team == Team::Blue {
                    winner = Team::Red;
                }
                self.bridge.room_channel_tx.send(
                    Event {
                        recipient: Recipient::All,
                        content: EventContent::Game(GameEvents::GameEnded(GameOutcome{winner, condition: WinCondition::BlackWordSelected}))
                    }
                ).await.expect(SEND_ERROR_MSG);
            },
            WordType::Blue => {
                self.game_state.num_blue_words_clicked += 1;
                if self.game_state.num_blue_words_clicked == NUM_BLUE_WORDS {
                    self.send_game_outcome_words_completed(team).await;
                }
            },
            WordType::Red => {
                self.game_state.num_blue_words_clicked += 1;
                if self.game_state.num_blue_words_clicked == NUM_RED_WORDS {
                    self.send_game_outcome_words_completed(team).await;
                }
            },
            _ => ()
        } 
    }

    async fn send_game_outcome_words_completed(&self, team: Team){
        self.bridge.room_channel_tx.send(
            Event {
                recipient: Recipient::All,
                content: EventContent::Game(GameEvents::GameEnded(GameOutcome{winner: team, condition: WinCondition::WordsCompleted}))
            }
        ).await.expect(SEND_ERROR_MSG);
    }
}