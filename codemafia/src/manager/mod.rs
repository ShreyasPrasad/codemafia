/* 
    Manager

    This module contains the logic that handles game room creation and routing
    to these rooms using game codes, all in a single struct.

    This struct is not thread-safe; clients should ensure the RoomManager can be
    invoked concurrently to ensure parallel game creation.
*/

use std::{collections::HashMap, sync::{Arc, Mutex}};
use rand::{Rng, rngs::ThreadRng}; // 0.8

pub mod room;
pub mod dispatcher;
pub mod player;
pub mod bridge;

use crate::{manager::room::{Room, MessageSender}, wordbank::creator::Creator};

/* Constants used to define the RNG that generates game codes that are distributed to player. */
const ROOM_CODE_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const ROOM_CODE_LEN: usize = 4;

pub struct RoomManager {
    rooms: HashMap<RoomCode, Room>,
    game_creator: Arc<Mutex<Creator>>
}

// Create a convenience aliasing type
pub type RoomCode = String;

impl RoomManager {
    pub fn new() -> Self {
        RoomManager {
            rooms: HashMap::<RoomCode, Room>::new(),
            game_creator: Arc::new(Mutex::new(Creator::new().unwrap()))
        }
    }
    /* Invoked when a room creation request is made. */
    pub fn create_room(&mut self) -> RoomCode {
        let new_room_code: RoomCode = self.get_room_code();
        self.rooms.insert(new_room_code.clone(), Room::new(self.game_creator.clone()));
        new_room_code
    }

    /* Invoked by a game when it is completed and should be cleaned up from within the manager. */
    pub fn remove_room(&mut self, room_code: RoomCode){
        self.rooms.remove(&room_code);
    }

    /* Invoked to obtain the RoomSender for a particular game room; returns None if the room doesn't exist. */
    pub fn get_room_handle(&self, room_code: RoomCode) -> Option<MessageSender> {
        match self.rooms.get(&room_code) {
            Some(room) => Some(room.get_room_sender()),
            None => None
        }
    }

    fn get_room_code(&self) -> RoomCode {
        /* Currently, conflicting game codes are not handled; they have a negligible chance of occuring. */
        let mut rng: ThreadRng = rand::thread_rng();

        let room_code: String = (0..ROOM_CODE_LEN)
            .map(|_| {
                let idx: usize = rng.gen_range(0..ROOM_CODE_CHARSET.len());
                ROOM_CODE_CHARSET[idx] as char
            })
            .collect();

        room_code
    }
}