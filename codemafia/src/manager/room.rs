/*
    Room

    This module contains the struct that organizes players (see mod player) into a single group, enabling
    broadcast communication. Access to this struct is not thread-safe and should be made synchronous
    using some concurrency primitive (see the use of Dashmap in mod.rs).
*/

use crate::events::chat::{ChatMessageEvent, ChatEvents};
use crate::events::room::{RoomEvents, RoomState, PlayerOnTeam};
use crate::events::{EventSender, Event, Recipient, EventContent};
use crate::messages::Message;
use crate::messages::chat::ChatMessage;
use crate::messages::game::GameMessage;
use crate::messages::room::{RoomMessage, RoomMessageAction};
use crate::player::PlayerId;
use crate::player::Player;
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
    /* Used by the room to send game messages to the game server. */
    pub game_channel: Receiver<GameMessage>,
    /* Used by the game to relay events back to the room. */
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
        /* Relay the chat message to all active players. */
        self.dispatch_event(
            Event { 
                recipient: Recipient::All, 
                content: EventContent::Chat(ChatEvents::ChatMessageEvent(
                    ChatMessageEvent{
                        sender: message.sender,
                        text: message.text
                    }
                )) 
            }
        )
    }

    fn handle_game_message(&self, message: GameMessage){
        todo!()
    }

    fn handle_room_message(&self, message: RoomMessage){
        match message.action {
            RoomMessageAction::InitialConnection(player_id) => {
                self.dispatch_event(
                    Event { 
                        recipient: Recipient::SinglePlayerList(vec![player_id]), 
                        content: EventContent::Room(RoomEvents::RoomState(self.get_room_state()))
                    }
                )
            },
            RoomMessageAction::JoinTeam(player_name, team) => {
                self.dispatch_event(
                    Event { 
                        recipient: Recipient::All,
                        content: EventContent::Room(RoomEvents::PlayerJoinedTeam(PlayerOnTeam {
                            name: player_name,
                            team
                        }))
                    }
                )
            },
            RoomMessageAction::StartGame => {
                self.dispatch_event(
                    Event { 
                        recipient: Recipient::All,
                        content: EventContent::Room(RoomEvents::GameStarted)
                    }
                )
            }
        }
    }

    fn dispatch_event(&self, event: Event) {
        match event.recipient {
            Recipient::All => {
                for (_, player) in self.players.iter() {
                    /* Do not await sending the event. */
                    player.channel.event_sender.send(event.content.clone());
                }
            },
            Recipient::SingleRoleList(roles) => {
                for (_, player) in self.players.iter() {
                    if let Some(player_role) = &player.role {
                        if roles.contains(&player_role){
                            /* Do not await sending the event. */
                            player.channel.event_sender.send(event.content.clone());
                        }
                    }
                }
            },
            Recipient::SinglePlayerList(players_by_id) => {
                for (_, player) in self.players.iter() {
                    if players_by_id.contains(&player.player_id) {
                        /* Do not await sending the event. */
                        player.channel.event_sender.send(event.content.clone());
                    }
                }
            }
        }
    }

    /* Construct the room state from the list of active players */
    fn get_room_state(&self) -> RoomState {
        let mut active_players: Vec<PlayerOnTeam> = vec![];
        self.players.iter().for_each(|(_, player)| {
            if let Some(player_name) = &player.name {
                if let Some(player_role) = &player.role {
                    active_players.push(PlayerOnTeam{name: player_name.to_string(), team: player_role.team.clone()});
                }
            }
        });
        RoomState {
            players: active_players
        }
    }
}
