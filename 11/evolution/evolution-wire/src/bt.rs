use std::ops::Deref;
use serde::{Serialize, Serializer, Deserialize, Deserializer, Error};
use serde::ser::impls::SeqIteratorVisitor;
use super::{ToWire, Natural};

const MIN_LENGTH: usize = 1;
const MAX_LENGTH: usize = 4;

/// A trading of a card for a new board.
///
/// The new board will be placed to the *right* of the existing sequence of
/// boards. Specifically using `i` as the card to trade for the new board, and
/// `j...k` as the optional cards to play for traits in `[i, j, ..., k]`.
///
/// # Specification
///
/// This file was last updated from
/// [this link](http://www.ccs.neu.edu/home/matthias/4500-s16/8.html#%28tech._configuration%29)
/// on 03/29/16.
#[derive(Debug)]
pub struct BT(pub Vec<Natural>);

impl BT {
    pub fn new(naturals: Vec<Natural>) -> Result<BT, ()> {
        if naturals.len() >= MIN_LENGTH && naturals.len() <= MAX_LENGTH {
            Ok(BT(naturals))
        } else {
            Err(())
        }
    }
}

impl Deref for BT {
    type Target = Vec<Natural>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Clone + ToWire<Natural>> ToWire<BT> for Vec<T> {
    fn to_wire(&self) -> BT {
        let wire_vec = self.iter().map(|n| n.clone().to_wire()).collect();
        BT::new(wire_vec).expect("given invalid BT")
    }
}

impl Serialize for BT {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        serializer.serialize_seq(SeqIteratorVisitor::new(
            self.0.iter(),
            Some(self.0.len()),
        ))
    }
}

impl Deserialize for BT {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        let in_vec = try!(Vec::deserialize(deserializer));
        let out_bt = BT::new(in_vec);
        match out_bt {
            Ok(bt) => Ok(bt),
            Err(_) => Err(Error::custom("invalid list to form a BT")),
        }
    }
}
