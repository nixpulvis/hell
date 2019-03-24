use game::*;
use interact::*;

impl<'a> Choose<FeedObservation, FeedChoice> for Auto<'a, FeedObservation, FeedChoice> {
    fn choose(&mut self, observation: &FeedObservation) -> Result<Option<FeedChoice>, ()> {
        let all_choices = &observation.choices();
        trace!("all_choices@Choose<FeedChoice>.choose: {:?}", all_choices);

        if all_choices.is_empty() {
            return Ok(None)
        }

        let auto_choice = auto_choice(observation.current_player_index(), &all_choices);
        trace!("auto_choice@Choose<FeedChoice>.choose: {:?}", auto_choice);

        match auto_choice {
            c @ Some(_) => Ok(c),
            None => {
                let external_choice = self.0.choose(observation);
                trace!("external_choice@Choose<FeedChoice>.choose: {:?}", external_choice);
                if let Ok(Some(ref c)) = external_choice {
                    // TODO: <refactor> Should be `try!(external_choice.validate())`.
                    if !all_choices.contains(c) {
                        return Err(())
                    }
                }
                external_choice
            }
        }
    }
}

/// Given the index of a player to ignore attacks to a slice of choices
/// returns on optional choice of an auto attack. The rules for an auto
/// attack are in order:
///
/// 1. Stores the most fat for a single fat tissue species.
/// 2. Feeds a single species.
/// 3. Attacks with a single carnivore to a single target, but does not
/// attack the given player.
//
// NOTE: This function turned out longer and more error prone than expected.
// A design that isn't focused around all possible choices might be better.
// TODO: Shouldn't be pub.
fn auto_choice(pdx: usize, choices: &[FeedChoice]) -> Option<FeedChoice> {
    use interact::FeedChoice::*;

    fn auto_choice_acc(pdx: usize,
                       acc: Option<FeedChoice>,
                       choices: &[FeedChoice]) -> Option<FeedChoice>
    {
        if choices.is_empty() {
            return acc
        }

        let possible = match (acc, choices[0]) {
            (Some(Store(i1, a1)), Store(i2, a2)) => {
                if i1 != i2 { return None } else if a1 > a2 { acc } else { Some(choices[0]) }
            }
            (_, Store(_, _)) => Some(choices[0]),

            (Some(Feed(i1)), Feed(i2)) => {
                if i1 != i2 { return None } else { Some(choices[0]) }
            }
            (Some(Store(_, _)), Feed(_)) => acc,
            (_, Feed(_)) => Some(choices[0]),

            (Some(Attack(_, _, _)), Attack(_, t2, _)) => {
                if t2 == pdx {
                    acc
                } else {
                    return None
                }
            }
            (Some(Store(_, _)), Attack(_, _, _)) |
            (Some(Feed(_)), Attack(_, _, _)) => acc,
            (_, Attack(_, t, _)) => {
                if t == pdx { acc } else { Some(choices[0]) }
            }

            (a, Abstain) => a,
        };
        auto_choice_acc(pdx, possible, &choices[1..])
    }

    auto_choice_acc(pdx, None, choices)
}

#[cfg(test)]
mod tests {
    use game::*;
    use object::*;
    use interact::*;

    #[test]
    fn choose_automatically_chooses_single_fat_food_species() {
        let mut game = game_with_players(3, &|player| {
            if player.id() == 1 {
                player.domain_mut().add(Placement::Right);
                player.domain_mut()[0].evolve(Trait::FatTissue).unwrap();
                player.domain_mut()[0].grow().unwrap();
                player.domain_mut()[0].grow().unwrap();
                player.domain_mut()[0].grow().unwrap();
                player.domain_mut()[0].store(vec![FoodToken]).unwrap();
            }
        });
        game.board_mut().push_foods(vec![FoodToken, FoodToken, FoodToken]);

        let observation: FeedObservation = game.observe();
        let current_index = observation.current_player_index();
        let choice = super::auto_choice(current_index, &observation.choices());

        assert_eq!(Some(FeedChoice::Store(0, 2)), choice);
    }

    #[test]
    fn choose_automatically_chooses_single_vegetarian() {
        let mut game = game_with_players(3, &|player| {
            if player.id() == 1 {
                player.domain_mut().add(Placement::Right);
            }
        });
        game.board_mut().push_food(FoodToken);

        let observation: FeedObservation = game.observe();
        let current_index = observation.current_player_index();
        let choice = super::auto_choice(current_index, &observation.choices());

        assert_eq!(Some(FeedChoice::Feed(0)), choice);
    }

