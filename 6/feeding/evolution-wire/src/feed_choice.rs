use serde::ser::{Serialize, Serializer};
use serde::ser::impls::{TupleVisitor2, TupleVisitor3};
use serde::de::{SeqVisitor, Visitor, Error};
use serde::{Deserialize, Deserializer};
use super::*;

/// A wire feed choice response message.
///
/// # Specification
///
/// This file was last updated from
/// [this link](http://www.ccs.neu.edu/home/matthias/4500-s16/6.html)
/// on 04/12/16.
#[derive(Debug, PartialEq, Eq)]
pub enum FeedChoice {
    Abstain,
    Feed(Natural),
    Store(Natural, NatPlus),
    Attack(Natural, Natural, Natural),
}

impl Serialize for FeedChoice {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        match *self {
            FeedChoice::Abstain => {
                serializer.serialize_bool(false)
            },
            FeedChoice::Feed(eater) => {
                serializer.serialize_u64(*eater)
            },
            FeedChoice::Store(eater, amount) => {
                let tuple = (eater, amount);
                serializer.serialize_tuple(TupleVisitor2::new(&tuple))
            },
            FeedChoice::Attack(attacker, target, defender) => {
                let tuple = (attacker, target, defender);
                serializer.serialize_tuple(TupleVisitor3::new(&tuple))
            },
        }
    }
}

impl Deserialize for FeedChoice {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        deserializer.deserialize(FeedChoiceVisitor)
    }
}

struct FeedChoiceVisitor;


impl Visitor for FeedChoiceVisitor {
    type Value = FeedChoice;

    fn visit_bool<E>(&mut self, v: bool) -> Result<Self::Value, E> where E: Error {
        if !v {
            Ok(FeedChoice::Abstain)
        } else {
            Err(Error::custom("invalid feed choice (true)"))
        }
    }

    fn visit_u64<E>(&mut self, v: u64) -> Result<Self::Value, E> where E: Error {
        Ok(FeedChoice::Feed(Natural(v)))
    }

    fn visit_seq<V>(&mut self, mut visitor: V) -> Result<Self::Value, V::Error> where V: SeqVisitor {
        let first = try!(visitor.visit::<Natural>());
        let second = try!(visitor.visit::<Natural>());
        let third = try!(visitor.visit::<Natural>());
        try!(visitor.end());

        match (first, second, third) {
            (Some(f), Some(s), None) => {
                let s = match NatPlus::new(*s) {
                    Ok(n) => n,
                    Err(_) => return Err(Error::custom("invalid feed choice (store)"))
                };
                Ok(FeedChoice::Store(f, s))
            }
            (Some(f), Some(s), Some(t)) => {
                Ok(FeedChoice::Attack(f, s, t))
            }
            _ => Err(Error::custom("invalid feed choice"))
        }
    }
}
