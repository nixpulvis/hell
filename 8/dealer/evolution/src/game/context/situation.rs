use game::Game;
use game::context::*;
use interact::*;
use object::Trait;

/// An attack within a game.
pub struct Situation<'a, C: 'a + Chooser>(&'a mut Game<C>, usize, usize, usize, usize);

impl<'a, C: Chooser> Situation<'a, C> {
    /// Create a new instance.
    ///
    /// # Arguments
    /// `game`: The `Game` instance to mutate.
    /// `indices`: A tuple of `(i, j, k, l)` where `i` is the index of the
    ///  currently attacking player, `j` is the carnivorous species belonging
    ///  to that player, `k` is the defending player, and `l` is the
    ///  defending species belonging to that player,
    pub fn new(game: &'a mut Game<C>, indices: (usize, usize, usize, usize)) -> Self {
        let (us_idx, attacker_idx, them_idx, defender_idx) = indices;
        Situation(game, us_idx, attacker_idx, them_idx, defender_idx)
    }

    /// The attacking player.
    pub fn us(&mut self) -> Player<C> {
        Player::new(self.0, self.1)
    }

    /// The attacking species.
    pub fn attacker(&mut self) -> Species<C> {
        Species::new(self.0, (self.1, self.2))
    }

    /// The defending player.
    pub fn them(&mut self) -> Player<C> {
        Player::new(self.0, self.3)
    }

    /// The defending species.
    pub fn defender(&mut self) -> Species<C> {
        Species::new(self.0, (self.3, self.4))
    }

    /// Apply the effects of an attack of the attacker to the defender to the
    /// game.
    pub fn fight(&mut self) -> Result<(), ()> {
        let attacker_extinct = if self.defender().has_trait(Trait::Horns) {
            try!(self.attacker().kill())
        } else {
            false
        };

        if try!(self.defender().kill()) {
            self.them().refund();
        }

        if attacker_extinct {
            self.us().refund();
        } else {
            self.attacker().feed();
        }

        self.us().scavenge();
        Ok(())
    }
}

impl<'a, C: Chooser> Context<'a, Game<C>> for Situation<'a, C> {
    fn context(&self) -> &Game<C> {
        &self.0
    }

    fn context_mut(&mut self) -> &mut Game<C> {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use game::*;
    use object::{Placement, FoodToken, Trait};

    #[test]
    fn fight() {
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
        assert_eq!(0, game.players()[0].domain()[0].food().len());
        assert_eq!(1, game.players()[1].domain().len());
        assert_eq!(0, game.players()[1].hand().len());

        Situation::new(&mut game, (0, 0, 1, 0)).fight().unwrap();

        assert_eq!(0, game.board().food().len());
        assert_eq!(1, game.players()[0].domain()[0].food().len());
        assert_eq!(0, game.players()[1].domain().len());
        assert_eq!(2, game.players()[1].hand().len());
    }

    #[test]
    fn horns() {
        let mut game = game_with_players(3, &|player| {
            match player.id() {
                1 => {
                    player.domain_mut().add(Placement::Right);
                    player.domain_mut()[0].evolve(Trait::Carnivore).unwrap();
                },
                2 => {
                    player.domain_mut().add(Placement::Right);
                    player.domain_mut()[0].evolve(Trait::Horns).unwrap();
                },
                _ => {},
            }
        });
        game.board_mut().push_food(FoodToken);

        assert_eq!(1, game.board().food().len());
        assert_eq!(1, game.players()[0].domain().len());
        assert_eq!(0, game.players()[0].hand().len());
        assert_eq!(1, game.players()[1].domain().len());
        assert_eq!(0, game.players()[1].hand().len());

        Situation::new(&mut game, (0, 0, 1, 0)).fight().unwrap();

        assert_eq!(1, game.board().food().len());
        assert_eq!(0, game.players()[0].domain().len());
        assert_eq!(2, game.players()[0].hand().len());
        assert_eq!(0, game.players()[1].domain().len());
        assert_eq!(2, game.players()[1].hand().len());
    }

    #[test]
    fn scavenge() {
        let mut game = game_with_players(3, &|player| {
            player.domain_mut().add(Placement::Right);
            match player.id() {
                1 => {
                    player.domain_mut()[0].evolve(Trait::Carnivore).unwrap();
                },
                _ => {
                    player.domain_mut().add(Placement::Right);
                    player.domain_mut()[0].evolve(Trait::Scavenger).unwrap();
                },
            }
        });
        game.board_mut().push_food(FoodToken);
        game.board_mut().push_food(FoodToken);
        game.board_mut().push_food(FoodToken);

        assert_eq!(3, game.board().food().len());
        assert_eq!(2, game.players()[1].domain().len());
        assert_eq!(2, game.players()[2].domain().len());

        Situation::new(&mut game, (0, 0, 1, 1)).fight().unwrap();

        assert_eq!(0, game.board().food().len());
        assert_eq!(1, game.players()[1].domain().len());
        assert_eq!(2, game.players()[2].domain().len());
        assert_eq!(1, game.players()[0].domain()[0].food().len());
        assert_eq!(1, game.players()[1].domain()[0].food().len());
        assert_eq!(1, game.players()[2].domain()[0].food().len());
    }
}
