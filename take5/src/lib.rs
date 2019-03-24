//! 6 Nimmt! (in the US, Take 5!) is a simple card game.
//!
//! Typically 104 cards (1-104) and a "bull" value between 2 and 7 (inclusive). Playing the game
//! requires a and minimum of 2 players and is ideally played with no more than 10, but in
//! principle any reasonable number of players may participate.
//!
//! The game is played in rounds, each round consists of turns.
//!
//! At the beginning of each round, the dealer hands each player ten cards and creates four stacks
//! with one card each, face up.
//!
//! At the beginning of each turn, every player independently designates a card to be discarded.
//! When all players have picked a card, the cards are placed on the stacks. The player whose card
//! has the smallest face value goes first, followed by the player with the card with the second
//! smallest face value, and so on.
//!
//! For each card, the placement procedure is as follows:
//!
//! - If its face value is larger than any of the cards on top of one of the four stacks, it is
//! placed on top of that stack whose top-most card is closest to the case in face value.
//!
//! - If this stack consists of five cards, the player loses the sum of the "bull" points on the
//! cards of this stack. The stack is discarded and replaced by the new card.
//!
//! - If the card’s face value is smaller than all of the cards on top of the four stacks, the
//! player must pick up one of the stacks and start a new one with the card. Again, the player
//! loses the sum of the bull points on the cards of this stack.
//!
//! The round is over when all players have discard all their cards and placed them on the stacks
//! according to the above procedure.
//! If any player has gained some number of bull points (typically 66) at the end of the round,
//! the game is over; the player with the smallest number of bull points is the winner. Otherwise,
//! the players play another round.
//!
//! Background [Wikipedia](https://en.wikipedia.org/wiki/6_Nimmt!)
//!
//! # Usage
//!
//! Add the following to your `Cargo.toml` where the `...`s are a valid path
//! to this crate.
//!
//! ```toml
//! [dependencies.take5]
//! path = "..."
//! ```
#![deny(warnings)]
#![feature(custom_derive, plugin)]

extern crate rand;
extern crate itertools;
extern crate rustc_serialize;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub use game::Game;
pub use configuration::Configuration;
pub use card::Card;
pub use dealer::Dealer;
pub use board::{Board, StackId};
pub use player::Player;
pub use stack::Stack;

/// Player trait and implementations.
pub mod player;

/// Dealer trait and implementations.
pub mod dealer;

mod game;
mod configuration;
mod card;
mod board;
mod stack;
