//! An Evolution (game) competition library.
//#![deny(warnings)]

#[macro_use]
extern crate log;
extern crate rand;
extern crate serde;
extern crate serde_json;
#[cfg(feature = "wire")]
extern crate evolution_wire;

/// A helper that compiles into an evaluation of an expression, but also logs the expression and
/// evaluated result to the debug logs before returning.
#[macro_export]
macro_rules! debug_expr {
    ($expr:expr) => {{
        let value = $expr;
        debug!("`{}` = {:?}", stringify!($expr), value);
        value
    }};
}

/// Extension methods provided by traits for built-in types.
#[macro_use]
pub mod ext;

/// Objects of the game, like a card or a species.
///
/// These objects contain **complete** knowledge, this means that they
/// represent the actual object in the game, not for example just the public
/// information.
pub mod object;

/// Data and methods for required interactions for the game.
///
/// An interaction is when a part of the system needs to share some information
/// and possibly get a response. For example the dealer will ask a player for
/// it's choice for feeding each turn.
///
/// # Examples
///
/// Making a choice for feeding.
///
/// ```rust
/// use evolution::Game;
/// use evolution::object::{FoodToken, Placement};
/// use evolution::silly::Silly;
/// use evolution::interact::{Observe, Choose, FeedChoice};
///
/// let mut game = Game::<Silly>::new(3).unwrap();
/// game.board_mut().push_food(FoodToken);
/// game.players_mut()[0].domain_mut().add(Placement::Right);
///
/// if let Ok(Some(choice)) = Silly.choose(&game.observe()) {
///     assert_eq!(FeedChoice::Feed(0), choice);
/// } else {
///     panic!("must have gotten a choice");
/// }
/// ```
pub mod interact;

/// A *very* silly chooser.
pub mod silly;

/// Types and functions required for playing a game of Evolution.
// TODO: Module shouldn't be public.
pub mod game;
pub use self::game::Game;

// /// Control flow for the game.
// pub mod machine;

/// A helper that compiles into a test harness for portions of the wire data types.
#[macro_export]
macro_rules! test_harness {
    ($internal_pat:pat = $wire_type:ty => $internal_data:ident, $body:block) => {
        extern crate log;
        extern crate evolution_logger;

        #[allow(unused_imports)]
        fn main() {
            use std::io;
            use serde_json as json;
            use log::LogLevelFilter;
            use evolution_logger::*;
            use evolution_wire::{ToWire, FromWire};
            use evolution::game::*;

            Logger::init().expect("logger failed to start");

            let mut reader = io::stdin();
            if let Ok(wire) = json::from_reader::<_, $wire_type>(&mut reader) {
                if let Ok($internal_pat) = $internal_data::from_wire(wire) {
                    $body
                } else {
                    panic!("invalid game object");
                }
            } else {
                panic!("invalid wire data");
            }
        }
    };
}