    #[test]
    fn choose_automatically_chooses_single_carnivore_who_can_attack() {
        let mut game = game_with_players(3, &|player| {
            match player.id() {
                1 => {
                    player.domain_mut().add(Placement::Right);
                    player.domain_mut()[0].evolve(Trait::Carnivore).unwrap();
                }
                2 => {
                    player.domain_mut().add(Placement::Right);
                },
                _ => {}
            }
        });
        game.board_mut().push_food(FoodToken);

        let observation: FeedObservation = game.observe();
        let current_index = observation.current_player_index();
        let choice = super::auto_choice(current_index, &observation.choices());

        assert_eq!(Some(FeedChoice::Attack(0, 0, 0)), choice);
    }

    #[test]
    fn choose_automatically_chooses_single_vegetarian_with_a_carnivore() {
        let mut game = game_with_players(3, &|player| {
            if player.id() == 1 {
                player.domain_mut().add(Placement::Right);
                player.domain_mut()[0].evolve(Trait::Carnivore).unwrap();
                player.domain_mut().add(Placement::Right);
            }
        });
        game.board_mut().push_food(FoodToken);

        let observation: FeedObservation = game.observe();
        let current_index = observation.current_player_index();
        let choice = super::auto_choice(current_index, &observation.choices());

        assert_eq!(Some(FeedChoice::Feed(1)), choice);
    }

    #[test]
    fn choose_does_not_automatically_choose_self_attack() {
        let mut game = game_with_players(3, &|player| {
            if player.id() == 1 {
                player.domain_mut().add(Placement::Right);
                player.domain_mut()[0].evolve(Trait::Carnivore).unwrap();
                player.domain_mut().add(Placement::Right);
                player.domain_mut()[1].evolve(Trait::Carnivore).unwrap();
            }
        });
        game.board_mut().push_food(FoodToken);

        let observation: FeedObservation = game.observe();
        let current_index = observation.current_player_index();
        let choice = super::auto_choice(current_index, &observation.choices());

        assert!(choice.is_none());
    }

    #[test]
    fn choose_does_not_automatically_choose_when_current_player_has_multiple_vegetarians() {
        let mut game = game_with_players(3, &|player| {
            if player.id() == 1 {
                player.domain_mut().add(Placement::Right);
                player.domain_mut().add(Placement::Right);
            }
        });
        game.board_mut().push_food(FoodToken);

        let observation: FeedObservation = game.observe();
        let current_index = observation.current_player_index();
        let choice = super::auto_choice(current_index, &observation.choices());

        assert!(choice.is_none());
    }

    #[test]
    fn choose_does_not_automatically_choose_when_current_player_has_multiple_carnivores() {
        let mut game = game_with_players(3, &|player| {
            match player.id() {
                1 => {
                    player.domain_mut().add(Placement::Right);
                    player.domain_mut()[0].evolve(Trait::Carnivore).unwrap();
                    player.domain_mut().add(Placement::Right);
                    player.domain_mut()[1].evolve(Trait::Carnivore).unwrap();
                }
                2 => {
                    player.domain_mut().add(Placement::Right);
                }
                _ => {}
            }
        });
        game.board_mut().push_food(FoodToken);

        let observation: FeedObservation = game.observe();
        let current_index = observation.current_player_index();
        let choice = super::auto_choice(current_index, &observation.choices());

        assert!(choice.is_none());
    }

    #[test]
    fn choose_does_not_automatically_choose_when_current_player_has_multiple_attacks() {
        let mut game = game_with_players(3, &|player| {
            match player.id() {
                1 => {
                    player.domain_mut().add(Placement::Right);
                    player.domain_mut()[0].evolve(Trait::Carnivore).unwrap();
                }
                2 => {
                    player.domain_mut().add(Placement::Right);
                    player.domain_mut().add(Placement::Right);
                },
                _ => {}
            }
        });
        game.board_mut().push_food(FoodToken);

        let observation: FeedObservation = game.observe();
        let current_index = observation.current_player_index();
        let choice = super::auto_choice(current_index, &observation.choices());

        assert!(choice.is_none());
    }

    #[test]
    fn choose_does_not_automatically_choose_with_multiple_attacks_across_multiple_players() {
        let mut game = game_with_players(3, &|player| {
            match player.id() {
                1 => {
                    player.domain_mut().add(Placement::Right);
                    player.domain_mut()[0].evolve(Trait::Carnivore).unwrap();
                }
                _ => {
                    player.domain_mut().add(Placement::Right);
                },
            }
        });
        game.board_mut().push_food(FoodToken);

        let observation: FeedObservation = game.observe();
        let current_index = observation.current_player_index();
        let choice = super::auto_choice(current_index, &observation.choices());

        assert!(choice.is_none());
    }
}
