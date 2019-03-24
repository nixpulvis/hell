use std::ops::Deref;
use serde::ser::impls::{TupleVisitor4, TupleVisitor5};
use serde::de::{SeqVisitor, Visitor, Error};
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use super::*;
use super::pair::Pair;

/// A list of species.
///
/// # Specification
///
/// This file was last updated from
/// [this link](http://www.ccs.neu.edu/home/matthias/4500-s16/6.html#%28tech._lo%29)
/// on 04/12/16.
pub type LOS = Vec<Species>;

/// A species with or without fat food.
///
/// This single data type represents a `Species+` in the specification, as
/// it's more sensible to represent this union as a single type with an
/// optional `"fat-food"` value.
///
/// If the `fat_food` field is `Some` then there **must** be the trait
/// fat tissue in the field `traits`.
///
/// # Specification
///
/// This file was last updated from
/// [this link](http://www.ccs.neu.edu/home/matthias/4500-s16/6.html#%28tech._species%2B%29)
/// on 04/12/16.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Species {
    pub food: Nat,
    pub body: Nat,
    pub population: NatPlus,
    pub traits: LOT,
    pub fat_food: Option<Nat>,
}

impl Serialize for Species {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let food = ("food", self.food);
        let body = ("body", self.body);
        let population = ("population", self.population);
        let traits = ("traits", &self.traits);

        let fat_food = match self.fat_food {
            Some(f) if *f > 0 => Some(f),
            _ => None,
        };

        if let Some(fat_food) = fat_food {
            let fat_food = ("fat-food", fat_food);
            let tuple = (food, body, population, traits, fat_food);
            serializer.serialize_tuple(TupleVisitor5::new(&tuple))
        } else {
            let tuple = (food, body, population, traits);
            serializer.serialize_tuple(TupleVisitor4::new(&tuple))
        }
    }
}

impl Deserialize for Species {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        deserializer.deserialize(SpeciesVisitor)
    }
}

#[derive(Default)]
struct SpeciesVisitor;

impl Visitor for SpeciesVisitor {
    type Value = Species;

    fn visit_seq<V>(&mut self, mut visitor: V) -> Result<Self::Value, V::Error>
        where V: SeqVisitor
    {
        let food = try!(visitor.visit());
        let body = try!(visitor.visit());
        let population = try!(visitor.visit());
        let traits = try!(visitor.visit());

        match (food, body, population, traits) {
            (Some(Pair::Food(food)),
             Some(Pair::Body(body)),
             Some(Pair::Population(population)),
             Some(Pair::Traits(traits))) =>
            {
                let fat_food = match try!(visitor.visit()) {
                    Some(Pair::FatFood(f)) => Some(f),
                    Some(_) => return Err(Error::custom("invalid species")),
                    None => None,
                };
                try!(visitor.end());

                let fat_food = if traits.contains(&Trait::FatTissue) {
                    match fat_food {
                        s @ Some(_) => s,
                        None => Some(Nat::new(0).unwrap()),
                    }
                } else {
                    None
                };

                Ok(Species {
                    population: population,
                    body: body,
                    traits: traits,
                    food: food,
                    fat_food: fat_food,
                })
            }
            _ => Err(Error::custom("invalid species"))
        }
    }
}

/// A wire species or nothing.
///
/// This is needed as opposed to simply an optional type because the spec
/// requires that the `None` case is represented as `false` for some reason.
pub struct MaybeSpecies(pub Option<Species>);

impl Deref for MaybeSpecies {
    type Target = Option<Species>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deserialize for MaybeSpecies {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        deserializer.deserialize(MaybeSpeciesVisitor)
    }
}

struct MaybeSpeciesVisitor;

impl Visitor for MaybeSpeciesVisitor {
    type Value = MaybeSpecies;

    fn visit_bool<E>(&mut self, v: bool) -> Result<Self::Value, E> where E: Error {
        if !v {
            Ok(MaybeSpecies(None))
        } else {
            Err(Error::custom("Can only use false for MaybeSpecies"))
        }
    }

    fn visit_seq<V>(&mut self, visitor: V) -> Result<Self::Value, V::Error>
        where V: SeqVisitor
    {
        Ok(MaybeSpecies(Some(try!(SpeciesVisitor.visit_seq(visitor)))))
    }
}
