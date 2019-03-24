use std::ops::{Deref, DerefMut};
use game::Game;
use game::context::*;
use interact::*;
use object::{FoodToken, Trait, Board as RealBoard};

/// A game aware board.
pub struct Board<'a, C: 'a + Chooser>(&'a mut Game<C>);

impl<'a, C: Chooser> Board<'a, C> {
    /// Create a new contextual board instance from a mutable `Game` object
    /// reference.
    ///
    /// # Arguments
    ///
    /// * `game` - A mutable reference to a `Game` instance.
    ///
    /// # Returns
    ///
    /// A contextual `Board` instance referencing the given `Game` instance.
    pub fn new(game: &'a mut Game<C>) -> Self {
        Board(game)
    }

    /// Reveal the cards played to the board as food. Doing so will also clear
    /// the cards from the board and add or subtract the appropriate number of
    /// `FoodToken`s.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - The contextual board instance to reveal cards for.
    ///
    /// # Panics
    ///
    /// This function panics if there are no cards to reveal.
    pub fn reveal(&mut self) {
        // NOTE: If we had a method to take ownership of these cards,
        //       we could do some really elegant composition of drain
        //       and fold. Maybe even a "take_cards" method yielding a
        //       draining iterator?
        let food_values: Vec<i64> = if let Some(cards) = self.cards_mut() {
            cards.iter().map(|c| c.food_value()).collect()
        } else {
            vec![]
        };
        for food_value in food_values {
            if food_value > 0 {
                self.push_foods((0..food_value).into_iter().map(|_| FoodToken).collect());
            } else {
                self.pop_foods(-food_value as u64);
            }
        }
        self.clear_cards();

        let mut fertiles = vec![];
        for (player_idx, player) in self.context().players().iter().enumerate() {
            for (species_idx, (species, _, _)) in player.domain().into_iter().enumerate() {
                if species.has_trait(Trait::Fertile) {
                    fertiles.push((player_idx, species_idx));
                }
            }
        }
        for (player_idx, species_idx) in fertiles {
            Species::new(self.context_mut(), (player_idx, species_idx)).breed().ok();
        }

        let mut long_necks = vec![];
        for (player_idx, player) in self.context().players().iter().enumerate() {
            for (species_idx, (species, _, _)) in player.domain().into_iter().enumerate() {
                if species.has_trait(Trait::LongNeck) {
                    long_necks.push((player_idx, species_idx));
                }
            }
        }
        for (player_idx, species_idx) in long_necks {
            Species::new(self.context_mut(), (player_idx, species_idx)).feed();
        }

        let mut all = vec![];
        for (player_idx, player) in self.context().players().iter().enumerate() {
            for (species_idx, (_, _, _)) in player.domain().into_iter().enumerate() {
                all.push((player_idx, species_idx));
            }
        }
        for (player_idx, species_idx) in all {
            Species::new(self.context_mut(), (player_idx, species_idx)).digest_fat();
        }
    }
}

impl<'a, C: Chooser> Context<'a, Game<C>> for Board<'a, C> {
    fn context(&self) -> &Game<C> {
        &self.0
    }

    fn context_mut(&mut self) -> &mut Game<C> {
        &mut self.0
    }
}

impl<'a, C: Chooser> Deref for Board<'a, C> {
    type Target = RealBoard;

    fn deref(&self) -> &Self::Target {
        self.context().board()
    }
}

