use std::ops::{Deref, DerefMut};
use game::*;
use interact::*;
use object::*;

pub struct Deal<'a, C: 'a + Chooser>(pub &'a mut Game<C>);

impl<'a, C: Chooser> Deal<'a, C> {
    pub fn deal_species(&mut self) -> Result<(), ()> {
        for player in self.players_mut() {
            if player.domain().len() == 0 {
                debug!("giving player {} a species board", player.id());
                player.domain_mut().add(Placement::Right);
            }
        }
        Ok(())
    }

    pub fn deal_cards(&mut self) -> Result<(), ()> {
        let mut amounts = vec![];
        for player in self.players() {
            amounts.push(3 + player.domain().len())
        }
        let mut players_cards = vec![];
        for amount in amounts {
            let cards = self.deals(amount);
            if cards.len() != amount {
                return Err(())
            }
            players_cards.push(cards);
        }
        for (player, cards) in self.players_mut()
                                   .iter_mut()
                                   .zip(players_cards.into_iter())
        {
            debug!("giving player {} cards: {}", player.id(), cards.len());
            player.push_cards(cards);
        }
        Ok(())
    }
}

impl<'a, C: Chooser> step::Step<C> for Deal<'a, C> {
    fn step(&mut self) -> Result<(), ()> {
        trace!("@Deal.step");
        try!(self.deal_species());
        try!(self.deal_cards());
        Ok(())
    }
}

impl<'a, C: Chooser> Deref for Deal<'a, C> {
    type Target = Game<C>;

    fn deref(&self) -> &Game<C> {
        &self.0
    }
}

impl<'a, C: Chooser> DerefMut for Deal<'a, C> {
    fn deref_mut(&mut self) -> &mut Game<C> {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use game::*;
    use object::*;

    #[test]
    fn deals_no_species_when_all_players_have_some() {
        let mut game = game_with_players(3, &|player| {
            player.domain_mut().add(Placement::Right);
        });
        assert_eq!(1, game.players()[0].domain().len());
        assert_eq!(1, game.players()[1].domain().len());
        assert_eq!(1, game.players()[2].domain().len());

        step::Deal(&mut game).step().unwrap();

        assert_eq!(1, game.players()[0].domain().len());
        assert_eq!(1, game.players()[1].domain().len());
        assert_eq!(1, game.players()[2].domain().len());
    }

    #[test]
    fn deals_a_species_to_only_players_without_one() {
        let mut game = game_with_players(3, &|player| {
            if player.id() != 2 {
                player.domain_mut().add(Placement::Right);
            }
        });

        assert_eq!(1, game.players()[0].domain().len());
        assert_eq!(0, game.players()[1].domain().len());
        assert_eq!(1, game.players()[2].domain().len());

        step::Deal(&mut game).step().unwrap();

        assert_eq!(1, game.players()[0].domain().len());
        assert_eq!(1, game.players()[1].domain().len());
        assert_eq!(1, game.players()[2].domain().len());
    }

    #[test]
    fn deals_cards_based_on_number_of_species() {
        let mut game = game_with_players(3, &|player| {
            for _ in 0..player.id() {
                player.domain_mut().add(Placement::Right);
            }
        });

        assert_eq!(0, game.players()[0].hand().len());
        assert_eq!(0, game.players()[1].hand().len());
        assert_eq!(0, game.players()[2].hand().len());

        step::Deal(&mut game).step().unwrap();

        assert_eq!(4, game.players()[0].hand().len());
        assert_eq!(5, game.players()[1].hand().len());
        assert_eq!(6, game.players()[2].hand().len());
    }

    #[test]
    fn deals_species_before_dealing_cards() {
        let mut game = game_with_players(3, &|_| {});

        assert_eq!(0, game.players()[0].hand().len());
        assert_eq!(0, game.players()[1].hand().len());
        assert_eq!(0, game.players()[2].hand().len());

        step::Deal(&mut game).step().unwrap();

        assert_eq!(4, game.players()[0].hand().len());
        assert_eq!(4, game.players()[1].hand().len());
        assert_eq!(4, game.players()[2].hand().len());
    }

    #[test]
    fn deals_species_with_one_population() {
        let mut game = game_with_players(3, &|_| {});

        assert_eq!(0, game.players()[0].domain().len());
        assert_eq!(0, game.players()[1].domain().len());
        assert_eq!(0, game.players()[2].domain().len());

        step::Deal(&mut game).step().unwrap();

        for player in game.players() {
            assert_eq!(1, player.domain().len());
            assert_eq!(1, player.domain()[0].population());
        }
    }

    #[test]
    fn deal_preserves_player_order() {
        let mut game = game_with_players(3, &|_| {});
        let expected_ids = game.players().iter().map(|p| p.id()).collect::<Vec<_>>();

        step::Deal(&mut game).step().unwrap();

        for (player, id) in game.players().iter().zip(expected_ids.into_iter()) {
            assert_eq!(id, player.id());
        }
    }

    #[test]
    fn deal_cards_breaks_after_cards_exhausted() {
        let mut game = game_with_players(3, &|player| {
            player.domain_mut().add(Placement::Right);
            player.domain_mut().add(Placement::Right);
            player.domain_mut().add(Placement::Right);
        });

        let ref mut got_err = false;
        for _ in 0..32 {
            match step::Deal(&mut game).step() {
                Ok(_) => {}
                Err(_) => {
                    *got_err = true;
                }
            }
        }
        assert!(*got_err);
    }
}
