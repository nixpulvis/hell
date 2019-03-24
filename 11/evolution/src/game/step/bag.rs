use std::ops::{Deref, DerefMut};
use interact::*;
use game::*;

/// cull all of the species in the game, giving 2 cards to each player
/// per species that went extinct. Then bag the food for all players.
pub struct Bag<'a, C: 'a + Chooser>(pub &'a mut Game<C>);

impl<'a, C: Chooser> step::Step<C> for Bag<'a, C> {
    fn step(&mut self) -> Result<(), ()> {
        trace!("@Bag.step");

        let game: *mut Game<C> = self.0;
        unsafe {
            for player in (*game).players_mut() {
                let extinctions = try!(player.domain_mut().cull());
                let cards = (*game).deals(extinctions * 2);
                player.push_cards(cards);
                player.bag_food();
            }
        }

        self.advance_starting_player();
        Ok(())
    }
}

impl<'a, C: Chooser> Deref for Bag<'a, C> {
    type Target = Game<C>;

    fn deref(&self) -> &Game<C> {
        &self.0
    }
}

impl<'a, C: Chooser> DerefMut for Bag<'a, C> {
    fn deref_mut(&mut self) -> &mut Game<C> {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use game::*;
    use object::*;

    #[test]
    fn bag_kills_off_starving_species() {
        let mut game = game_with_players(3, &|player| {
            player.domain_mut().add(Placement::Right);
            player.domain_mut()[0].breed().unwrap();
            player.domain_mut()[0].eat(FoodToken).unwrap();
        });

        for player in game.players() {
            assert_eq!(2, player.domain()[0].population());
            assert_eq!(1, player.domain()[0].food().len());
        }

        step::Bag(&mut game).step().unwrap();

        for player in game.players() {
            assert_eq!(1, player.domain()[0].population());
        }
    }

    #[test]
    fn bag_makes_species_go_extinct() {
        let mut game = game_with_players(3, &|player| {
            player.domain_mut().add(Placement::Right);
        });

        for player in game.players() {
            assert_eq!(1, player.domain()[0].population());
            assert_eq!(0, player.hand().len());
        }

        step::Bag(&mut game).step().unwrap();

        for player in game.players() {
            assert_eq!(0, player.domain().len());
            assert_eq!(2, player.hand().len());
        }
    }

    #[test]
    fn bag_moves_food_tokens_to_players_bags() {
        let mut game = game_with_players(3, &|player| {
            player.domain_mut().add(Placement::Right);
            player.domain_mut().add(Placement::Right);
            player.domain_mut()[0].eat(FoodToken).unwrap();
            player.domain_mut()[1].eat(FoodToken).unwrap();
        });

        for player in game.players() {
            assert_eq!(0, player.bag().len());
            assert_eq!(1, player.domain()[0].food().len());
            assert_eq!(1, player.domain()[1].food().len());
        }

        step::Bag(&mut game).step().unwrap();

        for player in game.players() {
            assert_eq!(2, player.bag().len());
            assert_eq!(0, player.domain()[0].food().len());
            assert_eq!(0, player.domain()[1].food().len());
        }
    }

    // #[test]
    // fn bag() {
    //     let mut game = game_with_players(3, &|player| {
    //         player.domain_mut().add(Placement::Right);
    //         player.domain_mut().add(Placement::Right);
    //     });
    //
    //     for player in game.players() {
    //         assert_eq!(1, player.domain()[0].population());
    //         assert_eq!(0, player.domain()[0].food().len());
    //         assert_eq!(1, player.domain()[1].population());
    //         assert_eq!(0, player.domain()[1].food().len());
    //         assert_eq!(0, player.hand().len());
    //     }
    //
    //     println!("hit");
    //
    //     game.bag();
    //
    //     for player in game.players() {
    //         println!("hitd");
    //         assert_eq!(0, player.domain().len());
    //         assert_eq!(4, player.hand().len());
    //     }
    // }
}
