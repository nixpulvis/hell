use serde::de::{SeqVisitor, Visitor, Error};
use serde::ser::impls::TupleVisitor2;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use Natural;

#[derive(Debug)]
pub struct GP {
    pub board_index: Natural,
    pub card_index: Natural,
}

impl Serialize for GP {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let tuple = (self.board_index, self.card_index);
        serializer.serialize_tuple(TupleVisitor2::new(&tuple))
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
        let first = try!(visitor.visit());
        let second = try!(visitor.visit());
        try!(visitor.end());

        match (first, second) {
            (Some(b), Some(c)) => {
                Ok(GP {
                    board_index: b,
                    card_index: c,
                })
            },
            _ => Err(Error::custom("invalid GP")),
        }
    }
}
