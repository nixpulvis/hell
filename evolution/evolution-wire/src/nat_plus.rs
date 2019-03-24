use std::ops::Deref;
use serde::{Serialize, Serializer, Deserialize, Deserializer, Error};
use super::*;

/// A natural number in the range [1, 7].
///
/// # Specification
///
/// This file was last updated from
/// [this link](http://www.ccs.neu.edu/home/matthias/4500-s16/8.html#%28tech._nat%2B%29)
/// on 04/12/16.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NatPlus(u64);

impl NatPlus {
    pub fn new(n: u64) -> Result<NatPlus, ()> {
        if n > 0 {
            Ok(NatPlus(*try!(Nat::new(n))))
        } else {
            Err(())
        }
    }
}

impl Deref for NatPlus {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToWire<NatPlus> for u64 {
    fn to_wire(&self) -> NatPlus {
        if let Ok(n) = NatPlus::new(*self) {
            n
        } else {
            panic!("attempted use of value greater than 7 or 0 as NatPlus");
        }
    }
}

impl Serialize for NatPlus {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        serializer.serialize_u64(self.0)
    }
}

impl Deserialize for NatPlus {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        let nat = try!(Nat::deserialize(deserializer));
        NatPlus::new(*nat).map_err(|_| Error::custom("nat plus is invalid"))
    }
}
