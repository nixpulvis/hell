use game::*;
use interact::*;

/// A choice of a species to feed.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FeedChoice {
    /// This represents a player's choice to not attack or store.
    Abstain,

    /// Contains the index of the `Species` the player would like to feed.
    /// The species must not be a carnivore.
    Feed(usize),

    /// Contains the index of the `Species` the player would like to store to,
    /// and the number of food tokens they would like to store from the
    /// watering hole. The species must have the trait `FatTissue`.
    Store(usize, u64),

    /// Contains in order, the current player's attacking species index, then
    /// the defending player's index in the current game round order, then the
    /// index of the defending species in the defending player.
    ///
    /// # Exmaples
    ///
    /// ```ignore
    /// Game Players: [id(1), id(2), id(3)]
    ///                       ~~~~~
    ///                       The current player.
    ///
    /// FeedObservation: current_player: id(2),
    ///                  opponents: [id(3), id(1)],
    ///                  wh: _
    ///
    /// FeedChoice: Attack(_, 1, _)
    ///     Current player attacking id(1).
    /// ```
    Attack(usize, usize, usize),
}

impl FeedChoice {
    // TODO: Docs... for real, this method is complex.
    pub fn internalize<C: Chooser>(&mut self, game: &Game<C>) {
        match *self {
            FeedChoice::Attack(_, ref mut target_idx, _) => {
                let idx = game.current_player_idx();
                *target_idx = (idx + *target_idx + 1) % game.players().len();
            }
            _ => {}
        }
    }
}

impl Choice for FeedChoice {}

#[cfg(feature = "wire")]
mod wire;

#[cfg(test)]
mod tests {
    use game::*;
    use interact::*;

    #[test]
    fn internalize_current_player_in_middle_attack_before() {
        let mut game = game_with_players(3, &|_| {});
        game.advance_current_player();

        assert_eq!(1, game.current_player_idx());

        let mut choice = FeedChoice::Attack(0, 1, 0);
        choice.internalize(&game);

        if let FeedChoice::Attack(_, target_idx, _) = choice {
            assert_eq!(0, target_idx);
        } else {
            panic!("didn't get an attack")
        }
    }

    #[test]
    fn internalize_current_player_in_middle_attack_after() {
        let game = game_with_players(3, &|_| {});

        assert_eq!(0, game.current_player_idx());

        let mut choice = FeedChoice::Attack(0, 1, 0);
        choice.internalize(&game);

        if let FeedChoice::Attack(_, target_idx, _) = choice {
            assert_eq!(2, target_idx);
        } else {
            panic!("didn't get an attack")
        }
    }

    #[test]
    fn internalize_current_player_is_last_attack_before() {
        let mut game = game_with_players(3, &|_| {});
        game.advance_current_player();
        game.advance_current_player();

        assert_eq!(2, game.current_player_idx());

        let mut choice = FeedChoice::Attack(0, 1, 0);
        choice.internalize(&game);

        if let FeedChoice::Attack(_, target_idx, _) = choice {
            assert_eq!(1, target_idx);
        } else {
            panic!("didn't get an attack")
        }
    }

    #[test]
    fn internalize_current_player_in_middle_attack_large() {
        let mut game = game_with_players(5, &|_| {});
        game.advance_current_player();
        game.advance_current_player();

        assert_eq!(2, game.current_player_idx());

        let mut choice = FeedChoice::Attack(0, 1, 0);
        choice.internalize(&game);

        if let FeedChoice::Attack(_, target_idx, _) = choice {
            assert_eq!(4, target_idx);
        } else {
            panic!("didn't get an attack")
        }
    }
}
