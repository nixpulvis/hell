use evolution_wire as wire;
use interact::*;
use object::*;

impl wire::ToWire<wire::remote::Start> for DealObservation {
    fn to_wire(&self) -> wire::remote::Start {
        wire::remote::Start {
            watering_hole: self.board.to_wire(),
            bag: self.player.bag.to_wire(),
            domain: self.player.domain.to_wire(),
            hand: self.hand.iter().map(|card| card.to_wire()).collect(),
        }
    }
}

impl wire::FromWire<wire::remote::Start> for DealObservation {
    fn from_wire(wire: wire::remote::Start) -> Result<Self, ()> {
        let board = try!(BoardObservation::from_wire(wire.watering_hole));
        let player = try!(Player::from_wire(wire));
        Ok(DealObservation {
            board: board,
            player: player.observe(),
            hand: player.hand().clone().to_vec(),
        })
    }
}

#[cfg(test)]
mod tests {
    use evolution_wire::{
        self as wire,
        FromWire,
        ToWire,
        FoodValue,
        LOT,
        Natural,
        Nat,
        NatPlus,
        SpeciesCard,
        Trait as WireTrait,
    };
    use interact::*;
    use object::*;

    #[test]
    fn to_wire() {
        let mut board = Board::default();
        board.push_foods(vec![FoodToken, FoodToken, FoodToken]);
        let mut player = Player::new(123);
        player.push_cards(vec![
            Card::mock(1, Trait::Ambush),
            Card::mock(2, Trait::Burrowing),
            Card::mock(3, Trait::Climbing),
            Card::mock(4, Trait::Carnivore),
        ]);
        player.domain_mut().add(Placement::Right);

        assert_eq!(3, board.food().len());
        assert_eq!(0, player.bag().len());
        assert_eq!(1, player.domain().len());
        assert_eq!(4, player.hand().len());

        let observation = (board, player).observe();
        let wire = observation.to_wire();

        assert_eq!(3, *wire.watering_hole);
        assert_eq!(0, *wire.bag);
        assert_eq!(1, wire.domain.len());
        assert_eq!(4, wire.hand.len());
    }

    #[test]
    fn from_wire() {
        let wire = wire::remote::Start {
            watering_hole: Natural(25),
            bag: Natural(3),
            domain: vec![
                wire::Species {
                    food: Nat::new(0).unwrap(),
                    body: Nat::new(0).unwrap(),
                    population: NatPlus::new(1).unwrap(),
                    traits: LOT::new(vec![]).unwrap(),
                    fat_food: None,
                }
            ],
            hand: vec![
                SpeciesCard(FoodValue::new(3).unwrap(), WireTrait::Ambush),
                SpeciesCard(FoodValue::new(2).unwrap(), WireTrait::Burrowing),
                SpeciesCard(FoodValue::new(1).unwrap(), WireTrait::Climbing),
                SpeciesCard(FoodValue::new(0).unwrap(), WireTrait::Carnivore),
            ],
        };

        assert_eq!(25, *wire.watering_hole);
        assert_eq!(3, *wire.bag);
        assert_eq!(1, wire.domain.len());
        assert_eq!(4, wire.hand.len());

        let observation = DealObservation::from_wire(wire).unwrap();

        assert_eq!(25, observation.board.food);
        assert_eq!(3, observation.player.bag);
        assert_eq!(1, observation.player.domain.len());
        assert_eq!(4, observation.hand.len());
    }
}
