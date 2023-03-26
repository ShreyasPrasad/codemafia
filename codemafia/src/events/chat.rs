/* Defines the content of a chat event.  */

pub enum ChatEvents {
    ChatMessageEvent(ChatMessageEvent)
}

pub struct ChatMessageEvent {
    sender: String,
    message: String,
}