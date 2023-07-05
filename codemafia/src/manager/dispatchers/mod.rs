use std::sync::Arc;

use dashmap::DashMap;
use tokio::sync::mpsc::Sender;

use crate::misc::{
    events::{Event, Recipient, SEND_ERROR_MSG},
    player::ActivePlayer,
};
use shared::{events::EventContent, player::PlayerId};

use self::info::DispatcherInfo;

pub mod cache;
pub mod default;
pub mod info;

pub trait EventDispatcher {
    fn new(players: Arc<DashMap<PlayerId, ActivePlayer>>) -> Self;
    fn get_event_sender(&self) -> Sender<Event>;

    /* This method sends the event to the included recipient (1 or more players). */
    fn dispatch_event(
        players_clone: Arc<DashMap<PlayerId, ActivePlayer>>,
        info: impl DispatcherInfo,
    ) {
        let event_content = info.event_content();
        match info.recipient() {
            Recipient::All => {
                /* Todo: parallelize event sending by assigning each event send to a new Tokio task. */
                tokio::spawn(async move {
                    for p_ref in players_clone.iter() {
                        dispatch_event_to_player(p_ref.value(), event_content.clone()).await;
                    }
                });
            }
            Recipient::SingleRoleList(roles) => {
                tokio::spawn(async move {
                    for p_ref in players_clone.iter() {
                        if let Some(player_role) = &p_ref.meta.role {
                            if roles.contains(&player_role) {
                                dispatch_event_to_player(p_ref.value(), event_content.clone())
                                    .await;
                            }
                        }
                    }
                });
            }
            Recipient::SinglePlayerList(players_by_id) => {
                tokio::spawn(async move {
                    for p_ref in players_clone.iter() {
                        if players_by_id.contains(&p_ref.meta.player_id) {
                            dispatch_event_to_player(p_ref.value(), event_content.clone()).await;
                        }
                    }
                });
            }
        }
    }
}

/* Each invocation of this function requires a heap allocation, but that should not be problematic
given the expected frequency of calls (~10/sec). */
async fn dispatch_event_to_player(player: &ActivePlayer, event_content: EventContent) {
    player
        .connection
        .event_sender
        .send(event_content)
        .await
        .expect(SEND_ERROR_MSG);
}
