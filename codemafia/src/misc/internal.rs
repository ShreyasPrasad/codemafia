use tokio::sync::{mpsc::Sender, oneshot};

use shared::{player::PlayerId, events::{EventContent, room::You}};

/* This enum consists of messages sent without an active player context established, such as when 
a player joins for the first time or reconnects. */
#[derive(Debug)]
pub enum InternalMessage {
       /* This message is sent by the websocket statemachine upon receiving a sessioned player connection. */
       SessionConnection(PlayerId, YouReceiver),
       /* Message received when a new player connects. */
       NewPlayer(String /* PlayerName */, Sender<EventContent>),
       UpdatePlayer(PlayerId, Sender<EventContent>)
}

/* Type used for situations where an active player has not yet been associated with an incoming connection,
and we need information about them from the room. */
pub type YouReceiver = oneshot::Sender<Option<You>>;