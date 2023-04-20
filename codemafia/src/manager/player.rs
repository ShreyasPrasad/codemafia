use super::room::RoomController;

use crate::events::player::PlayerEvents;
use crate::events::room::{RoomState, PlayerOnTeam, RoomEvents};
use crate::events::{Event, Recipient, EventContent, SEND_ERROR_MSG};
use crate::messages::game::Team;
use crate::player::role::{CodeMafiaRole, CodeMafiaRoleTitle};
use crate::player::{PlayerId, PlayerError};
use crate::player::Player;
use tokio::sync::mpsc::Sender;

/* Player-specific room controller methods. */
impl RoomController {
    pub fn create_player(&mut self, player_name: String, event_sender: Sender<EventContent>) -> Player {
        let new_player = Player::new(player_name, event_sender);
        /* Assign the owner. */
        if self.players.is_empty() {
            self.owner = Some(new_player.player_id);
        }
        new_player
    }

    pub fn update_player(&mut self, player_id: PlayerId, event_sender: Sender<EventContent>) -> Result<(), PlayerError>{
        match self.players.get_mut(&player_id) {
            Some(mut player) => {
                player.channel.event_sender = event_sender;
                Ok(())
            },
            None => {
                Err(PlayerError::DoesNotExist)
            }
        }
    }

    pub async fn dispatch_room_state_update(&self) {
        self.event_dispatcher.send(
            Event { 
                recipient: Recipient::All,
                content: EventContent::Room(RoomEvents::RoomState(self.get_room_state()))
            }
        ).await.expect(SEND_ERROR_MSG);
    }

    pub fn update_player_team(&mut self, player_id: PlayerId, team: Team, is_spymaster: bool) -> Result<(), PlayerError> {
        let role_title: Option<CodeMafiaRoleTitle> = 
            if is_spymaster { Some(CodeMafiaRoleTitle::SpyMaster) } else { Some(CodeMafiaRoleTitle::Ally) };

        match self.players.get_mut(&player_id) {
            Some(mut p_ref) => {
                p_ref.role = Some(CodeMafiaRole { role_title, team });
                Ok(())
            },
            None => Err(PlayerError::DoesNotExist)
        }
    }

    pub async fn set_player_cookie(&self, player_id: PlayerId) {
        self.event_dispatcher.send(
            Event { 
                recipient: Recipient::SinglePlayerList(vec![player_id]),
                content: EventContent::Player(PlayerEvents::SetPlayerIdCookie(player_id.to_string()))
            }
        ).await.expect(SEND_ERROR_MSG);
    }

    /* Construct the room state from the list of active players */
    fn get_room_state(&self) -> RoomState {
        let mut active_players: Vec<PlayerOnTeam> = vec![];
        self.players.iter().for_each(|p_ref| {
            if let Some(player_name) = &p_ref.name {
                if let Some(player_role) = &p_ref.role {
                    active_players.push(PlayerOnTeam{
                        name: player_name.to_string(), 
                        id: p_ref.player_id.to_string(), 
                        team: player_role.team.clone(),
                        is_spymaster: player_role.role_title == Some(CodeMafiaRoleTitle::SpyMaster)
                    });
                }
            }
        });
        RoomState {
            players: active_players
        }
    }
}