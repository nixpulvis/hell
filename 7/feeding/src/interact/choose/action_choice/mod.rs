use interact::*;
use std::collections::HashSet;

/// A choice how to use cards for a round.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ActionChoice {
    pub food_card: usize,
    pub population_growths: Vec<Growth>,
    pub body_growths: Vec<Growth>,
    pub boards: Vec<BoardTrade>,
    pub traits: Vec<TraitTrade>,
}

impl Choice for ActionChoice {}

impl ActionChoice {
    pub fn validate(&self, observation: &ActionObservation) -> Result<(), ()> {
        if self.is_valid(observation) {
            Ok(())
        } else {
            Err(())
        }
    }

    fn is_valid(&self, observation: &ActionObservation) -> bool {
        let mut domain = (**observation.current_player.domain())
            .into_iter()
            .map(|species| species.traits().len())
            .collect::<Vec<_>>();
        let mut hand: HashSet<usize> = (0..observation.current_player.hand().len()).collect();

        try_bool!(hand.remove(&self.food_card));

        for bt in self.boards.iter() {
            try_bool!(bt.validate(&mut domain, &mut hand));
        }

        for g in self.population_growths.iter().chain(self.body_growths.iter()) {
            try_bool!(g.validate(&domain, &mut hand));
        }

        for t in self.traits.iter() {
            try_bool!(t.validate(&domain, &mut hand));
        }

        true
    }
}

/// A choice to grow a species in either body size or population by trading a card.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Growth {
    pub species_index: usize,
    pub card_index: usize,
}

impl Growth {
    fn validate(&self, domain: &[usize], hand: &mut HashSet<usize>) -> bool {
        try_bool!(hand.remove(&self.card_index), self.species_index < domain.len());
        true
    }
}

/// A choice to trade a card for a species board, and optionally apply traits to it.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BoardTrade {
    pub card_index: usize,
    pub trait_card_indeces: Vec<usize>,
}

impl BoardTrade {
    fn validate(&self, domain: &mut Vec<usize>, hand: &mut HashSet<usize>) -> bool {
        try_bool!(hand.remove(&self.card_index));
        for i in self.trait_card_indeces.iter() {
            try_bool!(hand.remove(&i));
        }
        domain.push(self.trait_card_indeces.len());
        true
    }
}

/// A choice to swap out a trait on a species for a new one.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TraitTrade {
    pub species_index: usize,
    pub trait_index: usize,
    pub replacement_index: usize,
}

