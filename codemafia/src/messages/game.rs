/* Defines a game message and its different actions. */

#[derive(Debug)]
pub struct GameMessage {
    action: GameMessageAction,
}

#[derive(Debug)]
pub enum GameMessageAction {
    WordSuggested(String /* The word that was suggested */),
    WordClicked(u8 /* The index of the word that was clicked */),
    SpyMasterVoteInitiated,
    EndTurn,
}

#[derive(Debug)]
pub enum Team {
    Blue,
    Red
}