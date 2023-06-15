/* This controller is responisble for handling internal messages to maintain player connections. */
use std::sync::Arc;

use dashmap::DashMap;
use shared::{events::room::You, player::PlayerId};
use tokio::sync::mpsc::Sender;

use crate::misc::{
    events::{Event, SEND_ERROR_MSG},
    internal::InternalMessage,
    player::ActivePlayer,
};

use super::util::dispatch_room_state_update;

mod player;

/* These are aliases for the room listener and receiver; this is the channel that all players send their actions to.  */
pub type InternalSender = Sender<InternalMessage>;

/* A message buffer size of 16 should be more than sufficient for occasional player update messages. */
pub const INTERNAL_MSPC_BUFFER_SIZE: usize = 16;

pub struct InternalController {
    /* A reference to the list of active players. */
    players: Arc<DashMap<PlayerId, ActivePlayer>>,
    /* The event dispatcher, responsible for forwarding events to players. */
    event_sender: Sender<Event>,
    /* The first player to join the room is assigned owner and is responsible for starting the game. */
    owner: Option<PlayerId>,
}

impl InternalController {
    pub fn new(players: Arc<DashMap<PlayerId, ActivePlayer>>, event_sender: Sender<Event>) -> Self {
        InternalController {
            players,
            owner: None,
            event_sender,
        }
    }

    pub async fn handle_message(&mut self, message: InternalMessage) {
        match message {
            InternalMessage::NewPlayer(player_name, event_sender) => {
                let player: ActivePlayer = self.create_player(player_name, event_sender);
                /* Set the player cookie. */
                self.set_player_cookie(player.meta.player_id).await;
            }
            InternalMessage::SessionConnection(player_id, you_receiver) => {
                match self.players.get(&player_id) {
                    Some(player) => {
                        you_receiver
                            .send(Some(You {
                                name: player.meta.name.clone(),
                                id: player.meta.player_id.to_string(),
                            }))
                            .expect(SEND_ERROR_MSG);
                    }
                    None => {
                        you_receiver.send(None).expect(SEND_ERROR_MSG);
                    }
                }
                /* Set the player cookie. */
                self.set_player_cookie(player_id).await;
            }
            InternalMessage::UpdatePlayer(player_id, event_sender) => {
                if let Err(err) = self.update_player_connection(player_id, event_sender) {
                    println!("Error updating player: {}", err);
                }
                /* Set the player cookie. */
                self.set_player_cookie(player_id).await;
            }
        }
        /* Update the players after all the actions above. */
        dispatch_room_state_update(&self.event_sender, self.players.clone());
    }
}
