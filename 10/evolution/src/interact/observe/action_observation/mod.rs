use game::*;
use object::*;
use interact::*;

// TODO: Docs
#[derive(Debug, Clone)]
pub struct ActionObservation {
    pub current_player: Player,
    pub before: Vec<Domain>,
    pub after: Vec<Domain>,
}

impl Observation for ActionObservation {}

// TODO: impl Observe
impl<C: Chooser> Observe<ActionObservation> for Game<C> {
    fn observe(&self) -> ActionObservation {
        let idx = self.current_player_idx();
        let before = self.players()[..idx].iter().map(|p| p.domain().clone()).collect();
        let after = self.players()[idx + 1..].iter().map(|p| p.domain().clone()).collect();
        ActionObservation {
            current_player: self.current_player().clone(),
            before: before,
            after: after,
        }
    }
}

#[cfg(feature = "wire")]
mod wire;

#[cfg(test)]
mod tests {
    use game::*;
    use interact::*;
    use object::*;

    #[test]
    fn game_action_observation() {
        // game: [id(1), id(2), id(3), id(4)]
        // => current_player: id(2)
        //    before: [id(1)]
        //    after: [id(3), id(4)]
        let mut game = game_with_players(4, &|player| {
            match player.id() {
                1 => {
                    player.domain_mut().add(Placement::Right);
                }
                2 => {
                    player.domain_mut().add(Placement::Right);
                    player.domain_mut().add(Placement::Right);
                }
                3 => {
                    player.domain_mut().add(Placement::Right);
                    player.domain_mut().add(Placement::Right);
                    player.domain_mut().add(Placement::Right);
                }
                4 => {
                    player.domain_mut().add(Placement::Right);
                    player.domain_mut().add(Placement::Right);
                    player.domain_mut().add(Placement::Right);
                    player.domain_mut().add(Placement::Right);
                }
                _ => {}
            }
        });
        game.advance_current_player();

        assert_eq!(1, game.current_player_idx());
        assert_eq!(1, game.players()[0].domain().len());
        assert_eq!(2, game.players()[1].domain().len());
        assert_eq!(3, game.players()[2].domain().len());
        assert_eq!(4, game.players()[3].domain().len());

        let observation: ActionObservation = game.observe();

        assert_eq!(2, observation.current_player.domain().len());
        assert_eq!(1, observation.before[0].len());
        assert_eq!(3, observation.after[0].len());
        assert_eq!(4, observation.after[1].len());
    }
}
