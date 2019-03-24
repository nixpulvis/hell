use std::collections::HashSet;
use evolution_wire as wire;
use interact::*;

// TODO: <refactor> To/From wire, should live in the game module, also it should
// pass itself to assist validation.

impl wire::FromWire<wire::remote::Action4> for ActionChoice {
    fn from_wire(wire: wire::remote::Action4) -> Result<Self, ()> {
        let action_choice = ActionChoice {
            food_card: *wire.0 as usize,
            population_growths: try!(Vec::from_wire(wire.1)),
            body_growths: try!(Vec::from_wire(wire.2)),
            boards: try!(Vec::from_wire(wire.3)),
            traits: try!(Vec::from_wire(wire.4)),
        };

        if has_duplicate_cards(&action_choice) {
            Err(())
        } else {
            Ok(action_choice)
        }
    }
}

impl wire::ToWire<wire::remote::Action4> for ActionChoice {
    fn to_wire(&self) -> wire::remote::Action4 {
        wire::remote::Action4 (
            self.food_card.to_wire(),
            self.population_growths.iter().map(|g| g.to_wire()).collect(),
            self.body_growths.iter().map(|g| g.to_wire()).collect(),
            self.boards.iter().map(|b| b.to_wire()).collect(),
            self.traits.iter().map(|t| t.to_wire()).collect(),
        )
    }
}

impl wire::FromWire<wire::remote::GP> for Growth {
    fn from_wire(wire: wire::remote::GP) -> Result<Self, ()> {
        let species_index = *wire.board_index as usize;
        let card_index = *wire.card_index as usize;

        Ok(Growth {
            species_index: species_index,
            card_index: card_index,
        })
    }
}

impl wire::ToWire<wire::remote::GP> for Growth {
    fn to_wire(&self) -> wire::remote::GP {
        wire::remote::GP {
            board_index: self.species_index.to_wire(),
            card_index: self.card_index.to_wire(),
        }
    }
}

impl wire::FromWire<wire::remote::GB> for Growth {
    fn from_wire(wire: wire::remote::GB) -> Result<Self, ()> {
        let species_index = *wire.board_index as usize;
        let card_index = *wire.card_index as usize;

        Ok(Growth {
            species_index: species_index,
            card_index: card_index,
        })
    }
}

impl wire::ToWire<wire::remote::GB> for Growth {
    fn to_wire(&self) -> wire::remote::GB {
        wire::remote::GB {
            board_index: self.species_index.to_wire(),
            card_index: self.card_index.to_wire(),
        }
    }
}

// NOTE: These impls are based on old data, and are being left in for old tests.

impl wire::FromWire<wire::Action4> for ActionChoice {
    fn from_wire(wire: wire::Action4) -> Result<Self, ()> {
        let food_card = *wire.0 as usize;
        let population_growths = try!(Vec::from_wire(wire.1));
        let body_growths = try!(Vec::from_wire(wire.2));
        let boards = try!(Vec::from_wire(wire.3));
        let traits = try!(Vec::from_wire(wire.4));
        let action_choice = ActionChoice {
            food_card: food_card,
            population_growths: population_growths,
            body_growths: body_growths,
            boards: boards,
            traits: traits,
        };

        if has_duplicate_cards(&action_choice) {
            Err(())
        } else {
            Ok(action_choice)
        }
    }
}

impl wire::ToWire<wire::Action4> for ActionChoice {
    fn to_wire(&self) -> wire::Action4 {
        wire::Action4 (
            self.food_card.to_wire(),
            self.population_growths.iter().map(|g| g.to_wire()).collect(),
            self.body_growths.iter().map(|g| g.to_wire()).collect(),
            self.boards.iter().map(|b| b.to_wire()).collect(),
            self.traits.iter().map(|t| t.to_wire()).collect(),
        )
    }
}

impl wire::FromWire<wire::GP> for Growth {
    fn from_wire(wire: wire::GP) -> Result<Self, ()> {
        let species_index = *wire.board_index as usize;
        let card_index = *wire.card_index as usize;

        Ok(Growth {
            species_index: species_index,
            card_index: card_index,
        })
    }
}

impl wire::ToWire<wire::GP> for Growth {
    fn to_wire(&self) -> wire::GP {
        wire::GP {
            board_index: self.species_index.to_wire(),
            card_index: self.card_index.to_wire(),
        }
    }
}

impl wire::FromWire<wire::GB> for Growth {
    fn from_wire(wire: wire::GB) -> Result<Self, ()> {
        let species_index = *wire.board_index as usize;
        let card_index = *wire.card_index as usize;

        Ok(Growth {
            species_index: species_index,
            card_index: card_index,
        })
    }
}

impl wire::ToWire<wire::GB> for Growth {
    fn to_wire(&self) -> wire::GB {
        wire::GB {
            board_index: self.species_index.to_wire(),
            card_index: self.card_index.to_wire(),
        }
    }
}

impl wire::FromWire<wire::BT> for BoardTrade {
    fn from_wire(wire: wire::BT) -> Result<Self, ()> {
        // TODO: Could be done with one vector.
        let indeces = wire.iter().map(|n| **n as usize).collect::<Vec<_>>();

        Ok(BoardTrade {
            card_index: indeces[0],
            trait_card_indeces: indeces[1..].to_vec(),
        })
    }
}

