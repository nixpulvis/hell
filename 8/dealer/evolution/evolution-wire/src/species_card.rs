use serde::{Error, Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{SeqVisitor, Visitor};
use serde::ser::impls::TupleVisitor2;
use super::*;

/// A list of species cards.
///
/// # Specification
///
/// This file was last updated from
/// [this link](http://www.ccs.neu.edu/home/matthias/4500-s16/8.html#%28tech._loc%29)
/// on 04/12/16.
pub type LOC = Vec<SpeciesCard>;

/// A species card, with a food value, and a trait.
///
/// # Specification
///
/// This file was last updated from
/// [this link](http://www.ccs.neu.edu/home/matthias/4500-s16/8.html#%28tech._speciescard%29)
/// on 04/12/16.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SpeciesCard(pub FoodValue, pub Trait);

impl Deserialize for SpeciesCard {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        deserializer.deserialize(SpeciesCardVisitor)
    }
}

impl Serialize for SpeciesCard {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let SpeciesCard(ref food_value, ref trait_type) = *self;
        let tuple = (food_value, trait_type);
        serializer.serialize_tuple(TupleVisitor2::new(&tuple))
    }
}

struct SpeciesCardVisitor;

impl Visitor for SpeciesCardVisitor {
    type Value = SpeciesCard;

    fn visit_seq<V>(&mut self, mut visitor: V) -> Result<Self::Value, V::Error>
        where V: SeqVisitor
    {
        let food_value = try!(visitor.visit::<FoodValue>());
        let trait_type = try!(visitor.visit::<Trait>());
        try!(visitor.end());
        match (food_value, trait_type) {
            (Some(food_value),
             Some(trait_type)) => {
                Ok(SpeciesCard(food_value, trait_type))
            },
            _ => {
                Err(Error::custom("invalid species card"))
            }
        }
    }
}
