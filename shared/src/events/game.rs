/* Defines the content of a game event.  */

use crate::{
    elements::WordType, messages::game::Team, misc::sequenced::Sequenced,
    player::role::CodeMafiaRoleTitle,
};
use serde::Serialize;

use super::EventContent;

#[derive(Debug, Clone, Serialize)]
pub enum GameEvents {
    InSufficientPlayers,
    Board(OpaqueBoard),
    RoleUpdated(CodeMafiaRoleTitle),
    WordHint(
        Team,   /* The team that is giving a word hint */
        String, /* The word hint */
    ),
    WordClicked(
        u8,
        /* The index of the word that was clicked */
        WordType, /* The transparent word type */
    ),
    WordSuggested(
        String, /* The suggestor player name */
        u8,     /* The word that was suggested */
    ),
    Turn(TeamTurn),
    GameEnded(GameOutcome),
    /* Sent to a player reconnecting to the game, allowing them to populate the current game state. */
    GameState(Vec<Sequenced<EventContent>>, CurrentState),
}

#[derive(Debug, Clone, Serialize)]
pub struct OpaqueBoard {
    pub words: Vec<OpaqueWord>,
}

#[derive(Debug, Clone, Serialize)]
pub struct OpaqueWord {
    pub text: String,
    pub color: Option<WordType>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TeamTurn {
    pub team: Team,
    pub coordinator: String, /* The PlayerId of the coordinator. */
}

#[derive(Debug, Clone, Serialize)]
pub struct GameOutcome {
    pub winner: Team,
    pub condition: WinCondition,
}

#[derive(Debug, Clone, Serialize)]
pub enum WinCondition {
    BlackWordSelected,
    WordsCompleted,
    UndercoverOperativeGuessed,
}

#[derive(Debug, Clone, Serialize)]
pub struct CurrentState {
    turn: TeamTurn,
    board: OpaqueBoard,
}

// Create a convenience aliasing type
pub type RoomCode = String;
