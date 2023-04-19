/* Defines the content of a game event.  */

use serde::Serialize;

use crate::{wordbank::WordType, messages::game::Team, player::role::CodeMafiaRoleTitle};

#[derive(Debug, Clone, Serialize)]
pub enum GameEvents {
    InSufficientPlayers,
    Board(OpaqueBoard),
    RoleUpdated(CodeMafiaRoleTitle),
    WordHint(Team /* The team that is giving a word hint */, String /* The word hint */),
    WordClicked(u8, /* The index of the word that was clicked */ WordType /* The transparent word type */),
    WordSuggested(String /* The suggestor player name */, u8 /* The word that was suggested */),
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