use evolution_wire as wire;
use interact::*;
use object::*;

impl wire::FromWire<wire::Player> for PlayerObservation {
    fn from_wire(wire: wire::Player) -> Result<Self, ()> {
        let mut species_vec = Vec::new();
        for s in wire.species.into_iter() {
            species_vec.push(try!(Species::from_wire(s)))
        }

        // TODO: valid player?

        Ok(PlayerObservation {
            id: *wire.id,
            domain: species_vec.into(),
            bag: *wire.bag,
        })
    }
}

impl wire::ToWire<wire::Player> for PlayerObservation {
    fn to_wire(&self) -> wire::Player {
        wire::Player {
            id: self.id.to_wire(),
            species: self.domain.to_wire(),
            bag: self.bag.to_wire(),
            cards: None,
        }
    }
}

impl wire::ToWire<wire::remote::Boards> for PlayerObservation {
    fn to_wire(&self) -> wire::remote::Boards {
        self.domain.to_wire()
    }
}

#[cfg(test)]
mod tests {
    use evolution_wire::{self as wire, ToWire, FromWire};
    use interact::*;
    use object::*;

    #[test]
    fn observed_to_wire() {
        let mut player = Player::new(1);
        player.domain_mut().add(Placement::Right);
        player.push_card(Card::mock(4, Trait::Fertile));

        assert_eq!(1, player.domain().len());
        assert_eq!(1, player.hand().len());

        let wire_player = ToWire::<wire::Player>::to_wire(&player.observe());

        assert_eq!(1, *wire_player.id);
        assert_eq!(1, wire_player.species.len());
        assert_eq!(None, wire_player.cards);
    }

    #[test]
    fn observed_from_wire() {
        let wire_player = wire::Player {
            id: wire::NaturalPlus::new(3).unwrap(),
            species: vec![wire::Species {
                food: wire::Nat::new(1).unwrap(),
                body: wire::Nat::new(1).unwrap(),
                population: wire::NatPlus::new(1).unwrap(),
                traits: wire::LOT::new(vec![]).unwrap(),
                fat_food: None,
            }],
            bag: wire::Natural(25),
            cards: Some(vec![
                wire::SpeciesCard(wire::FoodValue::new(4).unwrap(), wire::Trait::Carnivore),
            ]),
        };

        assert_eq!(3, *wire_player.id);
        assert_eq!(1, wire_player.species.len());
        assert_eq!(25, *wire_player.bag);
        assert!(wire_player.cards.is_some());

        let observed_player = PlayerObservation::from_wire(wire_player).unwrap();

        assert_eq!(3, observed_player.id);
        assert_eq!(1, observed_player.domain.len());
    }
}
