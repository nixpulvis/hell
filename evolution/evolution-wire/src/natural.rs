use std::ops::Deref;
use serde::{Serialize, Serializer, Deserialize, Deserializer, Error};
use super::*;

/// Any integer >= 0.
///
/// # Specification
///
/// This file was last updated from
/// [this link](http://www.ccs.neu.edu/home/matthias/4500-s16/6.html#%28tech._natural%29)
/// on 04/12/16.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Natural(pub u64);

impl Deref for Natural {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToWire<Natural> for u64 {
    fn to_wire(&self) -> Natural {
        Natural(*self)
    }
}

impl ToWire<Natural> for usize {
    fn to_wire(&self) -> Natural {
        Natural(*self as u64)
    }
}

impl Serialize for Natural {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        serializer.serialize_u64(self.0)
    }
}

impl Deserialize for Natural {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        let i = *(try!(Integer::deserialize(deserializer)));
        if i < 0
        {
            Err(Error::custom("natural cannot be negative"))
        } else {
            Ok(Natural(i as u64))
        }
    }
}
