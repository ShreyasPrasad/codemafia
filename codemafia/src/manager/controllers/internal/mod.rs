/* This controller is responisble for handling internal messages to maintain player connections. */
use std::sync::Arc;

use dashmap::DashMap;
use shared::player::{PlayerId, PlayerMetadata};
use tokio::sync::mpsc::Sender;

use crate::{
    manager::dispatchers::{default::DefaultEventDispatcher, EventDispatcher},
    misc::{
        events::{Event, SEND_ERROR_MSG},
        internal::InternalMessage,
        player::{ActivePlayer, PlayerStatus},
    },
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
    dispatcher: DefaultEventDispatcher,
    /* The event sender, obtained from the dispatcher. */
    event_sender: Sender<Event>,
    /* The first player to join the room is assigned owner and is responsible for starting the game. */
    owner: Option<PlayerId>,
}

impl InternalController {
    pub fn new(
        players: Arc<DashMap<PlayerId, ActivePlayer>>,
        dispatcher: DefaultEventDispatcher,
    ) -> Self {
        let event_sender = dispatcher.get_event_sender();
        InternalController {
            players,
            owner: None,
            dispatcher,
            event_sender,
        }
    }

    pub async fn handle_message(&mut self, message: InternalMessage) {
        match message {
            InternalMessage::NewPlayer(player_name, event_sender, player_meta_receiver) => {
                let player_meta: PlayerMetadata = self.create_player(player_name, event_sender);
                /* Set the player cookie. */
                self.set_player_cookie(player_meta.player_id).await;
                player_meta_receiver
                    .send(Some(player_meta))
                    .expect(SEND_ERROR_MSG);
            }
            InternalMessage::SessionConnection(player_id, player_meta_receiver) => {
                match self.players.get(&player_id) {
                    Some(player) => {
                        player_meta_receiver
                            .send(Some(player.meta.clone()))
                            .expect(SEND_ERROR_MSG);
                    }
                    None => {
                        player_meta_receiver.send(None).expect(SEND_ERROR_MSG);
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
                /* Mark the player as connected. */
                self.set_player_connection_status(player_id, PlayerStatus::Connected)
                    .await;
            }
            InternalMessage::PlayerDisconnected(player_id) => {
                self.set_player_connection_status(player_id, PlayerStatus::Disconnected)
                    .await;
            }
        }
        /* Update the players after all the actions above. */
        dispatch_room_state_update(&self.event_sender, self.players.clone()).await;
    }
}
