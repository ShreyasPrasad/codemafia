use std::sync::Arc;

use dashmap::DashMap;
use tokio::sync::mpsc::{self, Sender};

use crate::misc::{events::Event, player::ActivePlayer};
use shared::player::PlayerId;

use super::EventDispatcher;

pub struct DefaultEventDispatcher {
    event_sender: Sender<Event>,
}

/* The size of the channel for events received and pending dispatch. */
const DEFAULT_EVENT_DISPATCHER_CHANNEL_SIZE: usize = 8;

impl EventDispatcher for DefaultEventDispatcher {
    fn new(players: Arc<DashMap<PlayerId, ActivePlayer>>) -> Self {
        let (tx, mut rx) = mpsc::channel::<Event>(DEFAULT_EVENT_DISPATCHER_CHANNEL_SIZE);
        let players_clone = players.clone();
        tokio::spawn(async move {
            while let Some(event) = rx.recv().await {
                Self::dispatch_event(players_clone.clone(), event.clone());
            }
        });
        Self { event_sender: tx }
    }

    fn get_event_sender(&self) -> Sender<Event> {
        self.event_sender.clone()
    }
}
