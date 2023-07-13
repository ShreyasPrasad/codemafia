# codemafia

## How to Play

codemafia is a take on the game of Codenames, in which teams compete to the guess the coloured words indicated by their "spymaster". In this variant, however, each team has an undercover operative that can see the other team's words as well as the black word. Their objective is to sabotage their own team's attempts to guess correctly.

## Architecture

The project consists of a front and back end, both of which are built using Rust. The backend leverages the Axum server framework and highly relies on Tokio tasks and message passing to orchestrate a real-time game. The frontend is built using the Yew framework, allowing for static WASM resources to be compiled and served.

### Backend

The backend high leverages the Tokio async runtime to maintain and coordinate active games. Each player's WebSocket connection is contained (in a single Tokio task)[https://github.com/ShreyasPrasad/codemafia/blob/b7c27c030d41ba486bb235ce9516f89868214584/codemafia/src/routes/game/game.rs#L65C23-L65C23]. These client tasks submit client messages materialized by the WebSocket connection to the game server (another Tokio task). Similarly, the client tasks relay events they receive from the game server downstream back to the player using the WebSocket.

#### Synchronization

Whenever possible, channels and message passing are used in place of conventional Mutex-based locking. In particular, (tokio::sync::mpsc)[https://docs.rs/tokio/latest/tokio/sync/mpsc/index.html] allows for a number of Tokio tasks to concurrently send messages to a single receiving task. The messages are then processed synchronously by the receiving task. When one-off messages need to be asynchronously returned the sender, (tokio::sync::oneshot)[https://docs.rs/tokio/latest/tokio/sync/oneshot/index.html] is frequently used. 

#### Game Creation

When a game is created, a unique 4-character code is generated and returned to the creator so they can distribute it to other players. When other players join using the room code, they are added to the list of players under the (Room structure)[https://github.com/ShreyasPrasad/codemafia/blob/main/codemafia/src/manager/room.rs]. 

#### Event Generation

Events are the messages sent from the server back to the client. The backend includes a robust event generation and dispatch system. Any task can submit an event with any number of recipients, (each specified by their game ID or in-game role)[https://github.com/ShreyasPrasad/codemafia/blob/b7c27c030d41ba486bb235ce9516f89868214584/codemafia/src/misc/events.rs#L17]. The (EventDispatcher)[https://github.com/ShreyasPrasad/codemafia/blob/b7c27c030d41ba486bb235ce9516f89868214584/codemafia/src/manager/dispatchers/mod.rs#L18] is responsible for accepting an event tagged with a recipient and contacting the necessary client tasks. The flexibility of this system can be observed in the (game code)[https://github.com/ShreyasPrasad/codemafia/blob/b7c27c030d41ba486bb235ce9516f89868214584/codemafia/src/game/board.rs#L54] that dispatches different board information to different players, based on their role. Ensuring a separation of concerns between game code and event dispatching makes it easy to keep the game code minimal and maintainable.

### Sharing Types

To support real-time communication between the frontend and backend, WebSockets are leveraged. Given that Rust is used across the stack, the opportunity to share WebSocket message structures exists. The (shared crate)[https://github.com/ShreyasPrasad/codemafia/tree/main/shared] contains these shared rust structures. The frontend is responsible for receiving/processes server event structures and sending client message structures, whereas the backend sends server events and receives/processes client messages. Using a shared crate leverages Rust's powerful type system for validating and working with socket messages. 

- All the supported shared messages are specified under the (message module)[https://github.com/ShreyasPrasad/codemafia/blob/main/shared/src/messages/mod.rs].
- All the supported shared events are specified under the (events module)[https://github.com/ShreyasPrasad/codemafia/blob/main/shared/src/events/mod.rs].


## Motivation

The big motivation behind this project was to gain more exposure to async programming in Rust. In particular, the project makes use of **tokio** primitives and uses the **axum** web framework to maintain websocket connections with clients. Hopefully, this project can serve as a resource for future devs looking to create a full-stack Rust project, with shared types between the front and back end.

Also, this project was an opportunity to delve deeper into WASM-based frontend apps, the supposed cryptonite to the JavaScript framework monopoly.
