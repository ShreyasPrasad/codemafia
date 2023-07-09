use tokio::sync::{mpsc::Sender, oneshot};

use shared::{
    events::EventContent,
    player::{PlayerId, PlayerMetadata},
};

/* This enum consists of messages sent without an active player context established, such as when
a player joins for the first time or reconnects. */
#[derive(Debug)]
pub enum InternalMessage {
    /* This message is sent by the websocket statemachine upon receiving a sessioned player connection. */
    SessionConnection(PlayerId, PlayerMetadataReceiver),
    /* Message received when a new player connects. */
    NewPlayer(
        String, /* PlayerName */
        Sender<EventContent>,
        PlayerMetadataReceiver,
    ),
    UpdatePlayer(PlayerId, Sender<EventContent>),
    PlayerDisconnected(PlayerId), /* Message received when a player that is currently connected, disconnects. */
}

/* Type used for situations where an active player has not yet been associated with an incoming connection,
and we need information about them from the room. */
pub type PlayerMetadataReceiver = oneshot::Sender<Option<PlayerMetadata>>;
