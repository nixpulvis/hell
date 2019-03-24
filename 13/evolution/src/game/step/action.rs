use std::ops::{Deref, DerefMut};
use game::*;
use object::*;
use interact::*;

/// Players **must** give a card as food, and can choose to give any number
/// more cards to upgrade their species.
pub struct Action<'a, C: 'a + Chooser>(pub &'a mut Game<C>, pub &'a mut Choose<ActionObservation, ActionChoice>);

impl<'a, C: Chooser> Action<'a, C> {
    fn apply(&mut self, choice: ActionChoice) {
        let ActionChoice {
            food_card,
            population_growths,
            body_growths,
            boards,
            traits,
        } = choice;

        // create a vector of all card indices to remove from the player
        let mut indices = vec![food_card];
        for &Growth { card_index, .. } in population_growths.iter().chain(body_growths.iter()) {
            indices.push(card_index);
        }
        for &BoardTrade { card_index, ref trait_card_indeces } in boards.iter() {
            indices.push(card_index);
            for trait_card_index in trait_card_indeces {
                indices.push(*trait_card_index);
            }
        }
        for &TraitTrade { replacement_index, .. } in traits.iter() {
            indices.push(replacement_index);
        }

        let idx = self.current_player_idx();

        let mut card_map = {
            context::Player::new(self, idx).get_cards(indices.as_mut_slice())
        };

        let food_card = card_map.remove(&food_card).expect("couldn't get food card");
        self.board_mut().add_card(food_card);

        let mut player = context::Player::new(self, idx);

        // iterate and apply choices
        // give species boards next so the domain indices are valid for other actions
        for BoardTrade { trait_card_indeces, .. } in boards {
            let mut species = player.domain_mut()
                                    .add(Placement::Right);
            for ref trait_card_index in trait_card_indeces {
                species.evolve(card_map.remove(trait_card_index).expect("couldn't get trait card").trait_type()).expect("couldn't evolve");
            }
        }
        for Growth { card_index, species_index } in population_growths {
            card_map.remove(&card_index).expect("couldn't get population card");
            player.domain_mut()[species_index].breed().expect("couldn't breed");
        }
        for Growth { card_index, species_index } in body_growths {
            card_map.remove(&card_index).expect("couldn't get body card");
            player.domain_mut()[species_index].grow().expect("couldn't grow");
        }
        for TraitTrade { species_index, trait_index, replacement_index } in traits {
            let new_trait = card_map.remove(&replacement_index).expect("couldn't get replacement trait").trait_type();
            player.domain_mut()[species_index].exchange_trait(trait_index, new_trait).expect("couldn't swap trait");
        }
    }
}

impl<'a, C: Chooser> step::Step<C> for Action<'a, C> {
    fn step(&mut self) -> Result<(), ()> {
        let observation = self.observe();
        match self.1.choose(&observation) {
            Ok(Some(c)) => {
                debug!("applying choice: {:?}", c);
                self.apply(c);
                // TODO: <needed> Should be:
                // match self.apply(c) {
                //     Ok(...) => { ... }
                //     Err(_) => { panic!("<blame game>") }
                // }
                self.advance_current_player();
                Ok(())
            }
            Ok(None) | Err(_) => {
                self.eject_current_player();
                Ok(())
            }
        }
    }
}

impl<'a, C: Chooser> Deref for Action<'a, C> {
    type Target = Game<C>;

    fn deref(&self) -> &Game<C> {
        &self.0
    }
}

