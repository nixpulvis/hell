use serde::{Error, Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{SeqVisitor, Visitor};
use serde::ser::impls::TupleVisitor3;
use super::Natural;

/// The replacement of a sigle trait on a species.
///
/// > Specifically, `[b, i, j]` specifies that board `b`’s `i`-th trait card is replaced with the
/// `j`-th card from the player’s card sequence.
///
/// # Specification
///
/// This file was last updated from
/// [this link](http://www.ccs.neu.edu/home/matthias/4500-s16/11.html#%28tech._rt%29)
/// on 04/12/16.
#[derive(Debug)]
pub struct RT(pub Natural, pub Natural, pub Natural);

impl Deserialize for RT {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        deserializer.deserialize(RTVisitor)
    }
}

impl Serialize for RT {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let RT(ref board_index, ref trait_index, ref hand_index) = *self;
        let tuple = (board_index, trait_index, hand_index);
        serializer.serialize_tuple(TupleVisitor3::new(&tuple))
    }
}

struct RTVisitor;

impl Visitor for RTVisitor {
    type Value = RT;

    fn visit_seq<V>(&mut self, mut visitor: V) -> Result<Self::Value, V::Error>
        where V: SeqVisitor
    {
        let board_index = try!(visitor.visit::<Natural>());
        let trait_index = try!(visitor.visit::<Natural>());
        let hand_index = try!(visitor.visit::<Natural>());
        try!(visitor.end());
        match (board_index, trait_index, hand_index){
            (Some(b), Some(i), Some(j)) => {
                Ok(RT(b, i, j))
            },
            _ => {
                Err(Error::custom("invalid RT"))
            }
        }
    }
}
