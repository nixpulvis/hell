use game::*;
use interact::*;

impl<'a> Choose<ActionObservation, ActionChoice> for Auto<'a, ActionObservation, ActionChoice> {
    fn choose(&mut self, observation: &ActionObservation) -> Result<Option<ActionChoice>, ()> {
        let all_choices = &observation.choices();
        trace!("all_choices@Choose<ActionChoice>.choose: {:?}", all_choices);

        let external_choice = self.0.choose(observation);
        trace!("external_choice@Choose<ActionChoice>.choose: {:?}", external_choice);

        if let Ok(Some(ref c)) = external_choice {
            try!(c.validate(&observation));
            // TODO: <needed> Waiting on proper impl of `Choices for ActionObservation`
            // if !all_choices.contains(c) {
            //     return Err(())
            // }
        }
        external_choice
    }
}

#[cfg(test)]
mod tests {
    use game::*;
    use interact::*;
    use object::*;

    #[test]
    fn invalid_choice_is_error() {
        let game = game_with_players(3, &|player| {
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

        assert_eq!(4, game.current_player().hand().len());

        let auto_choice = Auto(&mut choice).choose(&game.observe());

        assert!(auto_choice.is_err());
    }
}
