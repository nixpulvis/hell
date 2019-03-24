use std::ops::{Deref, DerefMut};
use interact::*;
use game::*;

pub struct Reveal<'a, C: 'a + Chooser>(pub &'a mut Game<C>);

impl<'a, C: Chooser> step::Step<C> for Reveal<'a, C> {
    fn step(&mut self) -> Result<(), ()> {
        trace!("@Reveal.step");
        context::Board::new(self).reveal();
        Ok(())
    }
}

impl<'a, C: Chooser> Deref for Reveal<'a, C> {
    type Target = Game<C>;

    fn deref(&self) -> &Game<C> {
        &self.0
    }
}

impl<'a, C: Chooser> DerefMut for Reveal<'a, C> {
    fn deref_mut(&mut self) -> &mut Game<C> {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use game::*;
    use object::*;

    #[test]
    fn reveal_preserves_player_order() {
        let mut game = game_with_players(3, &|_| {});
        let expected_ids = game.players().iter().map(|p| p.id()).collect::<Vec<_>>();
        let cards = game.deals(3);
        game.board_mut().set_cards(cards);

        step::Reveal(&mut game).step().unwrap();

        for (player, id) in game.players().iter().zip(expected_ids.into_iter()) {
            assert_eq!(id, player.id());
        }
    }

    #[test]
    fn reveal_converts_cards_into_food() {
        let mut game = game_with_players(3, &|_| {});
        let food_cards = vec![
            Card::mock(3, Trait::Burrowing),
            Card::mock(3, Trait::Burrowing),
            Card::mock(3, Trait::Burrowing),
        ];
        game.board_mut().set_cards(food_cards);

        assert_eq!(0, game.board().food().len());
        assert_eq!(3, game.board().cards().unwrap().len());

        step::Reveal(&mut game).step().unwrap();

        assert_eq!(9, game.board().food().len());
    }

    #[test]
    fn reveal_removes_food_if_negative() {
        let mut game = game_with_players(3, &|_| {});
        let food_cards = vec![
            Card::mock(-2, Trait::Carnivore),
            Card::mock(0, Trait::Carnivore),
            Card::mock(0, Trait::Carnivore),
        ];
        game.board_mut().push_food(FoodToken);
        game.board_mut().set_cards(food_cards);

        assert_eq!(1, game.board().food().len());

        step::Reveal(&mut game).step().unwrap();

        assert_eq!(0, game.board().food().len());
    }

    #[test]
    fn reveal_grows_fertile_species() {
        let mut game = game_with_players(3, &|player| {
            player.domain_mut().add(Placement::Right);
            if player.id() == 2 {
                player.domain_mut()[0].evolve(Trait::Fertile).unwrap();
            }
        });
        let cards = game.deals(3);
        game.board_mut().set_cards(cards);

        assert_eq!(1, game.players()[0].domain()[0].population());
        assert_eq!(1, game.players()[1].domain()[0].population());
        assert_eq!(1, game.players()[2].domain()[0].population());

        step::Reveal(&mut game).step().unwrap();

        assert_eq!(1, game.players()[0].domain()[0].population());
        assert_eq!(2, game.players()[1].domain()[0].population());
        assert_eq!(1, game.players()[2].domain()[0].population());
    }

    #[test]
    fn reveal_feeds_long_necks_with_food() {
        let mut game = game_with_players(3, &|player| {
            player.domain_mut().add(Placement::Right);
            if player.id() == 2 {
                player.domain_mut()[0].evolve(Trait::LongNeck).unwrap();
                player.domain_mut()[0].evolve(Trait::Cooperation).unwrap();
                player.domain_mut().add(Placement::Right);
            }
        });
        game.board_mut().push_foods((0..50).into_iter().map(|_| FoodToken).collect());
        let cards = game.deals(3);
        game.board_mut().set_cards(cards);

        assert_eq!(0, game.players()[0].domain()[0].food().len());
        assert_eq!(0, game.players()[1].domain()[0].food().len());
        assert_eq!(0, game.players()[1].domain()[1].food().len());
        assert_eq!(0, game.players()[2].domain()[0].food().len());

        step::Reveal(&mut game).step().unwrap();

        assert_eq!(0, game.players()[0].domain()[0].food().len());
        assert_eq!(1, game.players()[1].domain()[0].food().len());
        assert_eq!(1, game.players()[1].domain()[1].food().len());
        assert_eq!(0, game.players()[2].domain()[0].food().len());
    }

    #[test]
    fn reveal_digests_fat() {
        let mut game = game_with_players(3, &|player| {
            player.domain_mut().add(Placement::Right);
            if player.id() == 2 {
                player.domain_mut()[0].evolve(Trait::FatTissue).unwrap();
                player.domain_mut()[0].grow().unwrap();
                player.domain_mut()[0].grow().unwrap();
                player.domain_mut()[0].store(vec![FoodToken, FoodToken]).unwrap();
            }
        });
        let cards = game.deals(3);
        game.board_mut().set_cards(cards);

        assert_eq!(0, game.players()[0].domain()[0].food().len());
        assert_eq!(0, game.players()[1].domain()[0].food().len());
        assert_eq!(0, game.players()[2].domain()[0].food().len());

        step::Reveal(&mut game).step().unwrap();

        assert_eq!(0, game.players()[0].domain()[0].food().len());
        assert_eq!(1, game.players()[1].domain()[0].food().len());
        assert_eq!(1, game.players()[1].domain()[0].fat().len());
        assert_eq!(0, game.players()[2].domain()[0].food().len());
    }
}
