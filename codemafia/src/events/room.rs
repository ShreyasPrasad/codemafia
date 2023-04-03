/* Defines the content of a room event.  */

use crate::messages::game::Team;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub enum RoomEvents {
    PlayerJoinedTeam(PlayerOnTeam),
    RoomState(RoomState),
    GameStarted
}

#[derive(Debug, Clone, Serialize)]
pub struct PlayerOnTeam {
    pub name: String,
    pub team: Team
}

#[derive(Debug, Clone, Serialize)]
pub struct You {
    player_on_team: PlayerOnTeam,
    id: String /* player_id as str */
}

#[derive(Debug, Clone, Serialize)]
pub struct RoomState {
    /* The active players in the room, including you. */
    pub players: Vec<PlayerOnTeam>,
}
