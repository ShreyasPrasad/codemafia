use shared::{
    events::{game::GameEvents, EventContent},
    misc::sequenced::{SequenceNum, Sequenced},
    player::PlayerMetadata,
};
use std::sync::{Arc, RwLock};

use dashmap::DashMap;
use tokio::sync::mpsc::{self, Sender};

use crate::misc::{events::Event, player::ActivePlayer};
use shared::player::PlayerId;

use crate::misc::events::is_event_for_player_with_role;

use super::EventDispatcher;

/* The event cache is used to maintain a list of ordered events that represent the current game.
When a player attempts to reconnect to the game, they receive sequenced events from the EventCache,
allowing them to repopulate the game on their client independently. */
#[derive(Debug, Clone)]
pub struct EventCache {
    cache: Vec<Sequenced<Event>>,
}

impl EventCache {
    pub fn new() -> Self {
        EventCache { cache: vec![] }
    }

    pub fn add_event_to_cache(&mut self, event: Sequenced<Event>) {
        self.cache.push(event)
    }

    pub fn get_role_based_cache_from_sequence(
        &self,
        from_sequence_num: SequenceNum,
        player_meta: PlayerMetadata,
    ) -> Option<Vec<Sequenced<Event>>> {
        /* Check if the given sequence exceeds the size of the cache. */
        if from_sequence_num >= self.cache.len() as u8 {
            return None;
        }
        // Find and clone the events that apply to the player with the given metadata
        let events_for_player: Vec<Sequenced<Event>> = self
            .cache
            .iter()
            /* Too lazy to do binary search using the event_sequence_num. */
            .filter(|&event| {
                event.sequence_num > from_sequence_num
                    && is_event_for_player_with_role(&event.item, &player_meta)
            })
            .cloned()
            .collect::<Vec<Sequenced<Event>>>();

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
            let mut current_event_sequence_num = 0;
            while let Some(event) = rx.recv().await {
                /* Assign the event's sequence number. */
                let sequenced_event = Sequenced::new(event.clone(), current_event_sequence_num);
                Self::dispatch_event(players_clone.clone(), sequenced_event.clone());
                /* Add chat events and certain game/room events to the cache, and certain game events after they are dispatched to players. */
                match &event.content {
                    EventContent::Chat(..) => {
                        Self::add_event_to_cache(sequenced_event, event_cache_clone.clone())
                    }
                    EventContent::Game(game_event) => match game_event {
                        GameEvents::GameEnded(..)
                        | GameEvents::WordClicked(..)
                        | GameEvents::RoleUpdated(..) => {
                            Self::add_event_to_cache(sequenced_event, event_cache_clone.clone())
                        }
                        _ => (),
                    },
                    EventContent::Room(..) => {
                        Self::add_event_to_cache(sequenced_event, event_cache_clone.clone())
                    }
                    _ => (),
                };
                /* Increment the sequence number counter. */
                current_event_sequence_num += 1;
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
    fn add_event_to_cache(event: Sequenced<Event>, event_cache: Arc<RwLock<EventCache>>) {
        let mut write_guard = event_cache.write().unwrap();
        write_guard.add_event_to_cache(event);
    }
}
