/* Defines the content of a room event.  */

use crate::messages::game::Team;

#[derive(Clone)]
pub enum RoomEvents {
    PlayerJoinedTeam(PlayerOnTeam),
    RoomState(RoomState),
    GameStarted
}

#[derive(Clone)]
pub struct PlayerOnTeam {
    pub name: String,
    pub team: Team
}

#[derive(Clone)]
pub struct RoomState {
    pub players: Vec<PlayerOnTeam>
}
