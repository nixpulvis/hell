use std::ops::Deref;
use serde::{Serialize, Serializer, Deserialize, Deserializer, Error};
use super::*;

/// A natural number in the range [0, 7].
///
/// # Specification
///
/// This file was last updated from
/// [this link](http://www.ccs.neu.edu/home/matthias/4500-s16/5.html#%28tech._nat%29)
/// on 04/12/16.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Nat(u64);

impl Nat {
    pub fn new(n: u64) -> Result<Nat, ()> {
        if n > 7 {
            Err(())
        } else {
            Ok(Nat(n))
        }
    }
}

impl Deref for Nat {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToWire<Nat> for u64 {
    fn to_wire(&self) -> Nat {
        if let Ok(n) = Nat::new(*self) {
            n
        } else {
            panic!("attempted use of value greater than 7 as Nat");
        }
    }
}

impl Serialize for Nat {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        serializer.serialize_u64(self.0)
    }
}

impl Deserialize for Nat {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        let number = try!(Natural::deserialize(deserializer));
        Nat::new(number.0).map_err(|_| Error::custom("value is greater than 7"))
    }
}