impl TraitTrade {
    fn validate(&self, domain: &[usize], hand: &mut HashSet<usize>) -> bool {
        try_bool!(
            hand.remove(&self.replacement_index),
            self.species_index < domain.len(),
            self.trait_index < domain[self.species_index]
        );
        true
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::iter::FromIterator;
    use interact::*;
    use object::*;

    #[test]
    fn growth_validation_good() {
        let growth = Growth {
            species_index: 2,
            card_index: 3,
        };
        let domain = (0..4).collect::<Vec<_>>();
        let mut hand = HashSet::from_iter(0..4);
        assert!(growth.validate(&domain, &mut hand));
    }

    #[test]
    fn growth_validation_bad_index() {
        let growth = Growth {
            species_index: 5,
            card_index: 2,
        };
        let domain = (0..2).collect::<Vec<_>>();
        let mut hand = HashSet::from_iter(0..4);
        assert!(!growth.validate(&domain, &mut hand));
    }

    #[test]
    fn board_validation_good() {
        let trade = BoardTrade {
            card_index: 3,
            trait_card_indeces: vec![1, 4],
        };
        let mut domain = (0..3).collect::<Vec<_>>();
        let mut hand = HashSet::from_iter(0..5);

        assert_eq!(3, domain.len());

        assert!(trade.validate(&mut domain, &mut hand));

        assert_eq!(4, domain.len());
    }

    #[test]
    fn board_validation_bad_index() {
        let trade = BoardTrade {
            card_index: 14,
            trait_card_indeces: vec![1, 2, 3],
        };
        let mut domain = (0..3).collect::<Vec<_>>();
        let mut hand = HashSet::from_iter(0..6);

        assert!(!trade.validate(&mut domain, &mut hand));
    }

    #[test]
    fn trait_validation_good() {
        let trade = TraitTrade {
            species_index: 1,
            trait_index: 1,
            replacement_index: 2,
        };
        let domain = vec![0, 3, 0, 0];
        let mut hand = HashSet::from_iter(0..5);
        assert!(trade.validate(&domain, &mut hand));
    }

    #[test]
    fn trait_validation_bad_card() {
        let trade = TraitTrade {
            species_index: 1,
            trait_index: 0,
            replacement_index: 14,
        };
        let domain = (0..3).collect::<Vec<_>>();
        let mut hand = HashSet::from_iter(0..5);
        assert!(!trade.validate(&domain, &mut hand));
    }

    #[test]
    fn invalid_action_is_error() {
        let action_choice = ActionChoice {
            food_card: 0,
            population_growths: vec![],
            body_growths: vec![],
            boards: vec![BoardTrade {
                card_index: 1,
                trait_card_indeces: vec![0, 1, 2],
            }],
            traits: vec![],
        };
        let mut action_observation = ActionObservation {
            current_player: Player::new(2),
            before: vec![],
            after: vec![],
        };
        let cards = Card::deck().into_iter().take(5).collect();
        action_observation.current_player.push_cards(cards);
        assert_eq!(5, action_observation.current_player.hand().len());

        assert!(action_choice.validate(&action_observation).is_err());
    }

    #[test]
    fn duplicate_index_action_is_invalid() {
        let action_choice = ActionChoice {
            food_card: 0,
            population_growths: vec![],
            body_growths: vec![],
            boards: vec![BoardTrade {
                card_index: 1,
                trait_card_indeces: vec![0, 1, 2],
            }],
            traits: vec![],
        };
        let mut action_observation = ActionObservation {
            current_player: Player::new(2),
            before: vec![],
            after: vec![],
        };
        let cards = Card::deck().into_iter().take(5).collect();
        action_observation.current_player.push_cards(cards);
        assert_eq!(5, action_observation.current_player.hand().len());

        assert!(!action_choice.is_valid(&action_observation));
    }

    #[test]
    fn out_of_bounds_action_is_invalid() {
        let action_choice = ActionChoice {
            food_card: 0,
            population_growths: vec![],
            body_growths: vec![],
            boards: vec![BoardTrade {
                card_index: 1,
                trait_card_indeces: vec![2, 3, 10],
            }],
            traits: vec![],
        };
        let mut action_observation = ActionObservation {
            current_player: Player::new(2),
            before: vec![],
            after: vec![],
        };
        let cards = Card::deck().into_iter().take(5).collect();
        action_observation.current_player.push_cards(cards);
        assert_eq!(5, action_observation.current_player.hand().len());

        assert!(!action_choice.is_valid(&action_observation));
    }

    #[test]
    fn added_species_action_is_valid() {
        let action_choice = ActionChoice {
            food_card: 0,
            population_growths: vec![],
            body_growths: vec![],
            boards: vec![BoardTrade {
                card_index: 1,
                trait_card_indeces: vec![2, 3],
            }],
            traits: vec![
                TraitTrade {
                    species_index: 1,
                    trait_index: 1,
                    replacement_index: 4,
                },
            ],
        };
        let mut action_observation = ActionObservation {
            current_player: Player::new(2),
            before: vec![],
            after: vec![],
        };
        let cards = Card::deck().into_iter().take(5).collect();
        action_observation.current_player.push_cards(cards);
        action_observation.current_player.domain_mut().add(Placement::Right);
        assert_eq!(5, action_observation.current_player.hand().len());
        assert_eq!(1, action_observation.current_player.domain().len());

        assert!(action_choice.is_valid(&action_observation));
    }
}

#[cfg(feature = "wire")]
mod wire;
