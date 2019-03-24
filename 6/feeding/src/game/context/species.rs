use std::ops::{Deref, DerefMut};
use game::Game;
use game::context::*;
use interact::*;
use object::{Trait, Placement, Species as RealSpecies};

/// A game aware species.
#[derive(Debug)]
pub struct Species<'a, C: 'a + Chooser>(&'a mut Game<C>, usize, usize);

impl<'a, C: Chooser> Species<'a, C> {
    pub fn new(game: &'a mut Game<C>, indices: (usize, usize)) -> Self {
        let (player_idx, species_idx) = indices;
        Species(game, player_idx, species_idx)
    }

    /// This species' player.
    pub fn player(&mut self) -> Player<C> {
        Player::new(self.0, self.1)
    }

    /// This species' neighbor on the given placement.
    pub fn neighbor(&mut self, placement: Placement) -> Option<Species<C>> {
        let player_idx = self.1;
        let neighbor_idx = match placement {
            Placement::Right => self.2 + 1,
            Placement::Left => self.2.wrapping_sub(1),
        };
        if self.player().domain().get(neighbor_idx).is_some() {
            Some(Species::new(self.context_mut(), (player_idx, neighbor_idx)))
        } else {
            None
        }
    }

    /// Feed this species.
    pub fn feed(&mut self) {
        let mut acc = 0;
        for _ in 0..self.eats() {
            if !self.eat() {
                break;
            } else {
                acc += 1;
            }
        }

        for _ in 0..acc {
            self.cooperate();
        }
    }

    /// Tell this species to eat, returning false if they can't.
    // TODO: Result type when we do results for all these functions.
    pub fn eat(&mut self) -> bool {
        let food = match self.context_mut().board_mut().pop_food() {
            Some(f) => f,
            None => return false,
        };

        match RealSpecies::eat(&mut **self, food) {
            Ok(_) => true,
            Err(food) => {
                self.context_mut().board_mut().push_food(food);
                false
            }
        }
    }

    /// Trigger cooperation for this species if applicable.
    pub fn cooperate(&mut self) {
        if self.has_trait(Trait::Cooperation) {
            if let Some(mut neighbor) = self.neighbor(Placement::Right) {
                neighbor.feed();
            }
        }
    }

    /// Store the given amount of food into this species' fat.
    pub fn store(&mut self, amount: u64) {
        let food = self.context_mut().board_mut().pop_foods(amount).expect("not enough food");
        RealSpecies::store(&mut **self, food).expect("can't store food");
        // TODO: Push food back to board in the case of an error.
    }

    /// Kill a member of this species.
    pub fn kill(&mut self) -> Result<bool, ()> {
        let species_idx = self.2;
        self.player().domain_mut().kill(species_idx)
    }
}

impl<'a, C: Chooser> Context<'a, Game<C>> for Species<'a, C> {
    fn context(&self) -> &Game<C> {
        &self.0
    }

    fn context_mut(&mut self) -> &mut Game<C> {
        &mut self.0
    }
}

impl<'a, C: Chooser> Deref for Species<'a, C> {
    type Target = RealSpecies;

    fn deref(&self) -> &Self::Target {
        &self.0.players()[self.1].domain()[self.2]
    }
}

