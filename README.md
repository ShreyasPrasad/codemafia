# codemafia

## How to Play

codemafia is a take on the game of Codenames, in which teams compete to the guess the coloured words indicated by their "spymaster". In this variant, however, each team has an undercover operative that can see the other team's words as well as the black word. Their objective is to sabotage their own team's attempts to guess correctly.

## Architecture

### Backend

The backend architecture relies heavily on Tokio tasks and channel-based communication to allow for game room, chat and the server to be orchestrated. Each player's connection is maintained in a single Tokio task, and it uses a tokio mpsc channel to relay events (such as player activity, chat activity, and game activity) to a dedicated game room task and game server task. Using a central receiving task allows for synchronization to be achieved through a total ordering of the events that are received, allow for game consistency to be achieved.

## Motivation

The big motivation behind this project was to gain more exposure to async programming in Rust. In particular, the project makes use of **tokio** primitives and uses the **axum** web framework to maintain websocket connections with clients.

Hopefully, this project can serve as a resource for future devs looking to create a full-stack Rust project, with shared types between the front and back end.
