/* Defines the content of a game event.  */

use serde::Serialize;

use crate::{wordbank::WordType, messages::game::Team};

#[derive(Debug, Clone, Serialize)]
pub enum GameEvents {
    InSufficientPlayers,
    Board(OpaqueBoard),
    WordClicked(String /* The word that was clicked */),
    WordSuggested(String /* The suggestor player name */, String /* The word that was suggested */),
    Turn(TeamTurn),
    GameEnded(GameOutcome)
}

#[derive(Debug, Clone, Serialize)]
pub struct OpaqueBoard {
    words: Vec<OpaqueWord>
}

#[derive(Debug, Clone, Serialize)]
pub struct OpaqueWord {
    text: String,
    color: Option<WordType>
}

#[derive(Debug, Clone, Serialize)]
pub struct TeamTurn {
    team: Team,
    coordinator: String /* The PlayerId of the coordinator. */
}

#[derive(Debug, Clone, Serialize)]
pub struct GameOutcome {
    winner: Team,
    condition: WinCondition
}

#[derive(Debug, Clone, Serialize)]
pub enum WinCondition {
    BlackWordSelected,
    WordsCompleted,
    UndercoverOperativeGuessed
}