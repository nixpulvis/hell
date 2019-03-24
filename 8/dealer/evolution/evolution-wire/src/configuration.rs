use serde::ser::{Serialize, Serializer};
use serde::ser::impls::{TupleVisitor3};
use serde::de::{Deserialize, Deserializer, Error, SeqVisitor, Visitor};
use super::*;

/// A game state configuration.
///
/// # Specification
///
/// This file was last updated from
/// [this link](http://www.ccs.neu.edu/home/matthias/4500-s16/8.html#%28tech._configuration%29)
/// on 03/29/16.
#[derive(Debug)]
pub struct Configuration {
    pub players: LOP,
    pub watering_hole: Natural,
    pub deck: LOC,
}

impl Serialize for Configuration {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let tuple = (&self.players, &self.watering_hole, &self.deck);
        serializer.serialize_tuple(TupleVisitor3::new(&tuple))
    }
}

impl Deserialize for Configuration {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        deserializer.deserialize_seq(ConfigurationVisitor)
    }
}

#[derive(Debug)]
struct ConfigurationVisitor;

impl Visitor for ConfigurationVisitor {
    type Value = Configuration;

    fn visit_seq<V>(&mut self, mut visitor: V) -> Result<Self::Value, V::Error>
        where V: SeqVisitor
    {
        let players = try!(visitor.visit());
        let watering_hole = try!(visitor.visit());
        let deck = try!(visitor.visit());
        try!(visitor.end());

        match (players, watering_hole, deck) {
            (Some(players),
             Some(watering_hole),
             Some(deck)) => {
                Ok(Configuration {
                    players: players,
                    watering_hole: watering_hole,
                    deck: deck,
                })
            },
            _ => Err(Error::custom("invalid configuration")),
        }
    }
}
