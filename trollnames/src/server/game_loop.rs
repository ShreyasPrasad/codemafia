/* Game Loop
 
 Contains the Tokio task that runs the primary game loop.

 */

use tokio::sync::mpsc::{Receiver};
use tokio::sync::oneshot;

use super::event::Event;

pub struct GameLoop;

pub struct SupplierCommand {
    resp: ClientResponder
}

// convenient shorthand for channel used to returning a game back to caller
pub type ClientResponder = oneshot::Sender<Event>;

impl GameLoop {
    pub async fn game_loop(mut rx: Receiver<SupplierCommand>) {
        while let Some(cmd) = rx.recv().await {
            
        }
    }
}
