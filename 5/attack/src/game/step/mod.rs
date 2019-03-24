use std::ops::{Deref, DerefMut};
use game::Game;
use interact::*;

/// Represents some mutable effect on a `Game`.
pub trait Step<C: Chooser>: Deref<Target=Game<C>> + DerefMut {
    /// Take one step, mutating the game for future steps.
    fn step(&mut self) -> Result<(), ()>;
}

mod deal;
pub use self::deal::Deal;

mod action;
pub use self::action::Action;

mod reveal;
pub use self::reveal::Reveal;

mod feed;
pub use self::feed::Feed;

mod bag;
pub use self::bag::Bag;
