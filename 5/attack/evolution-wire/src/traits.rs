use std::ops::Deref;
use serde::{Serialize, Serializer, Deserialize, Deserializer, Error};
use serde::ser::impls::SeqIteratorVisitor;
use super::*;

/// A trait.
///
/// # Specification
///
/// This file was last updated from
/// [this link](http://www.ccs.neu.edu/home/matthias/4500-s16/5.html#%28tech._trait%29)
/// on 04/12/16.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Trait {
    Carnivore,
    Ambush,
    Burrowing,
    Climbing,
    Cooperation,
    FatTissue,
    Fertile,
    Foraging,
    HardShell,
    Herding,
    Horns,
    LongNeck,
    PackHunting,
    Scavenger,
    Symbiosis,
    WarningCall,
}

impl Serialize for Trait {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        match *self {
            Trait::Carnivore   => serializer.serialize_str("carnivore"),
            Trait::Ambush      => serializer.serialize_str("ambush"),
            Trait::Burrowing   => serializer.serialize_str("burrowing"),
            Trait::Climbing    => serializer.serialize_str("climbing"),
            Trait::Cooperation => serializer.serialize_str("cooperation"),
            Trait::FatTissue   => serializer.serialize_str("fat-tissue"),
            Trait::Fertile     => serializer.serialize_str("fertile"),
            Trait::Foraging    => serializer.serialize_str("foraging"),
            Trait::HardShell   => serializer.serialize_str("hard-shell"),
            Trait::Herding     => serializer.serialize_str("herding"),
            Trait::Horns       => serializer.serialize_str("horns"),
            Trait::LongNeck    => serializer.serialize_str("long-neck"),
            Trait::PackHunting => serializer.serialize_str("pack-hunting"),
            Trait::Scavenger   => serializer.serialize_str("scavenger"),
            Trait::Symbiosis   => serializer.serialize_str("symbiosis"),
            Trait::WarningCall => serializer.serialize_str("warning-call"),
        }
    }
}

impl Deserialize for Trait {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: Deserializer {
        match &try!(String::deserialize(deserializer))[..] {
            "carnivore"    => Ok(Trait::Carnivore),
            "ambush"       => Ok(Trait::Ambush),
            "burrowing"    => Ok(Trait::Burrowing),
            "climbing"     => Ok(Trait::Climbing),
            "cooperation"  => Ok(Trait::Cooperation),
            "fat-tissue"   => Ok(Trait::FatTissue),
            "fertile"      => Ok(Trait::Fertile),
            "foraging"     => Ok(Trait::Foraging),
            "hard-shell"   => Ok(Trait::HardShell),
            "herding"      => Ok(Trait::Herding),
            "horns"        => Ok(Trait::Horns),
            "long-neck"    => Ok(Trait::LongNeck),
            "pack-hunting" => Ok(Trait::PackHunting),
            "scavenger"    => Ok(Trait::Scavenger),
            "symbiosis"    => Ok(Trait::Symbiosis),
            "warning-call" => Ok(Trait::WarningCall),
            _ => return Err(Error::custom("invalid trait name"))
        }
    }
}

/// A list traits, which must be at most 3 in length.
///
/// If you are calling `LOT` yourself to make a list of traits, it is your
/// responsibility to ensure there are never more than 3 traits in the list.
///
/// # Specification
///
/// This file was last updated from
/// [this link](http://www.ccs.neu.edu/home/matthias/4500-s16/5.html#%28tech._lot%29)
/// on 04/12/16.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct LOT(Vec<Trait>);

impl LOT {
    pub fn new(traits: Vec<Trait>) -> Result<LOT, ()> {
        if traits.len() <= 3 {
            Ok(LOT(traits))
        } else {
            Err(())
        }
    }
}

impl Deref for LOT {
    type Target = Vec<Trait>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Clone + ToWire<Trait>> ToWire<LOT> for Vec<T> {
    fn to_wire(&self) -> LOT {
        let vec = self.iter().map(|t| t.clone().to_wire()).collect();
        LOT::new(vec).expect("given invalid LOT.")
    }
}

impl Serialize for LOT {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        serializer.serialize_seq(SeqIteratorVisitor::new(
            self.0.iter(),
            Some(self.0.len()),
        ))
    }
}

impl Deserialize for LOT {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        let vec = try!(Vec::deserialize(deserializer));
        if vec.len() > 3 {
            Err(Error::custom("list of traits length exceeded 3"))
        } else {
            Ok(LOT(vec))
        }
    }
}
