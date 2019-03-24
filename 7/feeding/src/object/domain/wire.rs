use evolution_wire as wire;
use object::*;

impl wire::ToWire<wire::LOS> for Domain {
    fn to_wire(&self) -> wire::LOS {
        self.0.as_slice().to_wire()
    }
}

impl wire::FromWire<wire::LOS> for Domain {
    fn from_wire(wire: wire::LOS) -> Result<Self, ()> {
        Ok(try!(Vec::from_wire(wire)).into())
    }
}

#[cfg(test)]
mod tests {
    use evolution_wire::{self as wire, ToWire, FromWire};
    use object::*;

    #[test]
    fn to_wire() {
        let mut species = Species::default();
        species.breed().unwrap();
        let domain = Domain::from(vec![species]);

        let wire = domain.to_wire();

        assert_eq!(2, *wire[0].population);
    }

    #[test]
    fn from_wire() {
        let wire = vec![
            wire::Species {
                food: wire::Nat::new(2).unwrap(),
                body: wire::Nat::new(4).unwrap(),
                population: wire::NatPlus::new(3).unwrap(),
                traits: wire::LOT::new(vec![wire::Trait::FatTissue]).unwrap(),
                fat_food: Some(wire::Nat::new(1).unwrap()),
            }
        ];

        let domain = Domain::from_wire(wire).unwrap();

        assert_eq!(1, domain.len());
        assert_eq!(2, domain[0].food().len());
        assert_eq!(3, domain[0].population());
        assert_eq!(1, domain[0].traits().len());
        assert!(domain[0].has_trait(Trait::FatTissue));
        assert_eq!(1, domain[0].fat().len());
    }
}
