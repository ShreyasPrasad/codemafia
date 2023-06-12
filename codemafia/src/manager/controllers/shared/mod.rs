use std::sync::{Arc, Mutex};

use shared::elements::Game;
use shared::events::EventContent;
use shared::events::chat::{ChatMessageEvent, ChatEvents};
use shared::events::room::RoomEvents;
use shared::player::role::{CodeMafiaRoleTitle, CodeMafiaRole};
use crate::creator::Creator;
use crate::manager::bridge::RoomToGameBridge;
use crate::misc::events::{Event, Recipient, SEND_ERROR_MSG};
use crate::misc::player::ActivePlayer;
use shared::messages::Message;
use shared::messages::chat::ChatMessage;
use shared::messages::game::{GameMessage, Team};
use shared::messages::room::{RoomMessage, RoomMessageAction};
use shared::player::{PlayerId, PlayerError};
use crate::game::GameServer;
use std::str::FromStr;

use dashmap::DashMap;
use tokio::sync::mpsc::{Sender, self};
use uuid::Uuid;

use super::util::dispatch_room_state_update;

/* A message buffer size of 8 should be more than sufficient as game messages are between 2 agents (the room
    and the game server). */
const GAME_MSPC_BUFFER_SIZE: usize = 8;

/* This controller is responisble for handling room-specific messages sent by players. To see what types of
messages it handles, look at the match statement below. */
pub struct SharedController {
    /* A reference to the list of active players. */
    players: Arc<DashMap<PlayerId, ActivePlayer>>,
    /* The active game, if any, owned by the room. */
    active_game: Option<Sender<GameMessage>>,
    /* The shared game creator. */
    game_creator: Arc<Mutex<Creator>>,
    /* The event dispatcher, responsible for forwarding events to players. */
    event_sender: Sender<Event>
}

impl SharedController {
    pub fn new(players: Arc<DashMap<PlayerId, ActivePlayer>>, game_creator: Arc<Mutex<Creator>>, event_sender: Sender<Event>) -> Self {
        SharedController { players, active_game: None, game_creator, event_sender }
    }

    pub async fn handle_message(&mut self, message: Message) {
        match message {
            Message::Chat(chat_message) => {
                self.handle_chat_message(chat_message).await;
            },
            Message::Game(game_message) => {
                /* Make sure game messages are sent to the game server asynchronously, so we dont block a thread. */
                self.handle_game_message(game_message).await;
            },
            Message::Room(room_message) => {
                self.handle_room_message(room_message).await;
            }
        };
    }

    async fn handle_chat_message(&self, message: ChatMessage){
        /* Relay the chat message to all active players. */
        self.event_sender.send(
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
            RoomMessageAction::JoinTeam(player_id, team, is_spymaster) => {
                let player_id = Uuid::from_str(&player_id).unwrap();
                /* Update the team and send room state update to all players upon success. */
                if let Ok(()) = self.update_player_team(player_id, team, is_spymaster) {
                  dispatch_room_state_update(&self.event_sender, self.players.clone()).await;
                } 
            },
            RoomMessageAction::StartGame => {
                /* Create the new game. */
                self.start_game().await;
                self.event_sender.send(
                    Event {
                        recipient: Recipient::All,
                        content: EventContent::Room(RoomEvents::GameStarted)
                    }
                ).await.expect(SEND_ERROR_MSG);
            }
        };
    }

    pub fn update_player_team(&mut self, player_id: PlayerId, team: Team, is_spymaster: bool) -> Result<(), PlayerError> {
        let role_title: Option<CodeMafiaRoleTitle> = 
            if is_spymaster { Some(CodeMafiaRoleTitle::SpyMaster) } else { Some(CodeMafiaRoleTitle::Ally) };

        match self.players.get_mut(&player_id) {
            Some(mut p_ref) => {
                p_ref.meta.role = Some(CodeMafiaRole { role_title, team });
                Ok(())
            },
            None => Err(PlayerError::DoesNotExist)
        }
    }

    async fn start_game(&mut self) {
        let game: Game;
        /* Make sure we are not holding a MutexGuard across an .await call. */
        {
            let mut sync_game_creator = self.game_creator.lock().unwrap();
            /* Create a new game for the room. */
            game = sync_game_creator.get_game();
        }
        let (game_channel_tx, game_channel_rx) = mpsc::channel::<GameMessage>(GAME_MSPC_BUFFER_SIZE);
        /* Construct the room-to-game bridge. */
        let bridge: RoomToGameBridge = RoomToGameBridge { game_channel_rx, room_channel_tx: self.event_sender.clone() };
        /* Create the game server. */
        let mut game_server: GameServer = GameServer::new(game, bridge, self.players.clone());
        /* Initialize the game. */
        game_server.init_game().await;
        /* Start the game loop. */
        tokio::spawn(async move {
            game_server.start_game_loop().await;
        });
        /* Save the message sender so we can forward game messages received from players. */
        self.active_game = Some(game_channel_tx);   
    }
}
