use std::sync::Arc;

use codemafia::{messages::game::Team, events::game::TeamTurn, player::{PlayerId, Player, role::CodeMafiaRoleTitle}};
use codemafia::{events::{game::GameEvents, Event, EventContent, SEND_ERROR_MSG}};
use codemafia::events::Recipient;
use dashmap::DashMap;
use itertools::interleave;

use super::GameServer;

/* Game-turn specific event handling. */
impl GameServer {
    pub fn get_turn_state_machine(players: Arc<DashMap<PlayerId, Player>>) -> TurnStateMachine {
        /* Determine the coordinator order by interweaving the blue and red ally players. */
        let mut blue_ally_player_ids: Vec<(Team, String)> = vec![];
        let mut red_ally_player_ids: Vec<(Team, String)> = vec![];
        
        players.iter().for_each(|player| {
            if let Some(player_role) = &player.role {
                /* Include both undercover operatives and allies. */
                if player_role.role_title != Some(CodeMafiaRoleTitle::SpyMaster) {
                    match player_role.team {
                        Team::Blue => blue_ally_player_ids.push((Team::Blue, player.player_id.to_string())),
                        Team::Red => red_ally_player_ids.push((Team::Red, player.player_id.to_string()))
                    }
                }
            }
        });
        
        let coordinators = interleave(red_ally_player_ids, blue_ally_player_ids).collect::<Vec<(Team, String)>>();
        TurnStateMachine::new(coordinators)
    }

    pub async fn advance_turn(&mut self) {
        if let Some(next_turn) = self.turn_state.next(){
            self.bridge.room_channel_tx.send(
                Event {
                    recipient: Recipient::All,
                    content: EventContent::Game(GameEvents::Turn(TeamTurn {
                        team: next_turn.0,
                        coordinator: next_turn.1
                    }))
                }
            ).await.expect(SEND_ERROR_MSG)
        }
    }
}

pub struct TurnStateMachine {
    coordinators: Vec<(Team, String)>,
    index: usize
}

impl Iterator for TurnStateMachine {
    type Item = (Team, String);

    fn next(&mut self) -> Option<Self::Item> {        
        let resp = Some(self.coordinators[self.index % self.coordinators.len()].clone());
        self.index += 1;
        resp
    }
}

impl TurnStateMachine {
    fn new(coordinators: Vec<(Team, String)>) -> Self {
        TurnStateMachine {
            coordinators,
            index: 0
        }
    }

    pub fn get_current_turn(&self) -> (Team, String) {
        let index: usize = self.index % self.coordinators.len();
        self.coordinators[index].clone()
    }
}