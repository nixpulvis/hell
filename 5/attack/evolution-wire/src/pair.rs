use serde::de::{SeqVisitor, Visitor, Error};
use serde::{Deserialize, Deserializer};
use super::*;

/// A general pair type for **deserializing** arrays like `["key", T]`.
#[derive(Debug)]
pub enum Pair {
    // Species pairs.
    Food(Nat),
    Body(Nat),
    Population(NatPlus),
    Traits(LOT),
    FatFood(Nat),
    // Player pairs.
    Id(NaturalPlus),
    Species(LOS),
    Bag(Natural),
    // OwnedPlayer pairs.
    Cards(LOC),
}

impl Deserialize for Pair {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        deserializer.deserialize(PairVisitor)
    }
}

struct PairVisitor;

impl PairVisitor {
    fn visit_natural<V>(&mut self, mut visitor: V) -> Result<Natural, V::Error>
        where V: SeqVisitor
    {
        match try!(visitor.visit()) {
            Some(value) => {
                try!(visitor.end());
                Ok(value)
            },
            None => {
                Err(Error::custom("second element must be valid natural"))
            }
        }
    }

    fn visit_natural_plus<V>(&mut self, mut visitor: V) -> Result<NaturalPlus, V::Error>
        where V: SeqVisitor
    {
        match try!(visitor.visit()) {
            Some(value) => {
                try!(visitor.end());
                Ok(value)
            },
            None => {
                Err(Error::custom("second element must be valid natural"))
            }
        }
    }

    fn visit_nat<V>(&mut self, mut visitor: V) -> Result<Nat, V::Error>
        where V: SeqVisitor
    {
        match try!(visitor.visit()) {
            Some(value) => {
                try!(visitor.end());
                Ok(value)
            },
            None => {
                Err(Error::custom("second element must be valid nat"))
            }
        }
    }

    fn visit_nat_plus<V>(&mut self, mut visitor: V) -> Result<NatPlus, V::Error>
        where V: SeqVisitor
    {
        match try!(visitor.visit()) {
            Some(value) => {
                try!(visitor.end());
                Ok(value)
            },
            None => {
                Err(Error::custom("second element must be valid nat+"))
            }
        }
    }

    fn visit_loc<V>(&mut self, mut visitor: V) -> Result<LOC, V::Error>
        where V: SeqVisitor
    {
        match try!(visitor.visit()) {
            Some(cards) => {
                try!(visitor.end());
                Ok(cards)
            },
            None =>  {
                Err(Error::custom("second element must be valid LOC"))
            }
        }
    }

    fn visit_lot<V>(&mut self, mut visitor: V) -> Result<LOT, V::Error>
        where V: SeqVisitor
    {
        match try!(visitor.visit()) {
            Some(traits) => {
                try!(visitor.end());
                Ok(traits)
            },
            None => {
                Err(Error::custom("second element must be valid LOT"))
            }
        }
    }

    fn visit_los<V>(&mut self, mut visitor: V) -> Result<LOS, V::Error>
        where V: SeqVisitor
    {
        match try!(visitor.visit::<LOS>()) {
            Some(species) => {
                try!(visitor.end());
                Ok(species)
            },
            None => {
                Err(Error::custom("second element must be valid LOS"))
            }
        }
    }
}

impl Visitor for PairVisitor {
    type Value = Pair;

    fn visit_seq<V>(&mut self, mut visitor: V) -> Result<Self::Value, V::Error> where V: SeqVisitor {
        let key = match try!(visitor.visit::<String>()) {
            Some(s) => s,
            None => {
                return Err(Error::custom("failed to parse key"));
            },
        };
        match &key[..] {
            "food" => {
                Ok(Pair::Food(try!(self.visit_nat(visitor))))
            },
            "body" => {
                Ok(Pair::Body(try!(self.visit_nat(visitor))))
            },
            "population" => {
                Ok(Pair::Population(try!(self.visit_nat_plus(visitor))))
            },
            "traits" => {
                Ok(Pair::Traits(try!(self.visit_lot(visitor))))
            },
            "fat-food" => {
                Ok(Pair::FatFood(try!(self.visit_nat(visitor))))
            },
            "id" => {
                Ok(Pair::Id(try!(self.visit_natural_plus(visitor))))
            },
            "species" => {
                Ok(Pair::Species(try!(self.visit_los(visitor))))
            },
            "bag" => {
                Ok(Pair::Bag(try!(self.visit_natural(visitor))))
            },
            "cards" => {
                Ok(Pair::Cards(try!(self.visit_loc(visitor))))
            },
            _ => Err(Error::custom("invalid key in species pair")),
        }
    }
}
