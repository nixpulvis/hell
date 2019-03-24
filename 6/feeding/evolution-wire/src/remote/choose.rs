use serde::de::{Deserialize, Deserializer, SeqVisitor, Visitor, Error};
use serde::ser::{Serialize, Serializer};
use serde::ser::impls::TupleVisitor2;
use remote::LOB;

/// A data type representing a request for a player choice during the Action step. The two lists of
/// boards represent the players before the current, and after the current, respectively, in turn
/// order.
///
/// See [specification, Figure 8.](http://www.ccs.neu.edu/home/matthias/4500-s16/r_remote.html)
pub struct Choose {
    pub before: LOB,
    pub after: LOB,
}

impl Serialize for Choose {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let tuple = (&self.before, &self.after);
        serializer.serialize_tuple(TupleVisitor2::new(&tuple))
    }
}

impl Deserialize for Choose {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        deserializer.deserialize_seq(ChooseVisitor)
    }
}

#[derive(Debug)]
struct ChooseVisitor;

impl Visitor for ChooseVisitor {
    type Value = Choose;

    fn visit_seq<V>(&mut self, mut visitor: V) -> Result<Self::Value, V::Error>
        where V: SeqVisitor
    {
        let before = try!(visitor.visit());
        let after = try!(visitor.visit());
        try!(visitor.end());
        match (before, after) {
            (Some(before), Some(after)) => {
                Ok(Choose {
                    before: before,
                    after: after,
                })
            },
            _ => Err(Error::custom("invalid choose")),
        }
    }
}
