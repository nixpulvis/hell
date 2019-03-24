use std::ops::Deref;
use serde::{Serialize, Serializer, Deserialize, Deserializer, Error};
use serde_json::value::Value;
use super::*;

/// An integer.
///
/// This should ideally work for all sized integers, but currently does not.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Integer(pub i64);

impl Deref for Integer {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToWire<Integer> for i64 {
    fn to_wire(&self) -> Integer {
        Integer(*self)
    }
}

impl Serialize for Integer {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        serializer.serialize_i64(self.0)
    }
}


impl Deserialize for Integer {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        fn is_within_threshold(float: f64, threshold: f64) -> bool {
            (float - float.round()).abs() <= threshold
        }

        match try!(Deserialize::deserialize(deserializer)) {
            Value::I64(i) => {
                Ok(Integer(i))
            },
            Value::U64(u) => {
                Ok(Integer(u as i64))
            },
            Value::F64(f) if is_within_threshold(f, 0.000001) => {
                Ok(Integer(f as i64))
            },
            _ => {
                Err(Error::custom("value cannot be interpreted as an integer"))
            },
        }
    }
}
