use serde::ser::{Serialize, Serializer};
use serde::ser::impls::{TupleVisitor3, TupleVisitor4};
use serde::de::{Deserialize, Deserializer, Error, SeqVisitor, Visitor};
use super::*;
use super::pair::Pair;

/// A list of wire player pluses.
///
/// # Specification
///
/// This file was last updated from
/// [this link](http://www.ccs.neu.edu/home/matthias/4500-s16/8.html#%28tech._lop%2B%29)
/// on 04/12/16.
pub type LOP = Vec<Player>;

/// A player possibly with owned data.
///
/// # Specification
///
/// This file was last updated from
/// [this link](http://www.ccs.neu.edu/home/matthias/4500-s16/8.html#%28tech._player%2B%29)
/// on 04/12/16.
#[derive(Debug, Eq, PartialEq)]
pub struct Player {
    pub id: NaturalPlus,
    pub species: LOS,
    pub bag: Natural,
    pub cards: Option<LOC>,
}

impl Serialize for Player {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let id = ("id", self.id);
        let species = ("species", &self.species);
        let bag = ("bag", self.bag);
        if let Some(ref cards) = self.cards {
            let tuple = (id, species, bag, ("cards", cards));
            serializer.serialize_tuple(TupleVisitor4::new(&tuple))
        } else {
            let tuple = (id, species, bag);
            serializer.serialize_tuple(TupleVisitor3::new(&tuple))
        }
    }
}

impl Deserialize for Player {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        deserializer.deserialize_seq(PlayerPlusVisitor::default())
    }
}

#[derive(Debug, Default)]
pub struct PlayerPlusVisitor;

impl Visitor for PlayerPlusVisitor {
    type Value = Player;

    fn visit_seq<V>(&mut self, mut visitor: V) -> Result<Self::Value, V::Error>
        where V: SeqVisitor
    {
        let id = try!(visitor.visit());
        let species = try!(visitor.visit());
        let bag = try!(visitor.visit());
        let cards = try!(visitor.visit());
        try!(visitor.end());

        match (id, species, bag, cards) {
            (Some(Pair::Id(id)),
             Some(Pair::Species(species)),
             Some(Pair::Bag(bag)),
             Some(Pair::Cards(cards))) => {
                let cards = if cards.is_empty() {
                    None
                } else {
                    Some(cards)
                };

                Ok(Player {
                    id: id,
                    species: species,
                    bag: bag,
                    cards: cards,
                })
            },
            (Some(Pair::Id(id)),
             Some(Pair::Species(species)),
             Some(Pair::Bag(bag)),
             None) => {
                Ok(Player {
                    id: id,
                    species: species,
                    bag: bag,
                    cards: None,
                })
            },
            _ => Err(Error::custom("invalid player plus")),
        }
    }
}
