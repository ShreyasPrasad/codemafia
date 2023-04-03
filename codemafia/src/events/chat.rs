/* Defines the content of a chat event.  */

use serde::Serialize;

#[derive(Clone, Serialize)]
pub enum ChatEvents {
    ChatMessageEvent(ChatMessageEvent)
}

#[derive(Clone, Serialize)]
pub struct ChatMessageEvent {
    pub sender: String,
    pub text: String,
}