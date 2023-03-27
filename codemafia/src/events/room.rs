/* Defines the content of a room event.  */

use crate::messages::game::Team;

pub enum RoomEvents {
    PlayerJoinedTeam(PlayerOnTeam),
    RoomState(RoomState),
    GameStarted
}

pub struct PlayerOnTeam {
    name: String,
    team: Team
}

pub struct RoomState {
    players: Vec<PlayerOnTeam>
}
