use object::*;
use interact::*;

/// The observed view of a board, with just the food of a board.
#[derive(Debug, Clone, Default)]
pub struct BoardObservation {
    pub food: u64,
}

impl Observation for BoardObservation {}

impl Observe<BoardObservation> for Board {
    fn observe(&self) -> BoardObservation {
        BoardObservation {
            food: self.food().len() as u64,
        }
    }
}

#[cfg(feature = "wire")]
mod wire;
