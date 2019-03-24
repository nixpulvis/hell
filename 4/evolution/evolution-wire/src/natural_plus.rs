use std::ops::Deref;
use serde::{Serialize, Serializer, Deserialize, Deserializer, Error};
use super::*;

/// Any integer > 0.
///
/// # Specification
///
/// This file was last updated from
/// [this link](http://www.ccs.neu.edu/home/matthias/4500-s16/6.html#%28tech._natural%2B%29)
/// on 04/12/16.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NaturalPlus(u64);

impl NaturalPlus {
    pub fn new(n: u64) -> Result<NaturalPlus, ()> {
        if n == 0 {
            Err(())
        } else {
            Ok(NaturalPlus(n))
        }
    }
}

impl Deref for NaturalPlus {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToWire<NaturalPlus> for u64 {
    fn to_wire(&self) -> NaturalPlus {
        if let Ok(n) = NaturalPlus::new(*self) {
            n
        } else {
            panic!("attempted use of 0 as NaturalPlus");
        }

    }
}

impl Serialize for NaturalPlus {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        serializer.serialize_u64(self.0)
    }
}

impl Deserialize for NaturalPlus {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        let number = try!(Natural::deserialize(deserializer));
        NaturalPlus::new(*number).map_err(|_| Error::custom("nat plus is equal to 0"))
    }
}
