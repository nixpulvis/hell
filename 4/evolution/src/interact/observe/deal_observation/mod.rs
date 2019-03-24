use interact::*;
use object::*;

#[derive(Debug)]
pub struct DealObservation {
    pub board: BoardObservation,
    pub player: PlayerObservation,
    pub hand: Vec<Card>,
}

impl Observation for DealObservation {}

impl Observe<DealObservation> for (Board, Player) {
    fn observe(&self) -> DealObservation {
        DealObservation {
            board: self.0.observe(),
            player: self.1.observe(),
            hand: self.1.hand().clone().to_vec(),
        }
    }
}

#[cfg(feature = "wire")]
mod wire;

#[cfg(test)]
mod tests {
    use interact::*;
    use object::*;

    #[test]
    fn observe() {
        let mut board = Board::default();
        board.push_foods(vec![FoodToken, FoodToken, FoodToken]);
        let mut player = Player::new(123);
        player.domain_mut().add(Placement::Right);
        player.push_cards(vec![
            Card::mock(1, Trait::Ambush),
            Card::mock(1, Trait::Burrowing),
            Card::mock(1, Trait::Climbing),
            Card::mock(1, Trait::Carnivore),
        ]);

        assert_eq!(3, board.food().len());
        assert_eq!(123, player.id());
        assert_eq!(1, player.domain().len());
        assert_eq!(4, player.hand().len());

        let observation = (board, player).observe();

        assert_eq!(3, observation.board.food);
        assert_eq!(123, observation.player.id);
        assert_eq!(1, observation.player.domain.len());
        assert_eq!(4, observation.hand.len());
    }
}