impl wire::ToWire<wire::BT> for BoardTrade {
    fn to_wire(&self) -> wire::BT {
        let mut indices = self.trait_card_indeces.clone();
        indices.insert(0, self.card_index);
        indices.to_wire()
    }
}

impl wire::FromWire<wire::RT> for TraitTrade {
    fn from_wire(wire: wire::RT) -> Result<Self, ()> {
        let species_index = *wire.0 as usize;
        let trait_index = *wire.1 as usize;
        let replacement_index = *wire.2 as usize;

        Ok(TraitTrade {
            species_index: species_index,
            trait_index: trait_index,
            replacement_index: replacement_index,
        })
    }
}

impl wire::ToWire<wire::RT> for TraitTrade {
    fn to_wire(&self) -> wire::RT {
        wire::RT(
            self.species_index.to_wire(),
            self.trait_index.to_wire(),
            self.replacement_index.to_wire(),
        )
    }
}

fn has_duplicate_cards(action_choice: &ActionChoice) -> bool {
    let mut index_set = HashSet::new();
    index_set.insert(action_choice.food_card);
    for growth in action_choice.population_growths.iter().chain(action_choice.body_growths.iter()) {
        if !index_set.insert(growth.card_index) {
            return true;
        }
    }
    for board_trade in action_choice.boards.iter() {
        if !index_set.insert(board_trade.card_index) {
            return true;
        }
        for trait_card_index in board_trade.trait_card_indeces.iter() {
            if !index_set.insert(*trait_card_index) {
                return true;
            }
        }
    }
    for trait_trade_index in action_choice.traits.iter().map(|t| t.replacement_index) {
        if !index_set.insert(trait_trade_index) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use evolution_wire::{self as wire, FromWire};
    use interact::*;

    #[test]
    fn from_wire_action_choice() {
        let wire = wire::Action4(
            wire::Natural(0),
            vec![
                wire::GP {
                    board_index: wire::Natural(0),
                    card_index: wire::Natural(1),
                },
            ],
            vec![
                wire::GB {
                    board_index: wire::Natural(0),
                    card_index: wire::Natural(2),
                },
            ],
            vec![
                wire::BT(vec![
                    wire::Natural(3),
                    wire::Natural(4),
                    wire::Natural(5),
                    wire::Natural(6),
                ]),
            ],
            vec![
                wire::RT(
                    wire::Natural(0),
                    wire::Natural(7),
                    wire::Natural(8)
                ),
            ],
        );
        let action_choice = ActionChoice::from_wire(wire)
                                         .expect("should be a valid ActionChoice");
        assert_eq!(0, action_choice.food_card);
        assert_eq!(1, action_choice.population_growths.len());
        assert_eq!(1, action_choice.body_growths.len());
        assert_eq!(1, action_choice.boards.len());
        assert_eq!(1, action_choice.traits.len());
    }

    #[test]
    fn from_wire_action_choice_invalid() {
        let wire = wire::Action4(
            wire::Natural(0),
            vec![
                wire::GP {
                    board_index: wire::Natural(0),
                    card_index: wire::Natural(1),
                },
            ],
            vec![
                wire::GB {
                    board_index: wire::Natural(0),
                    card_index: wire::Natural(2),
                },
            ],
            vec![
                wire::BT(vec![
                    wire::Natural(2),
                    wire::Natural(2),
                    wire::Natural(1),
                    wire::Natural(1),
                ]),
            ],
            vec![
                wire::RT(
                    wire::Natural(0),
                    wire::Natural(2),
                    wire::Natural(0)
                ),
            ],
        );
        let action_choice = ActionChoice::from_wire(wire);
        assert!(action_choice.is_err());
    }

    #[test]
    fn from_wire_population_growth() {
        let gp = wire::GP {
            board_index: wire::Natural(0),
            card_index: wire::Natural(1),
        };
        let population_growth = Growth::from_wire(gp).expect("failed to convert to Growth");
        assert_eq!(0, population_growth.species_index);
        assert_eq!(1, population_growth.card_index);
    }

    #[test]
    fn from_wire_body_growth() {
        let gb = wire::GB {
            board_index: wire::Natural(0),
            card_index: wire::Natural(1),
        };
        let body_growth = Growth::from_wire(gb).expect("failed to convert to Growth");
        assert_eq!(0, body_growth.species_index);
        assert_eq!(1, body_growth.card_index);
    }

    #[test]
    fn from_wire_board_trade() {
        let bt = wire::BT(vec![
            wire::Natural(0),
            wire::Natural(1),
            wire::Natural(2),
            wire::Natural(3),
        ]);
        let board_trade = BoardTrade::from_wire(bt).expect("failed to convert to BoardTrade");
        assert_eq!(0, board_trade.card_index);
        assert_eq!(1, board_trade.trait_card_indeces[0]);
        assert_eq!(2, board_trade.trait_card_indeces[1]);
        assert_eq!(3, board_trade.trait_card_indeces[2]);
    }

    #[test]
    fn from_wire_trait_trade() {
        let rt = wire::RT(
            wire::Natural(0),
            wire::Natural(1),
            wire::Natural(2)
        );
        let trait_trade = TraitTrade::from_wire(rt).expect("failed to convert to TraitTrade");
        assert_eq!(0, trait_trade.species_index);
        assert_eq!(1, trait_trade.trait_index);
        assert_eq!(2, trait_trade.replacement_index);
    }
}
