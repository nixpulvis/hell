use serde::de::Deserialize;
use serde_json::{self as json, Value};

/// A wire type for containing one of two variants, without knowing the variant ahead of time.
#[derive(Debug, PartialEq, Eq)]
pub enum Either<T, U> {
    Left(T),
    Right(U),
}

impl<T, U> Either<T, U>
    where T: Deserialize,
          U: Deserialize,
{
    /// Attempts to produce an `Either<T, U>` from a `serde_json::Value`, wrapping the converted
    /// value in a `Result`
    ///
    /// # Examples
    ///
    // TODO: Fix this namespace/import issue so this can run as a test
    /// ```ignore
    /// extern crate serde_json;
    /// extern crate evolution_wire;
    ///
    /// use serde_json::{self as json, Value};
    /// use evolution_wire::{Nat, Either, Trait};
    ///
    /// let value = json::from_str("carnivore").unwrap();
    /// let either = Either::<Nat, Trait>::from_value(value).unwrap();
    ///
    /// assert!(either.is_right());
    /// match either {
    ///     Either::Right(trait_type) => assert_eq!(Trait::Carnivore, trait_type),
    ///     _ => panic!("match failed"),
    /// }
    /// ```
    ///
    /// # Notes
    ///
    /// The conversion fails if either of the following conditions are true:
    ///
    /// 1. The `Either` was constructed with ambiguous types (such as two integral types)
    /// 2. The given `Value` failed to convert into one of the `Either` types
    pub fn from_value(value: Value) -> Result<Either<T, U>, ()> {
        let maybe_t = json::from_value(value.clone());
        let maybe_u = json::from_value(value);

        match (maybe_t, maybe_u) {
            (Ok(_), Ok(_)) => Err(()),
            (Ok(t), Err(_)) => Ok(Either::Left(t)),
            (Err(_), Ok(u)) => Ok(Either::Right(u)),
            _ => Err(()),
        }
    }

    pub fn is_left(&self) -> bool {
        match *self {
            Either::Left(_) => true,
            _ => false,
        }
    }

    pub fn is_right(&self) -> bool {
        !self.is_left()
    }
}
