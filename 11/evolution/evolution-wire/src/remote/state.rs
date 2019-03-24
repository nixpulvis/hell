use serde::de::{Deserialize, Deserializer, SeqVisitor, Visitor, Error};
use serde::ser::{Serialize, Serializer};
use serde::ser::impls::TupleVisitor5;
use {Natural, NaturalPlus};
use remote::*;

/// A wire data type representing a given player's internal state knowledge as well as public
/// knowledge of the rest of the game. Provides enough information to generate a choice during
/// Feeding.
///
/// See [specification](http://www.ccs.neu.edu/home/matthias/4500-s16/r_remote.html#%28tech._state%29).
#[derive(Debug)]
pub struct State {
    pub bag: Natural,
    pub domain: Boards,
    pub hand: Cards,
    pub watering_hole: NaturalPlus,
    pub opponents: LOB,
}

impl Serialize for State {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let tuple = (self.bag, &self.domain, &self.hand, self.watering_hole, &self.opponents);
        serializer.serialize_tuple(TupleVisitor5::new(&tuple))
    }
}

impl Deserialize for State {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        deserializer.deserialize(StateVisitor)
    }
}

#[derive(Debug)]
struct StateVisitor;

impl Visitor for StateVisitor {
    type Value = State;

    fn visit_seq<V>(&mut self, mut visitor: V) -> Result<Self::Value, V::Error>
        where V: SeqVisitor
    {
        let bag = try!(visitor.visit());
        let domain = try!(visitor.visit());
        let hand = try!(visitor.visit());
        let watering_hole = try!(visitor.visit());
        let opponents = try!(visitor.visit());
        try!(visitor.end());
        match (bag, domain, hand, watering_hole, opponents) {
            (Some(bag),
             Some(domain),
             Some(hand),
             Some(watering_hole),
             Some(opponents)) => {
                 Ok(State {
                     bag: bag,
                     domain: domain,
                     hand: hand,
                     watering_hole: watering_hole,
                     opponents: opponents,
                 })
             },
             _ => Err(Error::custom("invalid state")),
        }
    }
}
