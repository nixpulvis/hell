use interact::*;
use silly::*;

impl Choose<ActionObservation, ActionChoice> for Silly {
    fn choose(&mut self, observation: &ActionObservation) -> Result<Option<ActionChoice>, ()> {
        let current_player = &observation.current_player;
        let mut cards = current_player.hand().clone().into_iter().enumerate().collect::<Vec<_>>();
        cards.sort_by(|lhs, rhs| {
            lhs.1.cmp(rhs.1)
        });
        let cards = cards.into_iter().map(|(i, _)| i).collect::<Vec<_>>();

        let next_species_idx = current_player.domain().len();
        let food_card_index = cards[0];
        let board_trades = vec![
            BoardTrade {
                card_index: cards[1],
                trait_card_indeces: vec![cards[2]],
            },
        ];
        let population_growths = vec![
            Growth {
                species_index: next_species_idx,
                card_index: cards[3],
            },
        ];
        let body_growths = if cards.len() > 4 {
            vec![
                Growth {
                    species_index: next_species_idx,
                    card_index: cards[4],
                },
            ]
        } else {
            Vec::new()
        };
        let trait_trades = if cards.len() > 5 {
            vec![
                TraitTrade {
                    species_index: next_species_idx,
                    trait_index: 0,
                    replacement_index: cards[5],
                },
            ]
        } else {
            Vec::new()
        };

        Ok(Some(ActionChoice {
            food_card: food_card_index,
            boards: board_trades,
            population_growths: population_growths,
            body_growths: body_growths,
            traits: trait_trades,
        }))
    }
}

#[cfg(test)]
mod tests {
    use game::*;
    use interact::*;
    use object::*;
    use silly::*;

    #[test]
    fn action_choice_selection() {
        // comments interleaved to get my point across
        let game = game_with_players(3, &|player| {
            if player.id() == 1 {
                // so player (index 0) has two species (index 0, 1) - modifications should be made to
                // the new species (index 2)
                player.domain_mut().add(Placement::Right);
                player.domain_mut().add(Placement::Right);

                // these are purposely out of order, because the protocol says the "silly strategy"
                // should select cards in-order; but I imagine it's not always the case that the dealer
                //  should be handing them out in-order, so this ensures we sort here (even if we know)
                //  the dealer hands them out in-order right now
                player.push_cards(vec![
                    // I tried to make this as understandble as possible while still trying to trip up
                    // the Silly selection algorithm. Below LETTERS represent the temporal
                    // ordering cards should be used in, number in parentheses simply denote the index
                    // I'm expecting to see appear in the ActionChoice.
                    Card::mock(2, Trait::FatTissue),    // C. applied to the board          (index 0)
                    Card::mock(3, Trait::Ambush),       // B. traded for a species board    (index 1)
                    Card::mock(2, Trait::Ambush),       // A. food card selection           (index 2)
                    Card::mock(-3, Trait::Horns),       // F. swapped into first trait spot (index 3)
                    Card::mock(0, Trait::WarningCall),  // G. saved for later               (index 4)
                    Card::mock(2, Trait::HardShell),    // E. used to buff body size        (index 5)
                    Card::mock(2, Trait::Fertile),      // D. used to buff population       (index 6)
                ]);
            }
        });

        let action_observation = ActionObservation {
            current_player: game.current_player().clone(),
            before: vec![],
            after: vec![],
        };
        let action_choice = Silly.choose(&action_observation).unwrap().unwrap();

        assert_eq!(2, action_choice.food_card);
        assert_eq!(1, action_choice.boards[0].card_index);
        assert_eq!(0, action_choice.boards[0].trait_card_indeces[0]);
        assert_eq!(6, action_choice.population_growths[0].card_index);
        assert_eq!(5, action_choice.body_growths[0].card_index);
        assert_eq!(3, action_choice.traits[0].replacement_index);
    }
}
