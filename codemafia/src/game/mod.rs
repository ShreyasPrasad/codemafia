/* Server

 This module contains the logic for game completion itself, along with the necessary structures
 that encapsulate game actions and outcomes.

 */

pub mod server;
pub mod event;

#[derive(Debug)]
pub enum Action {
    Chat(ChatAction),
    Game(GameAction)
}

#[derive(Debug)]
pub struct ChatAction {
    text: String,
    recipient: u32
}

#[derive(Debug)]
pub struct GameAction {
    action_type: GameActionType,
}

#[derive(Debug)]
pub enum GameActionType {
    WordSuggested(String /* The word that was suggested */),
    WordClicked(u8 /* The index of the word that was clicked */),
    SpyMasterVoteInitiated,
    EndTurn,
    TeamSelected(Team)
}

#[derive(Debug)]
pub enum Team {
    Blue,
    Red
}