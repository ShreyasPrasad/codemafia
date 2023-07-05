/* Defines the content of a game event.  */

use serde::Serialize;

use crate::misc::sequenced::Sequenced;

use super::EventContent;

#[derive(Debug, Clone, Serialize)]
pub enum PlayerEvents {
    /* Sent to a new player connecting to the game to allow for seamless reconnects. */
    SetPlayerIdCookie(String),
    FastForwardEvents(Vec<Sequenced<EventContent>>),
}
