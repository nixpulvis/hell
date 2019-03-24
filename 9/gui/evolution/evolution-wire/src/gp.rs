use serde::de::{SeqVisitor, Visitor, Error};
use serde::ser::impls::TupleVisitor3;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use super::Natural;

/// A request to trade a card for a population increase.
///
/// A `["population",i,j]` array requests a trade of card `j` for a growth of the
/// population of species board `i` by one.
///
/// # Specification
///
/// This file was last updated from
/// [this link](http://www.ccs.neu.edu/home/matthias/4500-s16/11.html#%28tech._gp%29)
/// on 04/12/16.
#[derive(Debug)]
pub struct GP {
    pub board_index: Natural,
    pub card_index: Natural,
}

impl Serialize for GP {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let tuple = ("population", self.board_index, self.card_index);
        serializer.serialize_tuple(TupleVisitor3::new(&tuple))
    }
}

impl Deserialize for GP {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        deserializer.deserialize(GPVisitor)
    }
}

struct GPVisitor;

impl Visitor for GPVisitor {
    type Value = GP;

    fn visit_seq<V>(&mut self, mut visitor: V) -> Result<Self::Value, V::Error>
        where V: SeqVisitor
    {
        let key = match try!(visitor.visit::<String>()) {
            Some(string) => string,
            None => {
                return Err(Error::custom("failed to parse key"));
            },
        };
        let second = try!(visitor.visit());
        let third = try!(visitor.visit());
        try!(visitor.end());
        match (&key[..], second, third) {
            ("population", Some(b), Some(c)) => {
                Ok(GP {
                    board_index: b,
                    card_index: c,
                })
            },
            _ => Err(Error::custom("invalid GP")),
        }
    }
}
