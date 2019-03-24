use std::cmp;
use object::*;
use interact::*;

impl Choices<FeedChoice> for FeedObservation {
    fn choices(&self) -> Vec<FeedChoice> {
        self.visit_feed_choices(&mut |c| Some(c))
    }
}

impl FeedObservation {
    pub fn len(&self) -> usize {
        1 + self.opponents.len()
    }

    pub fn current_player_index(&self) -> usize {
        self.opponents.len()
    }

    fn visit_feed_choices<T>(&self, selector: &mut FnMut(FeedChoice) -> Option<T>) -> Vec<T> {
        let mut selections = vec![];

        if self.board.food == 0 {
            return selections
        }

        let mut abstain_added = false;
        'species_loop: for (species_index, (species, _, _)) in self.current_player
                                                                   .domain()
                                                                   .into_iter()
                                                                   .enumerate()
        {
            if let Some(fat_food) = species.can_store() {
                if !abstain_added {
                    if let Some(selection) = selector(FeedChoice::Abstain) {
                        selections.push(selection);
                        abstain_added = true;
                    } else {
                        break 'species_loop;
                    }
                }
                let amount = cmp::min(fat_food, self.board.food);
                for i in 1..(amount + 1) {
                    if let Some(selection) = selector(FeedChoice::Store(species_index, i)) {
                        selections.push(selection);
                    } else {
                        break 'species_loop;
                    }
                }
            }

            if !species.can_eat() {
                continue 'species_loop;
            }

            if species.has_trait(Trait::Carnivore) {
                if !abstain_added {
                    if let Some(selection) = selector(FeedChoice::Abstain) {
                        selections.push(selection);
                        abstain_added = true;
                    } else {
                        break 'species_loop;
                    }
                }
                for (target_player_index, target_player) in self.opponents.iter().enumerate() {
                    for (defender_index, (defender, left, right)) in target_player.domain.into_iter().enumerate() {
                        if !species.can_attack(defender, left, right) {
                            continue;
                        }

                        if let Some(selection) = selector(FeedChoice::Attack(species_index, target_player_index, defender_index)) {
                            selections.push(selection);
                        } else {
                            break 'species_loop;
                        }
                    }
                }
                for (defender_index, (defender, left, right)) in self.current_player.domain().into_iter().enumerate() {
                    if !species.can_attack(defender, left, right) || defender_index == species_index {
                        continue;
                    }

                    if let Some(selection) = selector(FeedChoice::Attack(species_index, self.current_player_index(), defender_index)) {
                        selections.push(selection);
                    } else {
                        break 'species_loop;
                    }
                }
            } else {
                if let Some(selection) = selector(FeedChoice::Feed(species_index)) {
                    selections.push(selection);
                } else {
                    break 'species_loop;
                }
            }

        }
        selections
    }
}

// TODO: Beef these tests way up. They are now used for validation.
#[cfg(test)]
mod tests {
    use game::*;
    use interact::*;
    use object::*;

    #[test]
    fn size_of_all_feed_choices_visited() {
        let mut game = game_with_players(3, &|player| {
            if player.id() == 1 {
                player.domain_mut().add(Placement::Right);
                player.domain_mut().add(Placement::Right);
                player.domain_mut()[0].evolve(Trait::FatTissue).unwrap();
                player.domain_mut()[0].grow().unwrap();
                player.domain_mut()[0].grow().unwrap();
                player.domain_mut()[0].grow().unwrap();
            }
        });
        game.board_mut().push_foods(vec![FoodToken, FoodToken]);

        assert_eq!(2, game.board().food().len());
        assert!(game.players()[0].domain()[0].can_eat());
        assert!(game.players()[0].domain()[1].can_eat());
        assert_eq!(Some(3), game.players()[0].domain()[0].can_store());

        let observation: FeedObservation = game.observe();
        let choices = observation.visit_feed_choices(&mut |c| Some(c));

        assert_eq!(5, choices.len());
    }

    #[test]
    fn feed_choices_with_empty_watering_hole() {
        let game = game_with_players(3, &|player| {
            if player.id() == 3 {
                player.domain_mut().add(Placement::Right);
            }
        });

        assert_eq!(0, game.board().food().len());

        let observation: FeedObservation = game.observe();
        let choices = observation.visit_feed_choices(&mut |c| Some(c));

        assert_eq!(Vec::<FeedChoice>::new(), choices);
    }

    #[test]
    fn feed_choices_with_no_species() {
        let mut game = game_with_players(3, &|_| {});
        game.board_mut().push_foods(vec![FoodToken, FoodToken]);

        assert_eq!(2, game.board().food().len());

        let observation: FeedObservation = game.observe();
        let choices = observation.visit_feed_choices(&mut |c| Some(c));

        assert_eq!(Vec::<FeedChoice>::new(), choices);
    }

    #[test]
    fn no_store_choices_for_more_food_than_available() {
        let mut game = game_with_players(3, &|player| {
            if player.id() == 1 {
                player.domain_mut().add(Placement::Right);
                player.domain_mut()[0].evolve(Trait::FatTissue).unwrap();
                player.domain_mut()[0].grow().unwrap();
                player.domain_mut()[0].grow().unwrap();
            }
        });
        game.board_mut().push_foods(vec![FoodToken, FoodToken]);

        assert_eq!(2, game.board().food().len());
        assert_eq!(Some(2), game.players()[0].domain()[0].can_store());

        let observation: FeedObservation = game.observe();
        let stores = observation.visit_feed_choices(&mut |c| Some(c)).into_iter().filter(|&c| {
            match c {
                FeedChoice::Store(_, _) => true,
                _ => false,
            }
        }).collect::<Vec<_>>();

        assert_eq!(2, stores.len());
        assert!(stores.contains(&FeedChoice::Store(0, 1)));
        assert!(stores.contains(&FeedChoice::Store(0, 2)));
    }
}
