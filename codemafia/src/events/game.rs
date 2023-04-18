/* Defines the content of a game event.  */

use serde::Serialize;

use crate::{wordbank::{WordType, Word}, messages::game::Team, player::role::CodeMafiaRoleTitle};

#[derive(Debug, Clone, Serialize)]
pub enum GameEvents {
    InSufficientPlayers,
    Board(OpaqueBoard),
    RoleUpdated(CodeMafiaRoleTitle),
    WordClicked(Word),
    WordSuggested(String /* The suggestor player name */, String /* The word that was suggested */),
    Turn(TeamTurn),
    GameEnded(GameOutcome)
}

#[derive(Debug, Clone, Serialize)]
pub struct OpaqueBoard {
    pub words: Vec<OpaqueWord>
}

#[derive(Debug, Clone, Serialize)]
pub struct OpaqueWord {
   pub text: String,
   pub color: Option<WordType>
}

#[derive(Debug, Clone, Serialize)]
pub struct TeamTurn {
    pub team: Team,
    pub coordinator: String /* The PlayerId of the coordinator. */
}

#[derive(Debug, Clone, Serialize)]
pub struct GameOutcome {
    pub winner: Team,
    pub condition: WinCondition
}

#[derive(Debug, Clone, Serialize)]
pub enum WinCondition {
    BlackWordSelected,
    WordsCompleted,
    UndercoverOperativeGuessed
}