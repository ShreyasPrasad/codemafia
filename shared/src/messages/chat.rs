/* Defines a chat message. */

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ChatMessage {
    pub text: String,
    pub sender: String,
}
