use std::ops::{Deref, DerefMut};
use game::*;
use game::context::{Species, Situation};
use interact::*;

/// A single step of feeding in the game. This step is in charge of getting
/// a choice for how to use one's species to either `Feed`, `Store` or
/// `Attack`. See `FeedChoice` for more information on these types.
///
/// Both `Feed` and `Attack` calls a species `feed` method, which triggers
/// other actions based on the traits of the species in the game.
pub struct Feed<'a, C: 'a + Chooser>(pub &'a mut Game<C>, pub &'a mut Choose<FeedObservation, FeedChoice>);

impl<'a, C: Chooser> Feed<'a, C> {
    fn apply(&mut self, mut choice: FeedChoice) -> Result<(), ()> {
        choice.internalize(self);
        let idx = self.current_player_idx();
        match choice {
            FeedChoice::Abstain => {
                self.skip_advance_current_player();
                Ok(())
            }
            FeedChoice::Feed(sx) => {
                Species::new(self, (idx, sx)).feed();
                Ok(())
            }
            FeedChoice::Store(sx, amount) => {
                Species::new(self, (idx, sx)).store(amount);
                Ok(())
            }
            FeedChoice::Attack(sx, tx, dx) => {
                Situation::new(self, (idx, sx, tx, dx)).fight()
            }
        }
    }
}

impl<'a, C: Chooser> step::Step<C> for Feed<'a, C> {
    fn step(&mut self) -> Result<(), ()> {
        let observation = self.observe();
        match self.1.choose(&observation) {
            Ok(Some(c)) => {
                debug!("applying choice: {:?}", c);
                match self.apply(c) {
                    Ok(_) => {}
                    Err(_) => {
                        warn!("ejecting playing during choice application");
                        self.eject_current_player()
                    },
                }
                self.advance_current_player();
                Ok(())
            }
            Ok(None) => {
                self.skip_advance_current_player();
                Ok(())
            }
            Err(_) => {
                warn!("error getting player choice");
                self.eject_current_player();
                Ok(())
            }
        }
    }
}

impl<'a, C: Chooser> Deref for Feed<'a, C> {
    type Target = Game<C>;

    fn deref(&self) -> &Game<C> {
        &self.0
    }
}

