use evolution_wire as wire;
use object::*;

impl wire::FromWire<wire::remote::Start> for Player {
    fn from_wire(wire: wire::remote::Start) -> Result<Player, ()> {
        let domain = try!(Domain::from_wire(wire.domain));
        let bag = (0..*wire.bag).map(|_| FoodToken).collect();
        let mut hand = Vec::new();
        for card in wire.hand {
            hand.push(try!(Card::from_wire(card)));
        }
        Ok(Player{
            id: 1,
            domain: domain,
            bag: bag,
            hand: hand,
        })
    }
}

impl wire::FromWire<wire::Player> for Player {
    fn from_wire(wire: wire::Player) -> Result<Player, ()> {
        let wire::Player { id, species, bag, cards } = wire;
        let mut species_vec = Vec::new();
        for s in species.into_iter() {
            species_vec.push(try!(Species::from_wire(s)))
        }
        let mut card_vec = Vec::new();
        if let Some(cards) = cards {
            for c in cards.into_iter() {
                card_vec.push(try!(Card::from_wire(c)))
            }
        }

        // TODO: valid player?

        Ok(Player {
            id: *id,
            domain: species_vec.into(),
            bag: (0..*bag).into_iter().map(|_| FoodToken).collect(),
            hand: card_vec,
        })
    }
}

impl wire::ToWire<wire::Player> for Player {
    fn to_wire(&self) -> wire::Player {
        if self.hand.is_empty() {
            wire::Player {
                id: self.id().to_wire(),
                species: self.domain().to_wire(),
                bag: (self.bag().len() as u64).to_wire(),
                cards: None,
            }
        } else {
            wire::Player {
                id: self.id.to_wire(),
                species: self.domain().to_wire(),
                bag: (self.bag().len() as u64).to_wire(),
                cards: Some(self.hand().to_wire()),
            }
        }
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
    use object::*;

    #[test]
    fn to_wire_player_without_cards() {
        let player = Player::new(2);

        let wire_player: wire::Player = player.to_wire();

        assert_eq!(2, *wire_player.id);
        assert_eq!(0, wire_player.species.len());
        assert_eq!(wire::Natural(0), wire_player.bag);
        assert_eq!(None, wire_player.cards);
    }

    #[test]
    fn to_wire_player_with_cards() {
        let mut player = Player::new(4);
        player.push_card(Card::mock(3, Trait::Burrowing));

        let wire_player: wire::Player = player.to_wire();

        assert_eq!(4, *wire_player.id);
        assert_eq!(0, wire_player.species.len());
        assert_eq!(wire::Natural(0), wire_player.bag);
        assert_eq!(1, wire_player.cards.unwrap().len());
    }

    #[test]
    fn from_wire_player_without_cards() {
        let wire_player = wire::Player {
            id: wire::NaturalPlus::new(1).unwrap(),
            species: vec![wire::Species {
                food: wire::Nat::new(1).unwrap(),
                body: wire::Nat::new(1).unwrap(),
                population: wire::NatPlus::new(1).unwrap(),
                traits: wire::LOT::new(vec![wire::Trait::FatTissue]).unwrap(),
                fat_food: Some(wire::Nat::new(1).unwrap()),
            }],
            bag: wire::Natural(0),
            cards: None,
        };

        let player = Player::from_wire(wire_player).unwrap();

        assert_eq!(1, player.id());
        assert_eq!(1, player.domain().len());
        assert_eq!(1, player.domain()[0].population());
    }

    #[test]
    fn from_wire_player_with_cards() {
        let wire_player = wire::Player {
            id: wire::NaturalPlus::new(1).unwrap(),
            species: vec![wire::Species {
                food: wire::Nat::new(1).unwrap(),
                body: wire::Nat::new(1).unwrap(),
                population: wire::NatPlus::new(1).unwrap(),
                traits: wire::LOT::new(vec![wire::Trait::FatTissue]).unwrap(),
                fat_food: Some(wire::Nat::new(1).unwrap()),
            }],
            bag: wire::Natural(0),
            cards: Some(vec![
                wire::SpeciesCard(wire::FoodValue::new(2).unwrap(),
                                  wire::Trait::Carnivore),
            ]),
        };

        let player = Player::from_wire(wire_player).unwrap();

        assert_eq!(1, player.id());
        assert_eq!(1, player.domain().len());
        assert_eq!(1, player.domain()[0].population());
        assert_eq!(0, player.bag().len());
        assert_eq!(Trait::Carnivore, player.hand()[0].trait_type());
    }

    #[test]
    fn from_wire_start_without_cards() {
        let wire = wire::remote::Start {
            watering_hole: Natural(0),
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
            hand: vec![],
        };

        assert_eq!(3, *wire.bag);
        assert_eq!(1, wire.domain.len());
        assert_eq!(0, wire.hand.len());

        let player = Player::from_wire(wire).unwrap();

        assert_eq!(3, player.bag().len());
        assert_eq!(1, player.domain().len());
        assert_eq!(0, player.hand().len());
    }

    #[test]
    fn from_wire_start_with_cards() {
        let wire = wire::remote::Start {
            watering_hole: Natural(0),
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

        assert_eq!(3, *wire.bag);
        assert_eq!(1, wire.domain.len());
        assert_eq!(4, wire.hand.len());

        let player = Player::from_wire(wire).unwrap();

        assert_eq!(3, player.bag().len());
        assert_eq!(1, player.domain().len());
        assert_eq!(4, player.hand().len());
    }
}
