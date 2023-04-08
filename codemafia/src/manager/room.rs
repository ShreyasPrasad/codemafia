/*
    Room

    This module contains the struct that organizes players (see mod player) into a single group, enabling
    broadcast communication. Access to this struct is not thread-safe and should be made synchronous
    using some concurrency primitive (see the use of Dashmap in mod.rs).
*/

use crate::events::chat::{ChatMessageEvent, ChatEvents};
use crate::events::room::{RoomEvents, RoomState, PlayerOnTeam, You};
use crate::events::{Event, Recipient, EventContent, SEND_ERROR_MSG};
use crate::game::GameServer;
use crate::messages::internal::InternalMessage;
use crate::messages::{Message, ClientMessage, Message::Client, Message::Internal};
use crate::messages::chat::ChatMessage;
use crate::messages::game::{GameMessage, Team};
use crate::messages::room::{RoomMessage, RoomMessageAction};
use crate::player::{PlayerId, PlayerError, PlayerChannel};
use crate::player::Player;
use crate::player::role::CodeMafiaRole;
use crate::wordbank::creator::Creator;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use dashmap::DashMap;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc::Receiver;
use uuid::Uuid;

use super::dispatcher::EventDispatcher;

/* These are aliases for the room listener and receiver; this is the channel that all players send their actions to.  */
pub type MessageSender = Sender<Message>;
pub type MessageReceiver = Receiver<Message>;

/* A message buffer size of 64 should be more than sufficient as room messages are handled as soon as they 
   appear from, from at most 10-12 players. */
const ROOM_MSPC_BUFFER_SIZE: usize = 64;

/* A message buffer size of 8 should be more than sufficient as game messages are between 2 agents (the room
    and the game server). */
const GAME_MSPC_BUFFER_SIZE: usize = 8;

pub struct Room {
    /* The clonable sender that the RoomController listens to; available to clients using get_room_sender() below. */
    sender: MessageSender
}

impl Room {
    /* Initialization of a new room; starts the room, so players can now send messages to be processed. */
    pub fn new(game_creator: Arc<Mutex<Creator>>) -> Self {
        let (tx, mut rx) = mpsc::channel::<Message>(ROOM_MSPC_BUFFER_SIZE);
        tokio::spawn(async move {
            let players: Arc<DashMap<PlayerId, Player>> = Arc::new(DashMap::new());
            let dispatcher: EventDispatcher = EventDispatcher::new(players.clone());
            let mut controller: RoomController = RoomController { 
                players: players.clone(), 
                owner: None, 
                active_game: None, 
                game_creator,
                event_dispatcher: dispatcher.get_event_sender()
            };
            
            while let Some(message) = rx.recv().await {
                controller.handle_message(message).await;
            }
        });
        Room { sender: tx }
    }

    pub fn get_room_sender(&self) -> MessageSender {
        /* Return a clone of the room sender so the new client can send messages. */
        self.sender.clone()
    }
}

/* This struct is responisble for handling room-specific messages sent by players. To see what types of
messages it handles, look at the match statement below. */
pub struct RoomController {
    players: Arc<DashMap<PlayerId, Player>>,
    /* The first player to join the room is assigned owner and is responsible for starting the game. */
    owner: Option<PlayerId>,
    /* The active game, if any, owned by the room. */
    active_game: Option<Sender<GameMessage>>,
    /* The shared game creator. */
    game_creator: Arc<Mutex<Creator>>,
    /* The event dispatcher, responsible for forwarding events to players. */
    pub event_dispatcher: Sender<Event>
}

/* This struct enables communication between the room and the game using message passing.
The benefit of this approach is that game server logic and player/room management are not coupled (SOC). */
pub struct RoomToGameBridge {
    /* Used by the room to send game messages to the game server. */
    pub game_channel_rx: Receiver<GameMessage>,
    /* Used by the game to relay events back to the dispatcher. */
    pub room_channel_tx: Sender<Event>
}

impl RoomController {
    pub async fn handle_message(&mut self, message: Message) {
        match message {
            Client(ClientMessage::Chat(chat_message)) => {
                self.handle_chat_message(chat_message).await;
            },
            Client(ClientMessage::Game(game_message)) => {
                /* Make sure game messages are sent to the game server asynchronously, so we dont block a thread. */
                self.handle_game_message(game_message).await;
            },
            Client(ClientMessage::Room(room_message)) => {
                self.handle_room_message(room_message).await;
            },
            Internal(internal_message) => {
                self.handle_internal_message(internal_message).await;
            }
        };
    }

    async fn handle_chat_message(&self, message: ChatMessage){
        /* Relay the chat message to all active players. */
        self.event_dispatcher.send(
            Event { 
                recipient: Recipient::All, 
                content: EventContent::Chat(ChatEvents::ChatMessageEvent(
                    ChatMessageEvent{
                        sender: message.sender,
                        text: message.text
                    }
                )) 
            }
        ).await.expect(SEND_ERROR_MSG);
    }

