/*
    Room

    This module contains the struct that organizes players (see mod player) into a single group, enabling
    broadcast communication. Access to this struct is not thread-safe and should be made synchronous
    using some concurrency primitive (see the use of Dashmap in mod.rs).
*/

use crate::creator::Creator;
use crate::misc::internal::InternalMessage;
use crate::misc::player::ActivePlayer;
use shared::messages::Message;
use shared::player::PlayerId;
use std::sync::{Arc, Mutex};

use dashmap::DashMap;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use super::controllers::internal::{InternalSender, INTERNAL_MSPC_BUFFER_SIZE, InternalController};
use super::controllers::shared::SharedController;
use super::dispatcher::EventDispatcher;

/* These are aliases for the room listener and receiver; this is the channel that all players send their actions to.  */
pub type MessageSender = Sender<Message>;

/* A message buffer size of 64 should be more than sufficient as room messages are handled as soon as they 
   appear from, from at most 10-12 players. */
const ROOM_MSPC_BUFFER_SIZE: usize = 64;

pub struct Room {
    /* The clonable sender that the RoomController listens to for shared messages; available to clients using get_shared_sender() below. */
    shared_sender: MessageSender,
    /* The clonable internal sender that the RoomController listens to; available to clients using get_internal_sender() below. */
    internal_sender: InternalSender
}

impl Room {
    /* Initialization of a new room; starts the room, so players can now send messages to be processed. */
    pub fn new(game_creator: Arc<Mutex<Creator>>) -> Self {
        let shared_sender = Self::start_shared_task(game_creator);
        let internal_sender = Self::start_internal_task();
        Room { shared_sender, internal_sender }
    }

    fn start_shared_task(game_creator: Arc<Mutex<Creator>>) -> MessageSender {
        let (tx, mut rx) = mpsc::channel::<Message>(ROOM_MSPC_BUFFER_SIZE);
        tokio::spawn(async move {
            let players: Arc<DashMap<PlayerId, ActivePlayer>> = Arc::new(DashMap::new());
            let dispatcher: EventDispatcher = EventDispatcher::new(players.clone());
            let mut controller: SharedController = SharedController::new(
                players.clone(),
                game_creator,
                dispatcher.get_event_sender()
            );
            
            while let Some(message) = rx.recv().await {
                controller.handle_message(message).await;
            }
        });
        tx
    }

    fn start_internal_task() -> InternalSender {
        let (tx, mut rx) = mpsc::channel::<InternalMessage>(INTERNAL_MSPC_BUFFER_SIZE);
        tokio::spawn(async move {
            let players: Arc<DashMap<PlayerId, ActivePlayer>> = Arc::new(DashMap::new());
            let dispatcher: EventDispatcher = EventDispatcher::new(players.clone());
            let mut controller: InternalController = InternalController::new(players, dispatcher.get_event_sender());
            while let Some(message) = rx.recv().await {
                controller.handle_message(message).await;
            }
        });
        tx
    }

    pub fn get_shared_sender(&self) -> MessageSender {
        /* Return a clone of the room sender so the new client can send messages. */
        self.shared_sender.clone()
    }

    pub fn get_internal_sender(&self) -> InternalSender {
        /* Return a clone of the room sender so the new client can send messages. */
        self.internal_sender.clone()
    }
}