use serde::de::{Deserialize, Deserializer, Error, SeqVisitor, Visitor};
use serde::ser::{Serialize, Serializer};
use serde::ser::impls::TupleVisitor3;
use super::*;

/// The information passed to a player for making a feed choice.
///
/// The current player should have private information, and the opponents
/// should not.
///
/// # Specification
///
/// This file was last updated from
/// [this link](http://www.ccs.neu.edu/home/matthias/4500-s16/6.html#%28tech._feeding%29)
/// on 04/12/16.
#[derive(Debug)]
pub struct Feeding {
    /// The player who's turn it is to feed.
    pub current_player: Player,
    /// Tokens of food that are left at the watering hole.
    pub watering_hole: NaturalPlus,
    /// The other players of the game, not including the feeding player.
    pub opponents: LOP,
}

impl Serialize for Feeding {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let tuple = (&self.current_player, &self.watering_hole, &self.opponents);
        serializer.serialize_tuple(TupleVisitor3::new(&tuple))
    }
}

impl Deserialize for Feeding {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        deserializer.deserialize(FeedingVisitor)
    }
}

#[derive(Debug)]
pub struct FeedingVisitor;

impl Visitor for FeedingVisitor {
    type Value = Feeding;

    fn visit_seq<V>(&mut self, mut visitor: V) -> Result<Self::Value, V::Error>
        where V: SeqVisitor
    {
        let current_player = try!(visitor.visit());
        let watering_hole = try!(visitor.visit());
        let opponents = try!(visitor.visit());
        try!(visitor.end());

        match (current_player, watering_hole, opponents) {
            (Some(current_player),
             Some(watering_hole),
             Some(opponents)) =>
            {
                Ok(Feeding {
                    current_player: current_player,
                    watering_hole: watering_hole,
                    opponents: opponents,
                })
            },
            _ => Err(Error::custom("invalid feeding")),
        }
    }
}
