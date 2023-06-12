use crate::misc::player::ActivePlayer;

use shared::events::EventContent;
use shared::events::player::PlayerEvents;
use shared::player::{PlayerId, PlayerError};
use tokio::sync::mpsc::Sender;
use crate::misc::events::{Event, Recipient, SEND_ERROR_MSG};

use super::InternalController;

/* Player-specific room controller methods. */
impl InternalController {
    pub fn create_player(&mut self, player_name: String, event_sender: Sender<EventContent>) -> ActivePlayer {
        let new_player = ActivePlayer::new(player_name, event_sender);
        /* Assign the owner. */
        if self.players.is_empty() {
            self.owner = Some(new_player.meta.player_id);
        }
        new_player
    }

    pub fn update_player_connection(&mut self, player_id: PlayerId, event_sender: Sender<EventContent>) -> Result<(), PlayerError>{
        match self.players.get_mut(&player_id) {
            Some(mut player) => {
                player.connection.event_sender = event_sender;
                Ok(())
            },
            None => {
                Err(PlayerError::DoesNotExist)
            }
        }
    }

    pub async fn set_player_cookie(&self, player_id: PlayerId) {
        self.event_sender.send(
            Event { 
                recipient: Recipient::SinglePlayerList(vec![player_id]),
                content: EventContent::Player(PlayerEvents::SetPlayerIdCookie(player_id.to_string()))
            }
        ).await.expect(SEND_ERROR_MSG);
    }
}