    async fn handle_game_message(&self, message: GameMessage){
        if let Some(active_game) = &self.active_game {
            /* TODO: Figure out a way to not do a blocking send. */
            if let Err(err) = active_game.send(message).await {
                println!("Error forwarding game message to server: {}", err);
            }
        }
    }

    async fn handle_room_message(&mut self, message: RoomMessage){
        match message.action {
            RoomMessageAction::JoinTeam(player_id, team) => {
                let player_id = Uuid::from_str(&player_id).unwrap();
                /* Update the team and send room state update to all players upon success. */
                if let Ok(()) = self.update_player_team(player_id, team) {
                  self.dispatch_room_state_update().await;
                } 
            },
            RoomMessageAction::StartGame => {
                /* Create the new game. */
                self.start_game();
                self.event_dispatcher.send(
                    Event {
                        recipient: Recipient::All,
                        content: EventContent::Room(RoomEvents::GameStarted)
                    }
                ).await.expect(SEND_ERROR_MSG);
            }
        };
    }

    async fn handle_internal_message(&mut self, message: InternalMessage){
        match message {
            InternalMessage::NewPlayer(player_name, event_sender) => {
                self.create_player(player_name, event_sender);
            },
            InternalMessage::SessionConnection(player_id, you_receiver) => {
                match self.players.get(&player_id) {
                    Some(player) => {
                        you_receiver.send(Some(You{
                            name: player.name.clone(),
                            id: player.player_id.to_string()
                        })).expect(SEND_ERROR_MSG);
                    },
                    None => {
                        you_receiver.send(None).expect(SEND_ERROR_MSG);
                    }
                }
            },
            InternalMessage::UpdatePlayer(player_id, event_sender) => {
                if let Err(err) = self.update_player(player_id, event_sender) {
                    println!("Error updating player: {}", err);
                }
            }
        }
        /* Update the players after all the actions above. */
        self.dispatch_room_state_update().await;
    }

    fn start_game(&mut self) {
        let mut sync_game_creator = self.game_creator.lock().unwrap();
        /* Create a new game for the room. */
        let game = sync_game_creator.get_game();

        let (game_channel_tx, game_channel_rx) = mpsc::channel::<GameMessage>(GAME_MSPC_BUFFER_SIZE);
        /* Construct the room-to-game bridge. */
        let bridge: RoomToGameBridge = RoomToGameBridge { game_channel_rx, room_channel_tx: self.event_dispatcher.clone() };
        /* Create the game server. */
        let mut game_server: GameServer = GameServer::new(game, bridge);
        /* Start the game loop. */
        tokio::spawn(async move {
            game_server.start_game_loop().await;
        });
        /* Save the message sender so we can forward game messages received from players. */
        self.active_game = Some(game_channel_tx);   
    }

    fn create_player(&mut self, player_name: String, event_sender: Sender<EventContent>) -> Player {
        let new_player = Player { 
            player_id: Uuid::new_v4(), 
            channel: PlayerChannel{ event_sender }, 
            role: None, 
            name: Some(player_name)
        };
        /* Assign the owner. */
        if self.players.is_empty() {
            self.owner = Some(new_player.player_id);
        }
        new_player
    }

    fn update_player(&mut self, player_id: PlayerId, event_sender: Sender<EventContent>) -> Result<(), PlayerError>{
        match self.players.get_mut(&player_id) {
            Some(mut player) => {
                player.channel.event_sender = event_sender;
                Ok(())
            },
            None => {
                Err(PlayerError::DoesNotExist)
            }
        }
    }

    async fn dispatch_room_state_update(&self) {
        self.event_dispatcher.send(
            Event { 
                recipient: Recipient::All,
                content: EventContent::Room(RoomEvents::RoomState(self.get_room_state()))
            }
        ).await.expect(SEND_ERROR_MSG);
    }

    fn update_player_team(&mut self, player_id: PlayerId, team: Team) -> Result<(), PlayerError> {
        match self.players.get_mut(&player_id) {
            Some(mut p_ref) => {
                match &mut p_ref.role {
                    Some(role) => {
                        role.team = team;
                        Ok(())
                    },
                    None => {
                        p_ref.role = Some(CodeMafiaRole { role_title: None, team });
                        Ok(())
                    }
                }
            },
            None => Err(PlayerError::DoesNotExist)
        }
    }

    /* Construct the room state from the list of active players */
    fn get_room_state(&self) -> RoomState {
        let mut active_players: Vec<PlayerOnTeam> = vec![];
        self.players.iter().for_each(|p_ref| {
            if let Some(player_name) = &p_ref.name {
                if let Some(player_role) = &p_ref.role {
                    active_players.push(PlayerOnTeam{name: player_name.to_string(), team: player_role.team.clone()});
                }
            }
        });
        RoomState {
            players: active_players
        }
    }
}
