/*
    Room

    This module contains the struct that organizes players (see mod player) into a single group, enabling
    broadcast communication. Access to this struct is not thread-safe and should be made synchronous
    using some concurrency primitive (see the use of Dashmap in mod.rs).
*/

use crate::events::{EventSender, Event, Recipient};
use crate::messages::Message;
use crate::messages::chat::ChatMessage;
use crate::messages::game::GameMessage;
use crate::messages::room::RoomMessage;
use crate::player::PlayerId;
use crate::{player::Player, game::GameServer};
use std::collections::HashMap;

use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc::Receiver;

/* These are aliases for the room listener and receiver; this is the channel that all players send their actions to.  */
pub type RoomSender = Sender<Message>;
pub type RoomReceiver = Receiver<Message>;

/* A message buffer size of 64 should be more than sufficient as messages are handled as soon as they 
   appear from, from at most 10-12 players. */
const MSPC_BUFFER_SIZE: usize = 64;

pub struct Room {
    /* The clonable sender that the RoomController listens to; available to clients using get_room_sender() below. */
    sender: RoomSender
}

impl Room {
    /* Initialization of a new room; starts the room, so players can now send messages to be processed. */
    pub fn new() -> Self {
        let (tx, mut rx) = mpsc::channel::<Message>(MSPC_BUFFER_SIZE);
        tokio::spawn(async move {
            let controller: RoomController = RoomController { players: HashMap::new(), owner: None };
            while let Some(message) = rx.recv().await {
                controller.handle_message(message);
            }
        });
        Room { sender: tx }
    }

    pub fn get_room_sender(&self) -> RoomSender {
        /* Return a clone of the room sender so the new client can send messages. */
        self.sender.clone()
    }
}

/* This struct is responisble for handling room-specific messages sent by players. To see what types of
messages it handles, look at the match statement below. */
pub struct RoomController {
    players: HashMap<PlayerId, Player>,
    /* The first player to join the room is assigned owner and is responsible for starting the game. */
    owner: Option<PlayerId>
}

/* This struct enables bidrectional communication between the room and the game using message passing.
The benefit of this approach is that game server logic and player/room management are not coupled (SOC). */
pub struct RoomToGameBridge {
    pub game_channel: Receiver<GameMessage>,
    pub room_channel: EventSender
}

impl RoomController {
    pub fn handle_message(&self, message: Message) {
        match message {
            Message::Chat(chat_message) => self.handle_chat_message(chat_message),
            Message::Game(game_message) => self.handle_game_message(game_message),
            Message::Room(room_message) => self.handle_room_message(room_message)
        }
    }

    fn handle_chat_message(&self, message: ChatMessage){

    }

    fn handle_game_message(&self, message: GameMessage){

    }

    fn handle_room_message(&self, message: RoomMessage){

    }

    fn dispatch_event(&self, event: Event) {
        match event.recipient {
            Recipient::All => {
                for (_, player) in self.players.iter() {
                    
                }
            },
            Recipient::SingleRoleList(roles) => {

            }
        }
    }
}