impl<'a, C: Chooser> DerefMut for Feed<'a, C> {
    fn deref_mut(&mut self) -> &mut Game<C> {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use game::*;
    use interact::*;
    use object::*;

    // TODO: Move the apply logic into it's own function.
    fn todo() -> FeedChoice {
        FeedChoice::Abstain
    }

    #[test]
    fn apply_abstain() {
        let mut game = game_with_players(3, &|player| {
            if player.id() == 1 {
                player.domain_mut().add(Placement::Right);
            }
        });
        game.board_mut().push_food(FoodToken);

        assert_eq!(1, game.board().food().len());
        assert_eq!(0, game.players()[0].domain()[0].food().len());

        step::Feed(&mut game, &mut todo()).apply(FeedChoice::Abstain).unwrap();

        assert_eq!(1, game.board().food().len());
        assert_eq!(0, game.players()[0].domain()[0].food().len());
    }

    #[test]
    fn apply_feed() {
        let mut game = game_with_players(3, &|player| {
            if player.id() == 1 {
                player.domain_mut().add(Placement::Right);
            }
        });
        game.board_mut().push_food(FoodToken);

        assert_eq!(1, game.board().food().len());
        assert_eq!(0, game.players()[0].domain()[0].food().len());

        step::Feed(&mut game, &mut todo()).apply(FeedChoice::Feed(0)).unwrap();

        assert_eq!(0, game.board().food().len());
        assert_eq!(1, game.players()[0].domain()[0].food().len());
    }

    #[test]
    fn apply_store() {
        let mut game = game_with_players(3, &|player| {
            if player.id() == 1 {
                player.domain_mut().add(Placement::Right);
                player.domain_mut()[0].evolve(Trait::FatTissue).unwrap();
                player.domain_mut()[0].grow().unwrap();
                player.domain_mut()[0].grow().unwrap();
            }
        });
        game.board_mut().push_foods(vec![FoodToken, FoodToken, FoodToken]);

        assert_eq!(3, game.board().food().len());
        assert_eq!(0, game.players()[0].domain()[0].fat().len());

        step::Feed(&mut game, &mut todo()).apply(FeedChoice::Store(0, 2)).unwrap();

        assert_eq!(1, game.board().food().len());
        assert_eq!(2, game.players()[0].domain()[0].fat().len());
    }

    #[test]
    fn apply_attack() {
        let mut game = game_with_players(3, &|player| {
            match player.id() {
                1 => {
                    player.domain_mut().add(Placement::Right);
                    player.domain_mut()[0].evolve(Trait::Carnivore).unwrap();
                },
                2 => {
                    player.domain_mut().add(Placement::Right);
                },
                _ => {},
            }
        });
        game.board_mut().push_food(FoodToken);

        assert_eq!(1, game.board().food().len());
        assert_eq!(1, game.players()[0].domain().len());
        assert_eq!(0, game.players()[0].domain()[0].food().len());
        assert_eq!(1, game.players()[1].domain().len());

        step::Feed(&mut game, &mut todo()).apply(FeedChoice::Attack(0, 0, 0)).unwrap();

        assert_eq!(0, game.board().food().len());
        assert_eq!(1, game.players()[0].domain().len());
        assert_eq!(1, game.players()[0].domain()[0].food().len());
        assert_eq!(0, game.players()[1].domain().len());
    }

    #[test]
    fn apply_foraging() {
        let mut game = game_with_players(3, &|player| {
            if player.id() == 1 {
                player.domain_mut().add(Placement::Right);
                player.domain_mut()[0].evolve(Trait::Foraging).unwrap();
                player.domain_mut()[0].breed().unwrap();
            }
        });
        game.board_mut().push_foods(vec![FoodToken, FoodToken]);

        assert_eq!(2, game.board().food().len());
        assert_eq!(0, game.players()[0].domain()[0].food().len());

        step::Feed(&mut game, &mut todo()).apply(FeedChoice::Feed(0)).unwrap();

        assert_eq!(0, game.board().food().len());
        assert_eq!(2, game.players()[0].domain()[0].food().len());
    }

    #[test]
    fn apply_foraging_scavenger() {
        let mut game = game_with_players(3, &|player| {
            match player.id() {
                1 => {
                    player.domain_mut().add(Placement::Right);
                },
                3 => {
                    player.domain_mut().add(Placement::Right);
                    player.domain_mut()[0].evolve(Trait::Carnivore).unwrap();
                    player.domain_mut().add(Placement::Right);
                    player.domain_mut()[1].evolve(Trait::Foraging).unwrap();
                    player.domain_mut()[1].evolve(Trait::Scavenger).unwrap();
                    player.domain_mut()[1].breed().unwrap();
                },
                _ => {},
            }
        });
        game.advance_current_player();
        game.advance_current_player();
        game.board_mut().push_foods(vec![FoodToken, FoodToken, FoodToken]);

        assert_eq!(2, game.current_player_idx());
        assert_eq!(3, game.board().food().len());
        assert_eq!(0, game.players()[2].domain()[1].food().len());

        step::Feed(&mut game, &mut todo()).apply(FeedChoice::Attack(0, 0, 0)).unwrap();

        assert_eq!(0, game.board().food().len());
        assert_eq!(2, game.players()[2].domain()[1].food().len());
    }

    #[test]
    fn apply_foraging_carnivore() {
        let mut game = game_with_players(3, &|player| {
            player.domain_mut().add(Placement::Right);
            if player.id() == 1 {
                player.domain_mut()[0].evolve(Trait::Carnivore).unwrap();
                player.domain_mut()[0].evolve(Trait::Foraging).unwrap();
                player.domain_mut()[0].breed().unwrap();
            }
        });
        game.board_mut().push_foods(vec![FoodToken, FoodToken]);

        assert_eq!(2, game.board().food().len());
        assert_eq!(0, game.players()[0].domain()[0].food().len());

        step::Feed(&mut game, &mut todo()).apply(FeedChoice::Attack(0, 1, 0)).unwrap();

        assert_eq!(0, game.board().food().len());
        assert_eq!(2, game.players()[0].domain()[0].food().len());
    }

    #[test]
    fn apply_cooperation() {
        let mut game = game_with_players(3, &|player| {
            if player.id() == 1 {
                player.domain_mut().add(Placement::Right);
                player.domain_mut()[0].evolve(Trait::Cooperation).unwrap();
                player.domain_mut().add(Placement::Right);
                player.domain_mut()[1].evolve(Trait::Cooperation).unwrap();
                player.domain_mut().add(Placement::Right);
                player.domain_mut()[2].evolve(Trait::Cooperation).unwrap();
            }
        });
        game.board_mut().push_foods(vec![FoodToken, FoodToken, FoodToken, FoodToken]);

        assert_eq!(4, game.board().food().len());
        assert_eq!(0, game.players()[0].domain()[0].food().len());
        assert_eq!(0, game.players()[0].domain()[1].food().len());
        assert_eq!(0, game.players()[0].domain()[2].food().len());

        step::Feed(&mut game, &mut todo()).apply(FeedChoice::Feed(0)).unwrap();

        assert_eq!(1, game.board().food().len());
        assert_eq!(1, game.players()[0].domain()[0].food().len());
        assert_eq!(1, game.players()[0].domain()[1].food().len());
        assert_eq!(1, game.players()[0].domain()[2].food().len());
    }

    #[test]
    fn apply_scavenge() {
        let mut game = game_with_players(3, &|player| {
            match player.id() {
                2 => {
                    player.domain_mut().add(Placement::Right);
                    player.domain_mut()[0].evolve(Trait::Scavenger).unwrap();
                },
                3 => {
                    player.domain_mut().add(Placement::Right);
                    player.domain_mut()[0].evolve(Trait::Carnivore).unwrap();
                    player.domain_mut().add(Placement::Right);
                    player.domain_mut()[1].evolve(Trait::Scavenger).unwrap();
                },
                _ => {},
            }
        });
        game.advance_current_player();
        game.advance_current_player();
        game.board_mut().push_foods(vec![FoodToken, FoodToken, FoodToken]);

        assert_eq!(2, game.current_player_idx());
        assert_eq!(3, game.board().food().len());
        assert_eq!(0, game.players()[2].domain()[1].food().len());

        step::Feed(&mut game, &mut todo()).apply(FeedChoice::Attack(0, 1, 0)).unwrap();

        assert_eq!(1, game.board().food().len());
        assert_eq!(0, game.players()[1].domain().len());
        assert_eq!(1, game.players()[2].domain()[1].food().len());
    }

    #[test]
    fn apply_horns() {
        let mut game = game_with_players(3, &|player| {
            match player.id() {
                2 => {
                    player.domain_mut().add(Placement::Right);
                    player.domain_mut()[0].evolve(Trait::Horns).unwrap();
                },
                3 => {
                    player.domain_mut().add(Placement::Right);
                    player.domain_mut()[0].evolve(Trait::Carnivore).unwrap();
                },
                _ => {},
            }
        });
        game.advance_current_player();
        game.advance_current_player();
        game.board_mut().push_foods(vec![FoodToken, FoodToken, FoodToken]);

        assert_eq!(2, game.current_player_idx());
        assert_eq!(3, game.board().food().len());

        step::Feed(&mut game, &mut todo()).apply(FeedChoice::Attack(0, 1, 0)).unwrap();

        assert_eq!(3, game.board().food().len());
        assert_eq!(0, game.players()[2].domain().len());
        assert_eq!(0, game.players()[1].domain().len());
    }

    #[test]
    fn apply_remove_extinct() {
        let mut game = game_with_players(3, &|player| {
            match player.id() {
                2 => {
                    player.domain_mut().add(Placement::Right);
                },
                3 => {
                    player.domain_mut().add(Placement::Right);
                    player.domain_mut()[0].evolve(Trait::Carnivore).unwrap();
                },
                _ => {},
            }
        });
        game.advance_current_player();
        game.advance_current_player();
        game.board_mut().push_foods(vec![FoodToken]);

        assert_eq!(2, game.current_player_idx());
        assert_eq!(0, game.players()[1].hand().len());

        step::Feed(&mut game, &mut todo()).apply(FeedChoice::Attack(0, 1, 0)).unwrap();

        assert_eq!(0, game.players()[1].domain().len());
        assert_eq!(2, game.players()[1].hand().len());
    }
}
