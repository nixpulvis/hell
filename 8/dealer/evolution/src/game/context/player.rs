use std::ops::{Deref, DerefMut};
use std::collections::HashMap;
use ext::AroundFromMut;
use game::Game;
use game::context::*;
use interact::*;
use object::{CARDS_PER_EXTINCTION, Card, Trait, Player as RealPlayer};

/// A game aware player.
pub struct Player<'a, C: 'a + Chooser>(&'a mut Game<C>, usize);

impl<'a, C: Chooser> Player<'a, C> {
    pub fn new(game: &'a mut Game<C>, player_idx: usize) -> Self {
        Player(game, player_idx)
    }

    /// Give this player `CARDS_PER_EXTINCTION` cards.
    pub fn refund(&mut self) {
        let mut cards = vec![];
        for _ in 0..CARDS_PER_EXTINCTION {
            if let Some(card) = self.context_mut().deal() {
                cards.push(card);
            }
        }
        self.push_cards(cards);
    }

    // Get some cards from a player as a map
    pub fn get_cards(&mut self, card_indices: &mut [usize]) -> HashMap<usize, Card> {
        card_indices.sort();
        card_indices.reverse();
        let mut card_map = HashMap::new();
        for card_index in card_indices {
            card_map.insert(*card_index, self.remove_card(*card_index));
        }
        card_map
    }

    /// Trigger scavenging for all applicable species in this game, starting
    /// with this player.
    pub fn scavenge(&mut self) {
        // TODO: The cool iterator is one over a game yielding our Player and Species.
        let player_idx = self.1;
        let mut scavengers = vec![];
        for (player_idx, player) in self.context_mut().players_mut().around_from_mut(player_idx) {
            for (species_idx, (species, _, _)) in player.domain_mut().into_iter().enumerate() {
                if species.has_trait(Trait::Scavenger) {
                    scavengers.push((player_idx, species_idx));
                }
            }
        }

        for (player_idx, species_idx) in scavengers {
            Species::new(self.context_mut(), (player_idx, species_idx)).feed();
        }
    }
}

impl<'a, C: Chooser> Context<'a, Game<C>> for Player<'a, C> {
    fn context(&self) -> &Game<C> {
        &self.0
    }

    fn context_mut(&mut self) -> &mut Game<C> {
        &mut self.0
    }
}

impl<'a, C: Chooser> Deref for Player<'a, C> {
    type Target = RealPlayer;

    fn deref(&self) -> &Self::Target {
        &self.0.players()[self.1]
    }
}

impl<'a, C: Chooser> DerefMut for Player<'a, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0.players_mut()[self.1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use game::*;
    use object::{FoodToken, Trait, Placement, Card};

    #[test]
    fn refund() {
        let mut game = game_with_players(3, &|_| {});

        assert_eq!(0, game.players()[0].hand().len());

        Player::new(&mut game, 0).refund();

        assert_eq!(2, game.players()[0].hand().len());
    }

    #[test]
    fn get_cards() {
        let mut game = game_with_players(3, &|player| {
            if player.id() == 1 {
                player.push_cards(vec![
                    Card::mock(2, Trait::Ambush),
                    Card::mock(1, Trait::Burrowing),
                    Card::mock(8, Trait::Carnivore),
                ]);
            }
        });

        assert_eq!(3, game.players()[0].hand().len());

        let card_map = Player::new(&mut game, 0).get_cards(&mut vec![0, 2]);

        assert_eq!(1, game.players()[0].hand().len());
        assert_eq!(2, card_map[&0].food_value());
        assert_eq!(Trait::Ambush, card_map[&0].trait_type());
        assert_eq!(8, card_map[&2].food_value());
        assert_eq!(Trait::Carnivore, card_map[&2].trait_type());
    }

    #[test]
    fn scavenge() {
        let mut game = game_with_players(3, &|player| {
            player.domain_mut().add(Placement::Right);
            player.domain_mut()[0].evolve(Trait::Scavenger).unwrap();
        });
        game.board_mut().push_food(FoodToken);
        game.board_mut().push_food(FoodToken);

        assert_eq!(0, game.players()[0].domain()[0].food().len());
        assert_eq!(0, game.players()[1].domain()[0].food().len());
        assert_eq!(0, game.players()[2].domain()[0].food().len());

        Player::new(&mut game, 0).scavenge();

        assert_eq!(1, game.players()[0].domain()[0].food().len());
        assert_eq!(1, game.players()[1].domain()[0].food().len());
        assert_eq!(0, game.players()[2].domain()[0].food().len());
    }
}
