# codemafia

## How to Play

codemafia is a take on the game of Codenames, in which teams compete to the guess the coloured words indicated by their "spymaster". In this variant, however, each team has an undercover operative that can see the other team's words as well as the black word. Their objective is to sabotage their own team's attempts to guess correctly.

## Architecture

The big motivation behind this project was to gain more exposure to async programming in Rust. In particular, the project makes use of **tokio** primitives and uses the **axum** web framework to maintain websocket connections with clients.
