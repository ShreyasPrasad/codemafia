/* Defines the content of a game event.  */

use serde::Serialize;

use super::EventContent;

#[derive(Debug, Clone, Serialize)]
pub enum PlayerEvents {
    /* Sent to a new player connecting to the game to allow for seamless reconnects. */
    SetPlayerIdCookie(String),
    FastForwardGameEvents(Vec<EventContent>),
}
