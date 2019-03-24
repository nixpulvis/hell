use std::ops::Deref;
use serde::{Serialize, Serializer, Deserialize, Deserializer, Error};
use super::*;

/// An integer in the range [-8, 8].
///
/// # Specification
///
/// This file was last updated from
/// [this link](http://www.ccs.neu.edu/home/matthias/4500-s16/8.html#%28tech._foodvalue%29)
/// on 04/12/16.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FoodValue(i64);

impl FoodValue {
    pub fn new(n: i64) -> Result<FoodValue, ()> {
        if n < -8 || n > 8 {
            Err(())
        } else {
            Ok(FoodValue(n))
        }
    }
}

impl Deref for FoodValue {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToWire<FoodValue> for i64 {
    fn to_wire(&self) -> FoodValue {
        if let Ok(n) = FoodValue::new(*self) {
            n
        } else {
            panic!("attempted to create a food value out-of-range");
        }
    }
}

impl Serialize for FoodValue {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        serializer.serialize_i64(self.0)
    }
}

impl Deserialize for FoodValue {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        let i = try!(Integer::deserialize(deserializer));
        FoodValue::new(*i).map_err(|_| Error::custom("could not convert Integer to FoodValue"))
    }
}
