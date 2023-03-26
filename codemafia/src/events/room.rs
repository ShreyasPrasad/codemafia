/* Defines the content of a room event.  */

use crate::messages::game::Team;

pub enum RoomEvents {
    PlayerJoinedTeamEvent(PlayerJoinedTeamEvent),
    RoomState(RoomState)
}

pub struct PlayerJoinedTeamEvent {
    name: String,
    team: Team
}

pub struct RoomState {
    
}


