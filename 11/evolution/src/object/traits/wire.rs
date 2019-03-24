use evolution_wire as wire;
use object::*;

impl wire::FromWire<wire::Trait> for Trait {
    fn from_wire(wire: wire::Trait) -> Result<Self, ()> {
        Ok(match wire {
            wire::Trait::Carnivore   => Trait::Carnivore,
            wire::Trait::Ambush      => Trait::Ambush,
            wire::Trait::Burrowing   => Trait::Burrowing,
            wire::Trait::Climbing    => Trait::Climbing,
            wire::Trait::Cooperation => Trait::Cooperation,
            wire::Trait::FatTissue   => Trait::FatTissue,
            wire::Trait::Fertile     => Trait::Fertile,
            wire::Trait::Foraging    => Trait::Foraging,
            wire::Trait::HardShell   => Trait::HardShell,
            wire::Trait::Herding     => Trait::Herding,
            wire::Trait::Horns       => Trait::Horns,
            wire::Trait::LongNeck    => Trait::LongNeck,
            wire::Trait::PackHunting => Trait::PackHunting,
            wire::Trait::Scavenger   => Trait::Scavenger,
            wire::Trait::Symbiosis   => Trait::Symbiosis,
            wire::Trait::WarningCall => Trait::WarningCall,
        })
    }
}

impl wire::ToWire<wire::Trait> for Trait {
    fn to_wire(&self) -> wire::Trait {
        match *self {
            Trait::Carnivore   => wire::Trait::Carnivore,
            Trait::Ambush      => wire::Trait::Ambush,
            Trait::Burrowing   => wire::Trait::Burrowing,
            Trait::Climbing    => wire::Trait::Climbing,
            Trait::Cooperation => wire::Trait::Cooperation,
            Trait::FatTissue   => wire::Trait::FatTissue,
            Trait::Fertile     => wire::Trait::Fertile,
            Trait::Foraging    => wire::Trait::Foraging,
            Trait::HardShell   => wire::Trait::HardShell,
            Trait::Herding     => wire::Trait::Herding,
            Trait::Horns       => wire::Trait::Horns,
            Trait::LongNeck    => wire::Trait::LongNeck,
            Trait::PackHunting => wire::Trait::PackHunting,
            Trait::Scavenger   => wire::Trait::Scavenger,
            Trait::Symbiosis   => wire::Trait::Symbiosis,
            Trait::WarningCall => wire::Trait::WarningCall,
        }
    }
}

#[cfg(test)]
mod tests {
    use evolution_wire::{self as wire, FromWire, ToWire};
    use object::*;

    #[test]
    fn test_to_wire() {
        let trait_type = Trait::Horns;
        assert_eq!(wire::Trait::Horns, trait_type.to_wire());
    }

    #[test]
    fn test_from_wire() {
        let wire = wire::Trait::Carnivore;
        assert_eq!(Trait::Carnivore, FromWire::from_wire(wire).unwrap());
    }
}
