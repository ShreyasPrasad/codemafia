/* Defines the content of a room event.  */

use crate::messages::game::Team;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum RoomEvents {
    RoomState(RoomState),
    GameStarted,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlayerOnTeam {
    pub name: String,
    pub id: String,
    pub team: Team,
    pub is_spymaster: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct You {
    pub name: Option<String>, /* player_name */
    pub id: String,           /* player_id as str */
}

#[derive(Debug, Clone, Serialize)]
pub struct RoomState {
    /* The active players in the room, including you. */
    pub players: Vec<PlayerOnTeam>,
}
