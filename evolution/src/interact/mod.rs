pub trait Chooser: Choose<ActionObservation, ActionChoice> + Choose<FeedObservation, FeedChoice> {
    fn start(&mut self, observation: &DealObservation);
    fn info(&self) -> Option<&str>;
}

// Re-export the public parts of this module.

/// Public to get around https://github.com/rust-lang/rust/pull/31920.
pub mod observe;
pub use self::observe::{
    Observe,
    Observation,
    PlayerObservation,
    BoardObservation,
    ActionObservation,
    FeedObservation,
    DealObservation,
};

/// Public to get around https://github.com/rust-lang/rust/pull/31920.
pub mod choose;
pub use self::choose::{
    Choose,
    Choice,
    ActionChoice,
    Growth,
    BoardTrade,
    TraitTrade,
    FeedChoice,
};

/// Public to get around https://github.com/rust-lang/rust/pull/31920.
pub mod choices;
pub use self::choices::Choices;
