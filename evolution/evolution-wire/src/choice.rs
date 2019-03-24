use serde::ser::{Serialize, Serializer};
use serde::ser::impls::TupleVisitor3;
use serde::de::{Deserialize, Deserializer, Error, SeqVisitor, Visitor};
use super::*;

/// The data a player gets to make a choice for the action step.
///
/// # Ordering
///
/// ```raw
/// configuration.players: [id(1), id(2), id(3), id(4)]
///                                ~~~~~ current_player
///
/// choice.current_player: id(2)
/// choice.before: [id(1).species]
/// choice.after: [id(3).species, id(4).species]
/// ```
///
/// # Specification
///
/// This file was last updated from
/// [this link](http://www.ccs.neu.edu/home/matthias/4500-s16/12.html#%28tech._choice%29)
/// on 04/07/16.
#[derive(Debug)]
pub struct Choice {
    /// The current player of an action step.
    pub current_player: Player,
    /// The species of the players before the `current_player` in turn order.
    pub before: Vec<LOS>,
    /// The species of the players after the `current_player` in turn order.
    pub after: Vec<LOS>,
}

impl Serialize for Choice {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let tuple = (&self.current_player, &self.before, &self.after);
        serializer.serialize_tuple(TupleVisitor3::new(&tuple))
    }
}

impl Deserialize for Choice {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        deserializer.deserialize_seq(ChoiceVisitor)
    }
}

struct ChoiceVisitor;

impl Visitor for ChoiceVisitor {
    type Value = Choice;

    fn visit_seq<V>(&mut self, mut visitor: V) -> Result<Self::Value, V::Error>
        where V: SeqVisitor
    {
        let current_player = try!(visitor.visit());
        let before = try!(visitor.visit());
        let after = try!(visitor.visit());
        try!(visitor.end());

        match (current_player, before, after) {
            (Some(current_player),
             Some(before),
             Some(after)) => {
                Ok(Choice {
                    current_player: current_player,
                    before: before,
                    after: after,
                })
            },
            _ => Err(Error::custom("invalid choice")),
        }
    }
}
