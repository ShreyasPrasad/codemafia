/* Server

 This module contains the logic for game completion itself, along with the necessary structures
 that encapsulate game actions and outcomes. All the clients in a game connect to the same
 gameserver at the same time for easy synchronization. Though, this has a high per-server overhead.
 It may change if/when the game scales.

 */

pub mod game_loop;
pub mod event;

pub enum Action {
    Chat(ChatAction),
    Game(GameAction)
}

pub struct ChatAction {
    text: String,
    recipient: u32
}

pub struct GameAction {
    action_type: GameActionType,
}

pub enum GameActionType {
    WordSuggested,
    WordClicked,
    SpyMasterVoteInitiated,
    EndTurn
}