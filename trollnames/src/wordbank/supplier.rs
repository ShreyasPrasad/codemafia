/* Supplier
  
  This module defines the async supplier of new games to clients that request them.
  
 */

use tokio::sync::mpsc::{Receiver};
use tokio::sync::oneshot;

use super::{Game, creator::{Creator, CreatorError}};

pub struct SupplierCommand {
    resp: Responder
}

// convenient shorthand for channel used to returning a game back to caller
pub type Responder = oneshot::Sender<Game>;

pub fn game_supplier(mut rx: Receiver<SupplierCommand>) {
    tokio::spawn(async move {
        // initialize a creator and use it to create games on-demand
        let creator_res: Result<Creator, CreatorError> = Creator::new();
        if let Ok(mut creator) = creator_res{
            while let Some(cmd) = rx.recv().await {
                cmd.resp.send(creator.get_game())
                    .expect("Unable to send newly constructed game.");
            }
        }
    });
}