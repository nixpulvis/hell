//! Wire data types for the evolution game.
#![deny(warnings)]

#[macro_use]
extern crate log;
extern crate serde;
extern crate serde_json;

/// A trait for getting meaningful data from wire data.
///
/// This is used to seperate the invariants of a wire protocol, and
/// application data formats. Here the wire type `W` can be much looser
/// than the type this trait is implemented for. In the cases where the
/// data from the wire is invalid the `from_wire` function will return
/// an `Err`.
pub trait FromWire<W: serde::Deserialize>: Sized {
    fn from_wire(wire: W) -> Result<Self, ()>;
}

impl<T: FromWire<U>, U: serde::Deserialize> FromWire<Vec<U>> for Vec<T> {
    fn from_wire(wire: Vec<U>) -> Result<Self, ()> {
        let mut vec = Vec::new();
        for element in wire.into_iter() {
            vec.push(try!(T::from_wire(element)));
        }
        Ok(vec)
    }
}

/// A trait for getting wire data from meaningful data.
///
/// Here the meaningful data is being converted into data for the wire,
/// which should never fail. If the internal data set is larger than that
/// of the wire, then there is a problem with the design of the wire data
/// format that requires rethinking things.
pub trait ToWire<W: serde::Serialize> {
    fn to_wire(&self) -> W;
}

impl<'a, T: ToWire<U>, U: serde::Serialize> ToWire<Vec<U>> for &'a [T] {
    fn to_wire(&self) -> Vec<U> {
        self.iter().map(|t| t.to_wire()).collect()
    }
}

#[cfg(test)]
#[macro_use]
macro_rules! assert_json {
    ($expected:expr, $actual:expr) => {
        use itertools::Itertools;
        assert_eq!($expected.split_whitespace().join(""),
                   $actual.split_whitespace().join(""));
    };
}

/// TODO: <doc>
pub mod channel;
pub use self::channel::Channel;

/// Remote protocol wire datatypes.
///
///
/// # Specification
///
/// These datatypes were last updated from
/// [this link](http://www.ccs.neu.edu/home/matthias/4500-s16/r_remote.html)
/// on 04/15/16.
pub mod remote;

mod action4;
pub use self::action4::{Action4, Step4};

mod bt;
pub use self::bt::BT;

mod choice;
pub use self::choice::Choice;

mod configuration;
pub use self::configuration::Configuration;

mod either;
pub use self::either::Either;

mod feed_choice;
pub use self::feed_choice::FeedChoice;

mod feeding;
pub use self::feeding::Feeding;

mod food_value;
pub use self::food_value::FoodValue;

mod gb;
pub use self::gb::GB;

mod gp;
pub use self::gp::GP;

mod integer;
pub use self::integer::Integer;

mod nat_plus;
pub use self::nat_plus::NatPlus;

mod nat;
pub use self::nat::Nat;

mod natural_plus;
pub use self::natural_plus::NaturalPlus;

mod natural;
pub use self::natural::Natural;

mod player;
pub use self::player::{LOP, Player};

mod rt;
pub use self::rt::RT;

mod situation;
pub use self::situation::Situation;

mod species_card;
pub use self::species_card::{LOC, SpeciesCard};

mod species;
pub use self::species::{LOS, Species};

mod start_round;
pub use self::start_round::StartRound;

mod traits;
pub use self::traits::{LOT, Trait};

mod pair;
