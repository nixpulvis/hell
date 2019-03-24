/// A structure enforcing single ownership over all food tokens in the game world.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoodToken;

/// A placement describes where players can place a new species in
/// relation to their other species.
#[derive(Debug, Copy, Clone)]
pub enum Placement {
    /// Indicates a placement of a new species on the far **left** of their
    /// existing species.
    Left,
    /// Indicates a placement of a new species on the far **right** of their
    /// existing species.
    Right,
}

/// The maximum population a species can have.
pub const MAX_POPULATION: u64 = 7;

/// The maximum body size a species can have.
pub const MAX_BODY_SIZE: u64 = 7;

/// The maximum number of traits a species may have.
pub const MAX_TRAITS: usize = 3;

/// The effective addeded body size the hard shell trait adds, for
/// deflecting attacks.
pub const HARD_SHELL_PROTECTION: u64 = 4;

/// The number of carnivore trait cards in a deck.
pub const NUM_CARNIVORE_CARDS: usize = 17;

/// The number of each non-carnivore cards in a deck.
pub const NUM_VEGITARIAN_CARDS: usize = 7;

/// The number of cards awarded to a player for each extinct species.
pub const CARDS_PER_EXTINCTION: usize = 2;

// Re-export the public parts of this module.

mod board;
pub use self::board::Board;

mod card;
pub use self::card::Card;

mod domain;
pub use self::domain::Domain;

mod player;
pub use self::player::Player;

mod species;
pub use self::species::Species;

mod traits;
pub use self::traits::Trait;
