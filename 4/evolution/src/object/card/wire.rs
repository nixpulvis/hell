use evolution_wire as wire;
use object::*;

impl wire::ToWire<wire::SpeciesCard> for Card {
    fn to_wire(&self) -> wire::SpeciesCard {
        wire::SpeciesCard(self.food_value().to_wire(), self.trait_type().to_wire())
    }
}

impl wire::FromWire<wire::SpeciesCard> for Card {
    fn from_wire(wire: wire::SpeciesCard) -> Result<Card, ()> {
        let food_value = *wire.0;
        let trait_type = try!(Trait::from_wire(wire.1));
        let bounds = match trait_type {
            Trait::Carnivore => (-8, 8),
            _ => (-3, 3),
        };
        if food_value >= bounds.0 && food_value <= bounds.1 {
            Ok(Card(food_value, trait_type))
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use evolution_wire::{self as wire, FromWire, ToWire};
    use object::*;

    #[test]
    fn from_wire() {
        let wire_card = wire::SpeciesCard(
            wire::FoodValue::new(2).unwrap(),
            wire::Trait::Burrowing,
        );
        let card_from_wire = Card::from_wire(wire_card).unwrap();

        assert_eq!(2, card_from_wire.food_value());
        assert_eq!(Trait::Burrowing, card_from_wire.trait_type());
    }

    #[test]
    fn from_wire_invalid() {
        let wire_card = wire::SpeciesCard(
            wire::FoodValue::new(8).unwrap(),
            wire::Trait::LongNeck,
        );
        let card_from_wire = Card::from_wire(wire_card);

        assert!(card_from_wire.is_err());
    }

    #[test]
    fn to_wire() {
        let card = Card::mock(-3, Trait::HardShell);
        let wire_card = card.to_wire();

        assert_eq!(wire::FoodValue::new(-3).unwrap(), wire_card.0);
        assert_eq!(wire::Trait::HardShell, wire_card.1);
    }
}
