use std::sync::Arc;

use dashmap::DashMap;
use tokio::sync::mpsc::{Sender, self};

use codemafia::{events::{Event, Recipient, SEND_ERROR_MSG}, player::{Player, PlayerId}};

pub struct EventDispatcher {
   event_sender: Sender<Event>
}

const EVENT_DISPATCHER_CHANNEL_SIZE: usize = 32;

impl EventDispatcher {
    pub fn new(players: Arc<DashMap<PlayerId, Player>>) -> EventDispatcher {
        let (tx, mut rx) = mpsc::channel::<Event>(EVENT_DISPATCHER_CHANNEL_SIZE);
        let players_clone = players.clone();
        tokio::spawn(async move {
            while let Some(event) = rx.recv().await {
                dispatch_event(players_clone.clone(), event);
            }
        });
        EventDispatcher { event_sender: tx}
    }

    pub fn get_event_sender(&self) -> Sender<Event> {
        self.event_sender.clone()
    }
}

/* This method sends the event to the included recipient (1 or more players). */
fn dispatch_event(players_clone: Arc<DashMap<PlayerId, Player>>, event: Event) {
    match event.recipient {
        Recipient::All => {
            /* Todo: parallelize event sending by assigning each event send to a new Tokio task. */
            tokio::spawn(async move {
                    for p_ref in players_clone.iter() {
                        p_ref.channel.event_sender.send(event.content.clone())
                            .await.expect(SEND_ERROR_MSG);
                    }
                }  
            );
        },
        Recipient::SingleRoleList(roles) => {
            tokio::spawn(async move {
                for p_ref in players_clone.iter() {
                    if let Some(player_role) = &p_ref.role {
                        if roles.contains(&player_role){
                            p_ref.channel.event_sender.send(event.content.clone())
                                .await.expect(SEND_ERROR_MSG);
                        }
                    }
            }});
        },
        Recipient::SinglePlayerList(players_by_id) => {
            tokio::spawn(async move {
                for p_ref in players_clone.iter() {
                    if players_by_id.contains(&p_ref.player_id) {
                        p_ref.channel.event_sender.send(event.content.clone())
                                .await.expect(SEND_ERROR_MSG);
                    }
                }
            });
        }
    }
}
