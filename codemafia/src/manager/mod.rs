/* 
    Manager

    This module contains the logic that handles game room creation and routing
    to these rooms using game codes, all in a single struct.

    This struct is thread-safe; clients can safely perform operations like
    game creation and removal concurrently.

*/

use std::sync::Arc;
use dashmap::DashMap;
use rand::{Rng, rngs::ThreadRng}; // 0.8

pub mod room;

use self::room::Room;

/* Constants used to define the RNG that generates game codes that are distributed to player. */
const ROOM_CODE_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const ROOM_CODE_LEN: usize = 4;

pub struct RoomManager {
    rooms: DashMap<RoomCode, Room>,
}

// Create a convenience aliasing type
type RoomCode = String;

impl RoomManager {
    pub fn new(self) -> Self {
        RoomManager {
            rooms: DashMap::<RoomCode, Room>::new(),
        }
    }
    /* Invoked when a room creation request is made. */
    pub fn create_room(&mut self){
        /* Add the room to the manager so at least one reference to the Sender exists (and thus receiver is not dropped). */
        self.rooms.insert(self.get_room_code(), Room::new());
    }

    /* Invoked by a game when it is completed and should be cleaned up from within the manager. */
    pub fn remove_room(&mut self, game_code: String){
        /* Removing the game from the map kills the Tokio game server task since mspc::Sender ref count reaches 0. */
        self.rooms.remove(&game_code);
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