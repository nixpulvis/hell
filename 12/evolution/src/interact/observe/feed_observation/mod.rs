use ext::*;
use game::*;
use interact::*;
use object::*;

#[derive(Debug, Clone)]
pub struct FeedObservation {
    pub current_player: Player,
    pub opponents: Vec<PlayerObservation>,
    pub board: BoardObservation,
}

impl Observation for FeedObservation {}

impl<C: Chooser> Observe<FeedObservation> for Game<C> {
    fn observe(&self) -> FeedObservation {
        let idx = self.current_player_idx();
        let current_player = self.current_player().clone();
        let board = self.board().observe();
        let opponents = self.players()
                            .around_from(idx)
                            .map(|(_, player)| player.observe())
                            .skip(1)
                            .collect();
        FeedObservation {
            current_player: current_player,
            opponents: opponents,
            board: board,
        }
    }
}

#[cfg(feature = "wire")]
mod wire;

#[cfg(test)]
mod tests {
    use ext::*;
    use game::*;
    use interact::*;

    #[test]
    fn observed_preserves_ordering() {
        let game = game_with_players(3, &|_| {});
        let observation: FeedObservation = game.observe();
        let game_opponents = game.players().around_from(game.current_player_idx()).skip(1);

        for ((_, player), obsered_player) in game_opponents.zip(observation.opponents.iter()) {
            assert_eq!(player.id(), obsered_player.id)
        }
    }
}
