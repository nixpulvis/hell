use std::fmt::Debug;
use super::Observation;

/// A single choice for an interaction.
///
/// Choices hold the needed information to apply them to the state of the
/// game. For example a `FeedChoice` will tell the game what species to feed.
pub trait Choice: Clone + PartialEq + Debug {
    // fn validate(&Game) -> Result<(), ()>;
}

/// Asking for a `Choice`.
pub trait Choose<O: Observation, C: Choice>: Debug {
    fn choose(&mut self, &O) -> Result<Option<C>, ()>;
}

// Choice's themselves implement `Choose`.
impl<O: Observation, C: Choice> Choose<O, C> for C {
    fn choose(&mut self, _: &O) -> Result<Option<C>, ()> {
        Ok(Some(self.clone()))
    }
}

mod action_choice;
pub use self::action_choice::{ActionChoice, Growth, BoardTrade, TraitTrade};

mod feed_choice;
pub use self::feed_choice::FeedChoice;
