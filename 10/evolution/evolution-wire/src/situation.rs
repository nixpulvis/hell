use serde::de::{SeqVisitor, Visitor};
use serde::{Deserialize, Deserializer, Error};
use super::*;
use super::species::MaybeSpecies;

/// An attacking situation between 2 to 4 species.
///
/// # Specification
///
/// This file was last updated from
/// [this link](http://www.ccs.neu.edu/home/matthias/4500-s16/5.html#%28tech._situation%29)
/// on 04/12/16.
#[derive(Debug, PartialEq, Eq)]
pub struct Situation {
    pub attacker: Species,
    pub defender: Species,
    pub left: Option<Species>,
    pub right: Option<Species>,
}

impl Deserialize for Situation {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        deserializer.deserialize(SituationSeqVisitor)
    }
}

pub struct SituationSeqVisitor;

impl Visitor for SituationSeqVisitor {
    type Value = Situation;

    fn visit_seq<V>(&mut self, mut visitor: V) -> Result<Self::Value, V::Error>
        where V: SeqVisitor
    {
        let defender = try!(visitor.visit());
        let attacker = try!(visitor.visit());
        let left = try!(visitor.visit::<MaybeSpecies>());
        let right = try!(visitor.visit::<MaybeSpecies>());
        try!(visitor.end());

        match (defender, attacker, left, right) {
            (Some(defender),
             Some(attacker),
             Some(left),
             Some(right)) =>
            {
                Ok(Situation {
                    defender: defender,
                    attacker: attacker,
                    left: left.0,
                    right: right.0,
                })
            },
            _ => Err(Error::custom("invalid situation")),
        }
    }
}
