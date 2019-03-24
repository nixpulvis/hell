//! Other implementations of players and dealers for the game of take5.
//!
//! # Usage
//!
//! Add the following to your `Cargo.toml` where the `...`s are a valid path
//! to this crate.
//!
//! ```toml
//! [dependencies.take5_ext]
//! path = "..."
//! ```
#![deny(warnings)]

extern crate rand;
extern crate take5;

pub use self::custom_dealer::CustomDealer;
pub use self::stdin_player::StdinPlayer;

mod custom_dealer;
mod stdin_player;
