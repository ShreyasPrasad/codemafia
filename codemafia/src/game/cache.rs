use serde::Serialize;

use codemafia::events::EventContent;

/* The event cache is used to maintain a list of ordered events that represent the current game.
When a player attempts to reconnect to the game, they receive the EventCache, allowing them to
repopulate the game on their client independently. */
#[derive(Debug, Clone, Serialize)]
pub struct EventCache {
    pub events: Vec<EventContent>
}