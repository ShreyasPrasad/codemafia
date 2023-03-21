/* 
    Event
    
    Represents an event that is distributed to all the players that are in the game.
*/

use tokio::sync::oneshot;

pub type EventSender = oneshot::Sender<Event>;

pub enum Event {

}