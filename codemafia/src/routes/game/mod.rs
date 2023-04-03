/* The key used to find the player ID among the cookies of an incoming request. */
pub const PLAYER_ID_COOKIE_KEY: &'static str = "player_id";

/* Route to allow players with a valid, existing session to reconnect to the same game. */
pub mod session;
/* Route to allow new players to connect to the game. */
pub mod game;
/* Share utilities between session and game. */
pub mod util;