impl<'a, C: Chooser> DerefMut for Species<'a, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0.players_mut()[self.1].domain_mut()[self.2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use game::*;
    use object::{FoodToken, Trait, Placement};

    #[test]
    fn neighbor_some() {
        let mut game = game_with_players(3, &|player| {
            if player.id() == 2 {
                player.domain_mut().add(Placement::Right);
                player.domain_mut().add(Placement::Right);
            }
        });

        assert_eq!(2, game.players()[1].domain().len());

        let mut context_species = Species::new(&mut game, (1, 0));

        assert!(context_species.neighbor(Placement::Right).is_some());
    }

    #[test]
    fn neighbor_none() {
        let mut game = game_with_players(3, &|player| {
            if player.id() == 2 {
                player.domain_mut().add(Placement::Right);
                player.domain_mut().add(Placement::Right);
            }
        });

        assert_eq!(2, game.players()[1].domain().len());

        assert!(Species::new(&mut game, (1, 1)).neighbor(Placement::Right).is_none());
    }

    #[test]
    fn feed() {
        let mut game = game_with_players(3, &|player| {
            if player.id() == 2 {
                player.domain_mut().add(Placement::Right);
            }
        });
        game.board_mut().push_food(FoodToken);

        assert_eq!(1, game.board().food().len());
        assert_eq!(0, game.players()[1].domain()[0].food().len());

        Species::new(&mut game, (1, 0)).feed();

        assert_eq!(0, game.board().food().len());
        assert_eq!(1, game.players()[1].domain()[0].food().len());
    }

    #[test]
    fn feed_forager() {
        let mut game = game_with_players(3, &|player| {
            if player.id() == 2 {
                player.domain_mut().add(Placement::Right);
                player.domain_mut()[0].evolve(Trait::Foraging).unwrap();
                player.domain_mut()[0].breed().unwrap();
                player.domain_mut()[0].breed().unwrap();
            }
        });
        game.board_mut().push_food(FoodToken);
        game.board_mut().push_food(FoodToken);
        game.board_mut().push_food(FoodToken);

        assert_eq!(3, game.board().food().len());
        assert_eq!(0, game.players()[1].domain()[0].food().len());

        Species::new(&mut game, (1, 0)).feed();

        assert_eq!(1, game.board().food().len());
        assert_eq!(2, game.players()[1].domain()[0].food().len());
    }

    #[test]
    fn feed_cooperation_chain() {
        let mut game = game_with_players(3, &|player| {
            if player.id() == 2 {
                player.domain_mut().add(Placement::Right);
                player.domain_mut()[0].evolve(Trait::Cooperation).unwrap();
                player.domain_mut().add(Placement::Right);
                player.domain_mut()[1].evolve(Trait::Cooperation).unwrap();
                player.domain_mut().add(Placement::Right);
            }
        });
        game.board_mut().push_food(FoodToken);
        game.board_mut().push_food(FoodToken);
        game.board_mut().push_food(FoodToken);

        assert_eq!(3, game.board().food().len());
        assert_eq!(0, game.players()[1].domain()[0].food().len());
        assert_eq!(0, game.players()[1].domain()[1].food().len());
        assert_eq!(0, game.players()[1].domain()[2].food().len());

        Species::new(&mut game, (1, 0)).feed();

        assert_eq!(0, game.board().food().len());
        assert_eq!(1, game.players()[1].domain()[0].food().len());
        assert_eq!(1, game.players()[1].domain()[1].food().len());
        assert_eq!(1, game.players()[1].domain()[2].food().len());
    }

    #[test]
    fn feed_cooperation_chain_without_enough_food() {
        let mut game = game_with_players(3, &|player| {
            if player.id() == 2 {
                player.domain_mut().add(Placement::Right);
                player.domain_mut()[0].evolve(Trait::Cooperation).unwrap();
                player.domain_mut().add(Placement::Right);
                player.domain_mut()[1].evolve(Trait::Cooperation).unwrap();
                player.domain_mut().add(Placement::Right);
                player.domain_mut()[2].evolve(Trait::Cooperation).unwrap();
                player.domain_mut().add(Placement::Right);
            }
        });
        game.board_mut().push_food(FoodToken);
        game.board_mut().push_food(FoodToken);
        game.board_mut().push_food(FoodToken);

        assert_eq!(3, game.board().food().len());
        assert_eq!(0, game.players()[1].domain()[0].food().len());
        assert_eq!(0, game.players()[1].domain()[1].food().len());
        assert_eq!(0, game.players()[1].domain()[2].food().len());
        assert_eq!(0, game.players()[1].domain()[3].food().len());

        Species::new(&mut game, (1, 0)).feed();

        assert_eq!(0, game.board().food().len());
        assert_eq!(1, game.players()[1].domain()[0].food().len());
        assert_eq!(1, game.players()[1].domain()[1].food().len());
        assert_eq!(1, game.players()[1].domain()[2].food().len());
        assert_eq!(0, game.players()[1].domain()[3].food().len());
    }

    #[test]
    fn feed_partially_full_cooperating_forager() {
        let mut game = game_with_players(3, &|player| {
            if player.id() == 2 {
                player.domain_mut().add(Placement::Right);
                player.domain_mut()[0].eat(FoodToken).unwrap();
                player.domain_mut()[0].breed().unwrap();
                player.domain_mut()[0].evolve(Trait::Cooperation).unwrap();
                player.domain_mut()[0].evolve(Trait::Foraging).unwrap();
                player.domain_mut().add(Placement::Right);
            }
        });
        game.board_mut().push_foods(vec![FoodToken, FoodToken, FoodToken]);

        assert_eq!(1, game.players()[1].domain()[0].food().len());
        assert_eq!(0, game.players()[1].domain()[1].food().len());
        assert_eq!(3, game.board().food().len());

        Species::new(&mut game, (1, 0)).feed();

        assert_eq!(2, game.players()[1].domain()[0].food().len());
        assert_eq!(1, game.players()[1].domain()[1].food().len());
        assert_eq!(1, game.board().food().len());
    }

    #[test]
    fn eat() {
        let mut game = game_with_players(3, &|player| {
            if player.id() == 2 {
                player.domain_mut().add(Placement::Right);
            }
        });
        game.board_mut().push_food(FoodToken);

        assert_eq!(1, game.board().food().len());
        assert_eq!(0, game.players()[1].domain()[0].food().len());

        assert!(Species::new(&mut game, (1, 0)).eat());

        assert_eq!(0, game.board().food().len());
        assert_eq!(1, game.players()[1].domain()[0].food().len());
    }

    #[test]
    fn cooperate() {
        let mut game = game_with_players(3, &|player| {
            if player.id() == 2 {
                player.domain_mut().add(Placement::Right);
                player.domain_mut()[0].evolve(Trait::Cooperation).unwrap();
                player.domain_mut().add(Placement::Right);
            }
        });
        game.board_mut().push_food(FoodToken);
        game.board_mut().push_food(FoodToken);
        game.board_mut().push_food(FoodToken);

        assert_eq!(3, game.board().food().len());
        assert_eq!(0, game.players()[1].domain()[0].food().len());
        assert_eq!(0, game.players()[1].domain()[1].food().len());

        Species::new(&mut game, (1, 0)).cooperate();

        assert_eq!(2, game.board().food().len());
        assert_eq!(0, game.players()[1].domain()[0].food().len());
        assert_eq!(1, game.players()[1].domain()[1].food().len());
    }

    #[test]
    fn store() {
        let mut game = game_with_players(3, &|player| {
            if player.id() == 2 {
                player.domain_mut().add(Placement::Right);
                player.domain_mut()[0].evolve(Trait::FatTissue).unwrap();
                player.domain_mut()[0].grow().unwrap();
                player.domain_mut()[0].grow().unwrap();
            }
        });
        game.board_mut().push_food(FoodToken);
        game.board_mut().push_food(FoodToken);
        game.board_mut().push_food(FoodToken);

        assert_eq!(3, game.board().food().len());
        assert_eq!(0, game.players()[1].domain()[0].food().len());
        assert_eq!(2, game.players()[1].domain()[0].body_size());

        Species::new(&mut game, (1, 0)).store(2);

        assert_eq!(1, game.board().food().len());
        assert_eq!(0, game.players()[1].domain()[0].food().len());
        assert_eq!(2, game.players()[1].domain()[0].fat().len());
    }

    #[test]
    #[should_panic]
    fn store_too_much() {
        let mut game = game_with_players(3, &|player| {
            if player.id() == 2 {
                player.domain_mut().add(Placement::Right);
                player.domain_mut()[0].evolve(Trait::FatTissue).unwrap();
                player.domain_mut()[0].grow().unwrap();
                player.domain_mut()[0].grow().unwrap();
            }
        });
        game.board_mut().push_food(FoodToken);
        game.board_mut().push_food(FoodToken);
        game.board_mut().push_food(FoodToken);

        assert_eq!(3, game.board().food().len());
        assert_eq!(0, game.players()[1].domain()[0].food().len());
        assert_eq!(2, game.players()[1].domain()[0].body_size());

        Species::new(&mut game, (1, 0)).store(3);
    }

    #[test]
    fn kill() {
        let mut game = game_with_players(3, &|player| {
            if player.id() == 2 {
                player.domain_mut().add(Placement::Right);
                player.domain_mut()[0].breed().unwrap();
            }
        });

        assert_eq!(2, game.players()[1].domain()[0].population());

        assert!(!Species::new(&mut game, (1, 0)).kill().unwrap());
        assert_eq!(1, game.players()[1].domain()[0].population());

        assert!(Species::new(&mut game, (1, 0)).kill().unwrap());
    }
}
