use crate::{messages::game::Team, events::game::TeamTurn};
use itertools::interleave;
use crate::{events::{game::GameEvents, Event, EventContent, SEND_ERROR_MSG}, player::role::{CodeMafiaRole, CodeMafiaRoleTitle}};
use crate::events::Recipient;

use super::GameServer;

/* Game-turn specific event handling. */
impl GameServer {
    pub async fn set_coordinators(&mut self) {
        /* Determine the coordinator order by interweaving the blue and red ally players. */
        let mut blue_ally_player_ids: Vec<(Team, String)> = vec![];
        let mut red_ally_player_ids: Vec<(Team, String)> = vec![];
        
        self.players.iter().for_each(|player| {
            if let Some(player_role) = &player.role {
                if player_role.role_title == Some(crate::player::role::CodeMafiaRoleTitle::Ally) {
                    match player_role.team {
                        Team::Blue => blue_ally_player_ids.push((Team::Blue, player.player_id.to_string())),
                        Team::Red => red_ally_player_ids.push((Team::Red, player.player_id.to_string()))
                    }
                }
            }
        });
        let coordinators = interleave(red_ally_player_ids, blue_ally_player_ids).collect::<Vec<(Team, String)>>();
        self.turn_state = Some(
            TurnStateMachine::new(coordinators)
        )
    }

    pub async fn advance_turn(&mut self) {
        if let Some(turn_state) = &mut self.turn_state {
            if let Some(next_turn) = turn_state.next(){
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
}

pub struct TurnStateMachine {
    coordinators: Vec<(Team, String)>,
    index: usize
}

impl Iterator for TurnStateMachine {
    type Item = (Team, String);

    fn next(&mut self) -> Option<Self::Item> {        
        self.index += 1;
        Some(self.coordinators[(self.index - 1) % self.coordinators.len()].clone())
    }
}

impl TurnStateMachine {
    fn new(coordinators: Vec<(Team, String)>) -> Self {
        TurnStateMachine {
            coordinators,
            index: 0
        }
    }
}