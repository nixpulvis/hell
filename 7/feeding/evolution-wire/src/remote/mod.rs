/// The player’s complete current state and the public state of the
/// competitors. Specifically, [bg,bd,c,w,others] consists of the player’s
/// current bag, the current species boards, the current cards, the number of
/// available food tokens, and the species boards of all other players
/// (in turn order).
// State is [Natural, [Species+, ..., Species+], Cards, Natural+, LOB].
// mod state;

pub type LOB = Vec<Boards>;

pub type Boards = Vec<super::Species>;

pub type Species = super::Species;

pub type LOT = super::LOT;

pub type Cards = super::LOC;

pub type FeedingChoice = super::FeedChoice;

mod action4;
pub use self::action4::Action4;

mod choose;
pub use self::choose::Choose;

mod gp;
pub use self::gp::GP;

mod gb;
pub use self::gb::GB;

mod start;
pub use self::start::Start;

mod state;
pub use self::state::State;
