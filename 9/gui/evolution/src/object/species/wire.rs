use std::collections::HashSet;
use evolution_wire as wire;
use object::*;

impl wire::FromWire<wire::Species> for Species {
    fn from_wire(wire: wire::Species) -> Result<Self, ()> {
        let mut traits = Vec::new();
        for t in wire.traits.iter() {
            traits.push(try!(Trait::from_wire(*t)))
        }

        let fat = match wire.fat_food {
            Some(n) => (0..*n).into_iter().map(|_| FoodToken).collect(),
            None => vec![],
        };

        let species = Species {
            population: *wire.population,
            body_size: *wire.body,
            traits: traits,
            food: (0..*wire.food).into_iter().map(|_| FoodToken).collect(),
            fat: fat,
        };

        // Check internal invariants.
        if species.population() <= MAX_POPULATION &&
           species.body_size() <= MAX_BODY_SIZE &&
           species.traits.len() <= MAX_TRAITS &&
           (species.food().len() as u64) <= species.population() &&
           species.traits().len() == species.traits().iter().collect::<HashSet<_>>().len() &&
           (species.fat().len() as u64) <= species.body_size()
        {
            Ok(species)
        } else {
            Err(())
        }
    }
}

impl wire::ToWire<wire::Species> for Species {
    fn to_wire(&self) -> wire::Species {
        let fat_food = if self.fat().is_empty() {
            None
        } else {
            Some((self.fat().len() as u64).to_wire())
        };

        wire::Species {
            food: (self.food().len() as u64).to_wire(),
            body: self.body_size().to_wire(),
            population: self.population().to_wire(),
            traits: self.traits.to_wire(),
            fat_food: fat_food,
        }
    }
}

#[cfg(test)]
mod tests {
    use evolution_wire::{self as wire, FromWire, ToWire};
    use object::*;

    #[test]
    fn to_wire() {
        let mut species = Species::default();
        let wire = species.to_wire();
        assert_eq!(0, *wire.food);
        assert_eq!(0, *wire.body);
        assert_eq!(1, *wire.population);
        assert_eq!(Vec::<wire::Trait>::new(), *wire.traits);
        assert_eq!(None, wire.fat_food);
        species.evolve(Trait::FatTissue).unwrap();
        let wire = species.to_wire();
        assert_eq!(vec![wire::Trait::FatTissue], *wire.traits);
        assert_eq!(None, wire.fat_food.map(|n| *n));
    }

    #[test]
    fn from_wire_without_fat_is_empty() {
        let wire = wire::Species {
            food: wire::Nat::new(1).unwrap(),
            body: wire::Nat::new(3).unwrap(),
            population: wire::NatPlus::new(2).unwrap(),
            traits: wire::LOT::new(vec![wire::Trait::Carnivore]).unwrap(),
            fat_food: None,
        };
        let species = Species::from_wire(wire).unwrap();
        assert!(species.fat().is_empty());
    }

    #[test]
    fn from_wire() {
        let wire = wire::Species {
           food: wire::Nat::new(1).unwrap(),
           body: wire::Nat::new(3).unwrap(),
           population: wire::NatPlus::new(2).unwrap(),
           traits: wire::LOT::new(vec![wire::Trait::FatTissue]).unwrap(),
           fat_food: Some(wire::Nat::new(3).unwrap()),
        };
        let species = Species::from_wire(wire).unwrap();
        assert_eq!(1, species.food().len());
        assert_eq!(3, species.body_size());
        assert_eq!(2, species.population());
        assert!(species.has_trait(Trait::FatTissue));
        assert_eq!(3, species.fat().len());
    }

    #[test]
    fn from_wire_invalid_food() {
        let wire = wire::Species {
            food: wire::Nat::new(6).unwrap(),
            body: wire::Nat::new(1).unwrap(),
            population: wire::NatPlus::new(4).unwrap(),
            traits: wire::LOT::new(vec![]).unwrap(),
            fat_food: None,
        };
        assert!(Species::from_wire(wire).is_err());
    }

    #[test]
    fn from_wire_invalid_traits() {
        let wire = wire::Species {
            food: wire::Nat::new(1).unwrap(),
            body: wire::Nat::new(1).unwrap(),
            population: wire::NatPlus::new(4).unwrap(),
            traits: wire::LOT::new(vec![
                wire::Trait::Carnivore,
                wire::Trait::Carnivore,
            ]).unwrap(),
            fat_food: None,
        };
        assert!(Species::from_wire(wire).is_err());
    }
}