impl<'a, C: Chooser> DerefMut for Board<'a, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.context_mut().board_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use game::*;
    use object::{Trait, FoodToken, Card, Placement};

    #[test]
    fn reveal() {
        let mut game = game_with_players(3, &|_| {});
        game.board_mut().set_cards(vec![
            Card::mock(1, Trait::Ambush),
            Card::mock(1, Trait::Burrowing),
            Card::mock(1, Trait::Carnivore),
        ]);

        assert_eq!(0, game.board().food().len());
        assert_eq!(3, game.board().cards().unwrap().len());

        Board::new(&mut game).reveal();

        assert_eq!(3, game.board().food().len());
        assert!(game.board().cards().is_none());
    }

    #[test]
    fn fertiles() {
        let mut game = game_with_players(3, &|player| {
            if player.id() == 1 {
                player.domain_mut().add(Placement::Right);
                player.domain_mut()[0].evolve(Trait::Fertile).unwrap();
            }
        });
        game.board_mut().set_cards(vec![
            Card::mock(1, Trait::Ambush),
            Card::mock(1, Trait::Ambush),
            Card::mock(1, Trait::Ambush),
        ]);

        assert_eq!(1, game.players()[0].domain()[0].population());

        Board::new(&mut game).reveal();

        assert_eq!(2, game.players()[0].domain()[0].population());
    }

    #[test]
    fn long_necks() {
        let mut game = game_with_players(3, &|player| {
            match player.id() {
                1 | 2 => {
                    player.domain_mut().add(Placement::Right);
                    player.domain_mut()[0].evolve(Trait::LongNeck).unwrap();
                },
                _ => {},
            }
        });
        game.board_mut().set_cards(vec![
            Card::mock(1, Trait::Ambush),
            Card::mock(0, Trait::Ambush),
            Card::mock(0, Trait::Ambush),
        ]);

        assert_eq!(0, game.board().food().len());
        assert_eq!(0, game.players()[0].domain()[0].food().len());
        assert_eq!(0, game.players()[1].domain()[0].food().len());

        Board::new(&mut game).reveal();

        assert_eq!(0, game.board().food().len());
        assert_eq!(1, game.players()[0].domain()[0].food().len());
        assert_eq!(0, game.players()[1].domain()[0].food().len());
    }

    #[test]
    fn fat_tissue() {
        let mut game = game_with_players(3, &|player| {
            match player.id() {
                1 | 2 => {
                    player.domain_mut().add(Placement::Right);
                    player.domain_mut()[0].evolve(Trait::FatTissue).unwrap();
                    player.domain_mut()[0].grow().unwrap();
                    player.domain_mut()[0].grow().unwrap();
                    player.domain_mut()[0].breed().unwrap();
                },
                _ => {},
            }
        });
        game.players_mut()[0].domain_mut()[0].store(vec![FoodToken, FoodToken]).unwrap();
        game.board_mut().set_cards(vec![
            Card::mock(0, Trait::Ambush),
            Card::mock(0, Trait::Ambush),
            Card::mock(0, Trait::Ambush),
        ]);

        assert_eq!(0, game.players()[0].domain()[0].food().len());
        assert_eq!(2, game.players()[0].domain()[0].fat().len());
        assert_eq!(0, game.players()[1].domain()[0].food().len());
        assert_eq!(0, game.players()[1].domain()[0].fat().len());

        Board::new(&mut game).reveal();

        assert_eq!(2, game.players()[0].domain()[0].food().len());
        assert_eq!(0, game.players()[0].domain()[0].fat().len());
        assert_eq!(0, game.players()[1].domain()[0].food().len());
        assert_eq!(0, game.players()[1].domain()[0].fat().len());
    }

    #[test]
    fn reveal_takes_or_adds_food_tokens_in_order() {
        // This means the logic of summing the value of the food cards is
        // incorrect. I couldn't find anything in the spec to conver this
        // case.
        let mut game = game_with_players(3, &|_| {});
        game.board_mut().set_cards(vec![
            Card::mock(-7, Trait::Carnivore),
            Card::mock(3, Trait::Carnivore),
            Card::mock(-2, Trait::Fertile),
        ]);
        game.board_mut().push_foods((0..6).into_iter().map(|_| FoodToken).collect());

        assert_eq!(6, game.board().food().len());

        Board::new(&mut game).reveal();

        assert_eq!(1, game.board().food().len());
    }
}
