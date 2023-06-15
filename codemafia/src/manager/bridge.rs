/* This struct enables communication between the room and the game using message passing.
The benefit of this approach is that game server logic and player/room management are not coupled (SOC). */

use crate::misc::events::Event;
use shared::messages::game::GameMessage;
use tokio::sync::mpsc::{Receiver, Sender};

pub struct RoomToGameBridge {
    /* Used by the room to send game messages to the game server. */
    pub game_channel_rx: Receiver<GameMessage>,
    /* Used by the game to relay events back to the dispatcher. */
    pub room_channel_tx: Sender<Event>,
}
