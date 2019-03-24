use serde::de::{Deserialize, Deserializer, SeqVisitor, Visitor, Error};
use serde::ser::{Serialize, Serializer};
use serde::ser::impls::TupleVisitor4;
use Natural;
use remote::*;

/// A wire data type used to signify the start of a new round.
///
/// See [specification, Figure 7](http://www.ccs.neu.edu/home/matthias/4500-s16/r_remote.html)
#[derive(Debug, Clone)]
pub struct Start {
    pub watering_hole: Natural,
    pub bag: Natural,
    pub domain: Boards,
    pub hand: Cards,
}

impl Serialize for Start {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let tuple = (&self.watering_hole, &self.bag, &self.domain, &self.hand);
        serializer.serialize_tuple(TupleVisitor4::new(&tuple))
    }
}

impl Deserialize for Start {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        deserializer.deserialize_seq(StartVisitor)
    }
}

#[derive(Debug)]
struct StartVisitor;

impl Visitor for StartVisitor {
    type Value = Start;

    fn visit_seq<V>(&mut self, mut visitor: V) -> Result<Self::Value, V::Error>
        where V: SeqVisitor
    {
        let watering_hole = try!(visitor.visit());
        let bag = try!(visitor.visit());
        let domain = try!(visitor.visit());
        let hand = try!(visitor.visit());
        try!(visitor.end());

        match (watering_hole, bag, domain, hand) {
            (Some(watering_hole), Some(bag), Some(domain), Some(hand)) => {
                Ok(Start {
                    watering_hole: watering_hole,
                    bag: bag,
                    domain: domain,
                    hand: hand,
                })
            },
            _ => Err(Error::custom("invalid start")),
        }
    }
}