impl<'a, C: Chooser> DerefMut for Action<'a, C> {
    fn deref_mut(&mut self) -> &mut Game<C> {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use game::*;
    use interact::*;
    use object::*;

    // TODO: Move the apply logic into it's own function.
    fn todo() -> ActionChoice {
        ActionChoice {
            food_card: 0,
            population_growths: vec![],
            body_growths: vec![],
            boards: vec![],
            traits: vec![],
        }
    }

    #[test]
    fn apply_puts_food_card_on_board() {
        let mut game = game_with_players(3, &|player| {
            if player.id() == 1 {
                player.push_card(Card::mock(4, Trait::Burrowing));
            }
        });

        assert_eq!(1, game.players()[0].hand().len());

        step::Action(&mut game, &mut todo()).apply(ActionChoice {
            food_card: 0,
            population_growths: vec![],
            body_growths: vec![],
            boards: vec![],
            traits: vec![],
        });

        assert_eq!(0, game.players()[0].hand().len());
        assert_eq!(1, game.board().cards().expect("no cards on board").len());
    }

    #[test]
    fn apply_grows_populations() {
        let mut game = game_with_players(3, &|player| {
            if player.id() == 1 {
                player.push_cards(vec![
                    Card::mock(1, Trait::Scavenger),
                    Card::mock(1, Trait::Scavenger),
                ]);
                player.domain_mut().add(Placement::Right);
            }
        });

        assert_eq!(2, game.players()[0].hand().len());

        step::Action(&mut game, &mut todo()).apply(ActionChoice {
            food_card: 0,
            population_growths: vec![
                Growth {
                    species_index: 0,
                    card_index: 1,
                },
            ],
            body_growths: vec![],
            boards: vec![],
            traits: vec![],
        });

        assert_eq!(0, game.players()[0].hand().len());
        assert_eq!(2, game.players()[0].domain()[0].population());
    }

    #[test]
    fn apply_grows_body() {
        let mut game = game_with_players(3, &|player| {
            if player.id() == 1 {
                player.push_cards(vec![
                    Card::mock(1, Trait::Scavenger),
                    Card::mock(1, Trait::Scavenger),
                ]);
                player.domain_mut().add(Placement::Right);
            }
        });

        assert_eq!(2, game.players()[0].hand().len());

        step::Action(&mut game, &mut todo()).apply(ActionChoice {
            food_card: 0,
            population_growths: vec![],
            body_growths: vec![
                Growth {
                    species_index: 0,
                    card_index: 1,
                },
            ],
            boards: vec![],
            traits: vec![],
        });

        assert_eq!(0, game.players()[0].hand().len());
        assert_eq!(1, game.players()[0].domain()[0].body_size());
    }

    #[test]
    fn apply_trade_for_board() {
        let mut game = game_with_players(3, &|player| {
            if player.id() == 1 {
                player.push_cards(vec![
                    Card::mock(1, Trait::Ambush),
                    Card::mock(1, Trait::Burrowing),
                    Card::mock(1, Trait::Carnivore),
                    Card::mock(1, Trait::Climbing),
                    Card::mock(1, Trait::Cooperation),
                ]);
                player.domain_mut().add(Placement::Right);
            }
        });

        assert_eq!(5, game.players()[0].hand().len());

        step::Action(&mut game, &mut todo()).apply(ActionChoice{
            food_card: 0,
            population_growths: vec![],
            body_growths: vec![],
            boards: vec![
                BoardTrade {
                    card_index: 1,
                    trait_card_indeces: vec![2, 3, 4]
                }
            ],
            traits: vec![],
        });

        assert_eq!(0, game.players()[0].hand().len());
        assert!(game.players()[0].domain()[1].has_trait(Trait::Carnivore));
        assert!(game.players()[0].domain()[1].has_trait(Trait::Climbing));
        assert!(game.players()[0].domain()[1].has_trait(Trait::Cooperation));
    }

    #[test]
    fn apply_trait_swap() {
        let mut game = game_with_players(3, &|player| {
            if player.id() == 1 {
                player.push_cards(vec![
                    Card::mock(1, Trait::Ambush),
                    Card::mock(1, Trait::Burrowing),
                ]);
                player.domain_mut().add(Placement::Right);
                player.domain_mut()[0].evolve(Trait::Carnivore).unwrap();
            }
        });

        assert_eq!(2, game.players()[0].hand().len());
        assert!(game.players()[0].domain()[0].has_trait(Trait::Carnivore));

        step::Action(&mut game, &mut todo()).apply(ActionChoice{
            food_card: 0,
            population_growths: vec![],
            body_growths: vec![],
            boards: vec![],
            traits: vec![
                TraitTrade {
                    species_index: 0,
                    trait_index: 0,
                    replacement_index: 1,
                }
            ],
        });

        assert_eq!(0, game.players()[0].hand().len());
        assert!(game.players()[0].domain()[0].has_trait(Trait::Burrowing));
        assert!(!game.players()[0].domain()[0].has_trait(Trait::Carnivore));
    }

    #[test]
    fn step_kicks_on_invalid_actions() {
        let mut game = game_with_players(3, &|player| {
            if player.id() == 1 {
                player.domain_mut().add(Placement::Right);
                player.push_cards(vec![
                    Card::mock(2, Trait::Ambush),
                    Card::mock(1, Trait::Burrowing),
                    Card::mock(2, Trait::Climbing),
                    Card::mock(4, Trait::Carnivore),
                ]);
            }
        });
        let mut choice = ActionChoice {
            food_card: 0,
            population_growths: vec![
                Growth {
                    species_index: 0,
                    card_index: 1,
                },
            ],
            body_growths: vec![
                Growth {
                    species_index: 1,
                    card_index: 2,
                }
            ],
            boards: vec![],
            traits: vec![
                TraitTrade {
                    species_index: 0,
                    trait_index: 0,
                    replacement_index: 3,
                }
            ],
        };
        let first_player_id = game.current_player().id();

        assert_eq!(4, game.current_player().hand().len());
        assert_eq!(3, game.players().len());

        step::Action(&mut game, &mut Auto(&mut choice)).step().unwrap();

        assert_eq!(2, game.players().len());
        for id in game.players().iter().map(|p| p.id()) {
            assert!(id != first_player_id);
        }
    }
}
