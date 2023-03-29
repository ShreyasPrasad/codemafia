/* Defines the content of a chat event.  */

#[derive(Clone)]
pub enum ChatEvents {
    ChatMessageEvent(ChatMessageEvent)
}

#[derive(Clone)]
pub struct ChatMessageEvent {
    pub sender: String,
    pub text: String,
}