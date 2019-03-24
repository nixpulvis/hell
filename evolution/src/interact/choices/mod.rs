use interact::*;

/// The knowledge of all possible `Choice`s for the **current player** in
/// the game.
pub trait Choices<C: Choice> {
    fn choices(&self) -> Vec<C>;
}

mod action_observation;
mod feed_observation;
