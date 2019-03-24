use evolution_wire as wire;
use object::*;

impl wire::ToWire<wire::Natural> for Board {
    fn to_wire(&self) -> wire::Natural {
        wire::Natural(self.food().len() as u64)
    }
}

impl wire::FromWire<wire::Natural> for Board {
    fn from_wire(wire: wire::Natural) -> Result<Self, ()> {
        Ok(Board {
            food: (0..*wire).into_iter().map(|_| FoodToken).collect(),
            cards: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use evolution_wire::{self as wire, ToWire, FromWire};
    use object::*;

    #[test]
    fn board_to_wire() {
        let mut board = Board::default();
        board.push_foods(vec![FoodToken, FoodToken]);

        assert_eq!(2, board.food().len());

        let wire = board.to_wire();

        assert_eq!(2, *wire);
    }

    #[test]
    fn board_from_wire() {
        let wire = wire::Natural(10);
        let board = Board::from_wire(wire);

        assert_eq!(10, board.unwrap().food().len());
    }
}
