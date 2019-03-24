use std::collections::{HashMap, HashSet};
use evolution_wire as wire;
use game::*;
use object::*;
use silly::*;

// TODO: `Game::new` should do validation.

impl wire::FromWire<wire::Configuration> for Game<Silly> {
    fn from_wire(wire: wire::Configuration) -> Result<Self, ()> {
        let players = try!(Vec::<Player>::from_wire(wire.players));
        let board = try!(Board::from_wire(wire.watering_hole));
        let deck = try!(Vec::<Card>::from_wire(wire.deck));

        if players.len() < MIN_PLAYERS ||
           players.len() > MAX_PLAYERS
        {
            return Err(())
        }

        let mut validation_map : HashMap<Trait, usize> = HashMap::new();
        for player in players.iter() {
            for card in player.hand() {
                let mut entry = validation_map.entry(card.trait_type()).or_insert(0);
                *entry += 1;
            }
        }
        for card in deck.iter() {
            let mut entry = validation_map.entry(card.trait_type()).or_insert(0);
            *entry += 1;
        }
        for (trait_type, count) in validation_map.iter() {
            let trait_type = *trait_type;
            let count = *count;
            let maximum_count = match trait_type {
                Trait::Carnivore => NUM_CARNIVORE_CARDS,
                _ => NUM_VEGITARIAN_CARDS,
            };
            if count > maximum_count {
                return Err(())
            }
        }

        Ok(Game {
            players: players,
            // HACK: <refactor> It's interesting that we don't actually need
            // to fill in the `choosers`.
            choosers: HashMap::new(),
            skip_set: HashSet::new(),
            current_player: Some(0),
            board: board,
            deck: deck,
        })
    }
}

impl wire::ToWire<wire::Configuration> for Game<Silly> {
    fn to_wire(&self) -> wire::Configuration {
        wire::Configuration {
            players: self.players().to_wire(),
            watering_hole: (self.board().food().len() as u64).to_wire(),
            deck: self.deck.as_slice().to_wire(),
        }
    }
}

#[cfg(test)]
mod tests {
    use evolution_wire::{self as wire, FromWire, ToWire};
    use game::*;
    use object::*;
    use silly::*;

    #[test]
    fn from_wire() {
        let configuration = wire::Configuration {
            players: vec![
                wire::Player {
                    id: wire::NaturalPlus::new(1).unwrap(),
                    species: vec![],
                    bag: wire::Natural(10),
                    cards: None,
                },
                wire::Player {
                    id: wire::NaturalPlus::new(2).unwrap(),
                    species: vec![],
                    bag: wire::Natural(20),
                    cards: None,
                },
                wire::Player {
                    id: wire::NaturalPlus::new(3).unwrap(),
                    species: vec![],
                    bag: wire::Natural(30),
                    cards: None,
                },
            ],
            watering_hole: wire::Natural(100),
            deck: vec![],
        };

        let game = Game::from_wire(configuration).unwrap();

        assert_eq!(3, game.players().len());
        assert_eq!(1, game.players()[0].id());
        assert_eq!(2, game.players()[1].id());
        assert_eq!(3, game.players()[2].id());
        assert_eq!(100, game.board().food().len());
        assert_eq!(0, game.deck().len());
    }

    #[test]
    fn to_wire() {
        let mut game = Game::<Silly>::new(3).unwrap();
        game.board_mut().push_foods(vec![FoodToken, FoodToken]);

        assert_eq!(3, game.players().len());
        assert_eq!(1, game.players()[0].id());
        assert_eq!(2, game.players()[1].id());
        assert_eq!(3, game.players()[2].id());
        assert_eq!(2, game.board().food().len());
        assert_eq!(122, game.deck().len());

        let configuration = game.to_wire();

        assert_eq!(3, configuration.players.len());
        assert_eq!(1, *configuration.players[0].id);
        assert_eq!(2, *configuration.players[1].id);
        assert_eq!(3, *configuration.players[2].id);
        assert_eq!(2, *configuration.watering_hole);
        assert_eq!(122, configuration.deck.len());
    }

    #[test]
    #[ignore]
    fn game_to_feeding() {
        // TODO: Test `ToWire<wire::Feeding> for FeedObservation`.
        unimplemented!()
    }
}
