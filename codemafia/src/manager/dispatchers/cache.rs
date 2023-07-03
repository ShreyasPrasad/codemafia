use async_trait::async_trait;
use shared::{events::EventContent, player::PlayerMetadata};
use std::sync::{Arc, RwLock};

use dashmap::DashMap;
use tokio::sync::mpsc::{self, Sender};

use crate::misc::{events::Event, player::ActivePlayer};
use shared::player::PlayerId;

use crate::misc::events::is_event_for_player_with_role;

use super::EventDispatcher;

/* The cache sequence is a 0 indexed pointer to the last processed element in the event cache. */
pub type CacheSequence = u8;

/* The event cache is used to maintain a list of ordered events that represent the current game.
When a player attempts to reconnect to the game, they receive the EventCache, allowing them to
repopulate the game on their client independently. */
#[derive(Debug, Clone)]
pub struct EventCache {
    cache: Vec<Event>,
}

impl EventCache {
    pub fn new() -> Self {
        EventCache { cache: vec![] }
    }

    pub fn add_event_to_cache(&mut self, content: Event) {
        self.cache.push(content)
    }

    pub fn get_role_based_cache_from_sequence(
        &self,
        sequence: CacheSequence,
        player_meta: PlayerMetadata,
    ) -> Option<Vec<Event>> {
        /* Check if the given sequence exceeds the size of the cache. */
        if sequence >= self.cache.len() as u8 {
            return None;
        }
        // Find and clone the events that apply to the player with the given metadata
        let events_for_player: Vec<Event> = self.cache.as_slice()[sequence as usize..]
            .iter()
            .filter(|&event| is_event_for_player_with_role(event, &player_meta))
            .cloned()
            .collect::<Vec<Event>>();

        Some(events_for_player)
    }
}

pub struct CachedEventDispatcher {
    event_sender: Sender<Event>,
    event_cache: Arc<RwLock<EventCache>>,
}

/* The size of the channel for events received and pending dispatch. */
const CACHED_EVENT_DISPATCHER_CHANNEL_SIZE: usize = 16;

impl EventDispatcher for CachedEventDispatcher {
    fn new(players: Arc<DashMap<PlayerId, ActivePlayer>>) -> Self {
        let (tx, mut rx) = mpsc::channel::<Event>(CACHED_EVENT_DISPATCHER_CHANNEL_SIZE);
        let event_cache = Arc::new(RwLock::new(EventCache::new()));
        let players_clone = players.clone();
        let event_cache_clone = event_cache.clone();
        tokio::spawn(async move {
            while let Some(event) = rx.recv().await {
                Self::dispatch_event(players_clone.clone(), event.clone());
                /* Add game, chat, and room events to the cache, after they are dispatched to players. */
                match &event.content {
                    EventContent::Game(_) | EventContent::Chat(_) | EventContent::Room(_) => {
                        Self::add_event_to_cache(event.clone(), event_cache_clone.clone())
                    }
                    _ => (),
                };
            }
        });
        Self {
            event_sender: tx,
            event_cache,
        }
    }

    fn get_event_sender(&self) -> Sender<Event> {
        self.event_sender.clone()
    }
}

impl CachedEventDispatcher {
    pub fn get_event_cache(&self) -> Arc<RwLock<EventCache>> {
        self.event_cache.clone()
    }

    /* TODO: This method acquires a write-lock on the EventCache, appending the given event to it. Since the event
    cache is an append-only data structure, there may be more efficient concurrency primitives to update it
    (aside from a RWLock), but it will do for now. */
    fn add_event_to_cache(event: Event, event_cache: Arc<RwLock<EventCache>>) {
        let mut write_guard = event_cache.write().unwrap();
        write_guard.add_event_to_cache(event);
    }
}
