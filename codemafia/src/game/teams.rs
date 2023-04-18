/* Team-specific game action handlers. */

use rand::rngs::ThreadRng;
use rand::seq::IteratorRandom;
use rand::thread_rng;

use crate::events::Event;
use crate::events::EventContent;
use crate::events::Recipient;
use crate::events::SEND_ERROR_MSG;
use crate::events::game::GameEvents;
use crate::player::role::CodeMafiaRole;
use crate::player::role::CodeMafiaRoleTitle;
use crate::messages::game::Team;

use super::GameServer;

// Some useful constants.
const MIN_NUMBER_OF_PLAYERS_ON_TEAM : usize = 4;
const MIN_NUMBER_OF_SPYMASTERS: usize = 2;

impl GameServer {
    pub async fn complete_teams(&self){
        let mut num_blue = 0;
        let mut num_red = 0;
        let mut num_spymasters = 0;
        // Conduct some basic pre-game checks
        self.players.iter().for_each(|player| {
            if let Some(role) = &player.role {
                if role.team == Team::Blue {
                    num_blue = num_blue + 1;
                } else {
                    num_red = num_red + 1;
                }
                if let Some(role_title) = role.role_title {
                    if role_title == CodeMafiaRoleTitle::SpyMaster {
                        num_spymasters = num_spymasters + 1;
                    }
                }
            }
        });

        if num_blue < MIN_NUMBER_OF_PLAYERS_ON_TEAM 
            || num_red < MIN_NUMBER_OF_PLAYERS_ON_TEAM  
            || num_spymasters < MIN_NUMBER_OF_SPYMASTERS {
            self.bridge.room_channel_tx.send(
                Event {
                    recipient: Recipient::All,
                    content: EventContent::Game(GameEvents::InSufficientPlayers)
                }
            ).await.expect(SEND_ERROR_MSG);
        }

        // Assign the 2 undercover players randomly.
        self.assign_undercover_players();

        // Send the players their new roles.
        self.send_player_roles().await;
    }

    fn assign_undercover_players(&self) {
        let mut rng: ThreadRng = thread_rng();
        let mut red_player_assigned: bool = false;
        loop {
            let mut player = self.players.iter_mut().choose(&mut rng).unwrap();
            if let Some(role) = &mut player.role {
                if let Some(role_title) = role.role_title {
                    if role_title != CodeMafiaRoleTitle::SpyMaster {
                        if role.team == Team::Red && !red_player_assigned {
                            // Assign red undercover player.
                            role.role_title = Some(CodeMafiaRoleTitle::Undercover);
                            red_player_assigned = true;
                        } else if role.team == Team::Blue && red_player_assigned {
                            // Assign blue undercover player.
                            role.role_title = Some(CodeMafiaRoleTitle::Undercover);
                            break;
                        }
                    }  
                }
            }
        }
    }

    async fn send_player_roles(&self) {
        /* Send allies on both teams their ally role. */
        self.bridge.room_channel_tx.send(
            Event {
                recipient: Recipient::SingleRoleList(
                    vec![CodeMafiaRole { team: Team::Blue, role_title: Some(CodeMafiaRoleTitle::Ally)}]),
                content: EventContent::Game(GameEvents::RoleUpdated(CodeMafiaRoleTitle::Ally))
            }
        ).await.expect(SEND_ERROR_MSG);

        /* Send undercover operatives their role. */
        self.bridge.room_channel_tx.send(
            Event {
                recipient: Recipient::SingleRoleList(
                    vec![CodeMafiaRole { team: Team::Blue, role_title: Some(CodeMafiaRoleTitle::Undercover)},
                         CodeMafiaRole { team: Team::Red, role_title: Some(CodeMafiaRoleTitle::Undercover)}]),
                content: EventContent::Game(GameEvents::RoleUpdated(CodeMafiaRoleTitle::Undercover))
            }
        ).await.expect(SEND_ERROR_MSG);
    }
}