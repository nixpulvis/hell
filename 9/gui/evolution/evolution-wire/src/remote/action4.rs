use serde::{Serialize, Serializer, Deserialize, Deserializer, Error};
use serde::ser::impls::TupleVisitor5;
use serde::de::{SeqVisitor, Visitor};
use {Natural, BT, RT};
use super::{GP, GB};

#[derive(Debug)]
pub struct Action4(pub Natural, pub Vec<GP>, pub Vec<GB>, pub Vec<BT>, pub Vec<RT>);

impl Serialize for Action4 {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let tuple = (&self.0, &self.1, &self.2, &self.3, &self.4);
        serializer.serialize_tuple(TupleVisitor5::new(&tuple))
    }
}

impl Deserialize for Action4 {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        deserializer.deserialize_seq(Action4Visitor)
    }
}

#[derive(Debug)]
struct Action4Visitor;

impl Visitor for Action4Visitor {
    type Value = Action4;

    fn visit_seq<V>(&mut self, mut visitor: V) -> Result<Self::Value, V::Error>
        where V: SeqVisitor
    {
        let natural = try!(visitor.visit());
        let gp_vec = try!(visitor.visit());
        let gb_vec = try!(visitor.visit());
        let bt_vec = try!(visitor.visit());
        let rt_vec = try!(visitor.visit());
        try!(visitor.end());

        match (natural, gp_vec, gb_vec, bt_vec, rt_vec) {
            (Some(natural), Some(gp_vec), Some(gb_vec), Some(bt_vec), Some(rt_vec)) => {
                Ok(Action4(natural, gp_vec, gb_vec, bt_vec, rt_vec))
            },
            _ => Err(Error::custom("invalid action4")),
        }
    }
}
