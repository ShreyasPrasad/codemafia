/*
    Room

    This module contains the struct that organizes players (see mod player) into a single group, allowing
    broadcast communication. Access to this struct is not thread-safe and should be made synchronous
    using some concurrency primitive (see the use of Dashmap in mod.rs).
*/

use crate::game::Message;
use crate::{player::Player, game::server::GameServer};
use std::collections::HashSet;

use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc::Receiver;

/* These are aliases for the room listener and ; this is the channel that all players send their actions to.  */
type RoomSender = Sender<Message>;
type RoomReceiver = Receiver<Message>;

/* A message buffer size of 64 should be more than sufficient as messages are handled as soon as they 
   appear from, from at most 10-12 players. */
const MSPC_BUFFER_SIZE: usize = 64;

pub struct Room {
    /* The active players in the room. */
    sender: RoomSender
}

impl Room {
    /* Initialization of a new room; starts the room, so players can now send messages to be processed. */
    pub fn new() -> Self {
        let (tx, mut rx) = mpsc::channel::<Message>(MSPC_BUFFER_SIZE);
        tokio::spawn(async move {
            let controller: RoomController = RoomController { players: HashSet::new() };
            while let Some(message) = rx.recv().await {
                controller.handle_message(message);

            }
        });
        Room { sender: tx }
    }

    pub fn get_room_sender_channel(&self) -> RoomSender {
        /* Return a clone of the room sender so the new player can send messages. */
        self.sender.clone()
    }
}

/* This struct is responisble for handling room-specific messages sent by players. To see what types of
messages it handles, look at the match statement below. */
pub struct RoomController {
    players: HashSet<Player>
}

impl RoomController {
    pub fn handle_message(&self, message: Message) {

    }
}
