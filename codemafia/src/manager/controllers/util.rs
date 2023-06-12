use std::sync::Arc;

use dashmap::DashMap;
use shared::{events::{EventContent, room::{RoomEvents, RoomState, PlayerOnTeam}}, player::{role::CodeMafiaRoleTitle, PlayerId}};
use tokio::sync::mpsc::Sender;

use crate::{misc::{events::{Event, Recipient, SEND_ERROR_MSG}, player::ActivePlayer}};

pub async fn dispatch_room_state_update(event_sender: &Sender<Event>, players: Arc<DashMap<PlayerId, ActivePlayer>>) {
    event_sender.send(
        Event { 
            recipient: Recipient::All,
            content: EventContent::Room(RoomEvents::RoomState(get_room_state(players)))
        }
    ).await.expect(SEND_ERROR_MSG);
}

/* Construct the room state from the list of active players */
fn get_room_state(players: Arc<DashMap<PlayerId, ActivePlayer>>) -> RoomState {
    let mut active_players: Vec<PlayerOnTeam> = vec![];
    players.iter().for_each(|p_ref| {
        if let Some(player_name) = &p_ref.meta.name {
            if let Some(player_role) = &p_ref.meta.role {
                active_players.push(PlayerOnTeam{
                    name: player_name.to_string(), 
                    id: p_ref.meta.player_id.to_string(), 
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