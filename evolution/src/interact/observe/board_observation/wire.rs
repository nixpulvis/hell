use evolution_wire as wire;
use interact::*;

impl wire::ToWire<wire::Natural> for BoardObservation {
    fn to_wire(&self) -> wire::Natural {
        self.food.to_wire()
    }
}

impl wire::FromWire<wire::Natural> for BoardObservation {
    fn from_wire(wire: wire::Natural) -> Result<Self, ()> {
        Ok(BoardObservation {
            food: *wire,
        })
    }
}

impl wire::ToWire<wire::NaturalPlus> for BoardObservation {
    fn to_wire(&self) -> wire::NaturalPlus {
        self.food.to_wire()
    }
}

impl wire::FromWire<wire::NaturalPlus> for BoardObservation {
    fn from_wire(wire: wire::NaturalPlus) -> Result<Self, ()> {
        Ok(BoardObservation {
            food: *wire,
        })
    }
}

#[cfg(test)]
mod tests {
    use evolution_wire::{self as wire, ToWire, FromWire};
    use interact::*;
    use object::*;

    #[test]
    fn observed_board_to_wire_natural() {
        let mut board = Board::default();
        board.push_foods(vec![FoodToken, FoodToken]);

        assert_eq!(2, board.food().len());

        let wire: wire::Natural = board.observe().to_wire();

        assert_eq!(2, *wire);
    }

    #[test]
    fn observed_board_from_wire_natural() {
        let wire = wire::Natural(20);
        let board = BoardObservation::from_wire(wire).unwrap();
        assert_eq!(20, board.food);
    }

    #[test]
    fn observed_board_to_wire_natural_plus() {
        let mut board = Board::default();
        board.push_foods(vec![FoodToken, FoodToken]);

        assert_eq!(2, board.food().len());

        let wire: wire::NaturalPlus = board.observe().to_wire();

        assert_eq!(2, *wire);
    }

    #[test]
    fn observed_board_from_wire_natural_plus() {
        let wire = wire::NaturalPlus::new(20).unwrap();
        let board = BoardObservation::from_wire(wire).unwrap();
        assert_eq!(20, board.food);
    }
}
