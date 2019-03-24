use evolution_wire as wire;
use game::*;
use interact::*;
use object::*;

 impl wire::ToWire<wire::remote::State> for FeedObservation {
     fn to_wire(&self) -> wire::remote::State {
         wire::remote::State {
             bag: self.current_player.bag().len().to_wire(),
             domain: self.current_player.domain().to_wire(),
             hand: self.current_player.hand().to_wire(),
             watering_hole: self.board.to_wire(),
             opponents: self.opponents.iter().map(|o| o.domain.to_wire()).collect(),
         }
     }
 }

 impl wire::FromWire<wire::remote::State> for FeedObservation {
    fn from_wire(wire: wire::remote::State) -> Result<Self, ()> {
        let current_player = try!(Player::from_wire(wire::Player {
            id: wire::NaturalPlus::new(1).expect("failed to make player id"),
            species: wire.domain,
            bag: wire.bag,
            cards: if wire.hand.len() > 0 {
                Some(wire.hand)
            } else {
                None
            },
        }));
        let mut opponents = Vec::new();
        for o in wire.opponents {
            opponents.push(PlayerObservation {
                id: 1,
                domain: try!(Domain::from_wire(o)),
                bag: 0,
            });
        }
        let board = try!(BoardObservation::from_wire(wire.watering_hole));
        Ok(FeedObservation {
            current_player: current_player,
            opponents: opponents,
            board: board,
        })
    }
 }

impl wire::ToWire<wire::Feeding> for FeedObservation {
    fn to_wire(&self) -> wire::Feeding {
        // NOTE: Remember this is only the observed state, see
        // `wire::remote::State` for the type which holds all the player's
        // state.
        let player = self.current_player.to_wire();
        let opponents = self.opponents.iter().map(|p| p.to_wire()).collect();
        let watering_hole = self.board.to_wire();
        wire::Feeding {
            current_player: player,
            opponents: opponents,
            watering_hole: watering_hole,
        }
    }
}

impl wire::FromWire<wire::Feeding> for FeedObservation {
    fn from_wire(wire: wire::Feeding) -> Result<Self, ()> {
        let board = try!(BoardObservation::from_wire(wire.watering_hole));
        let opponents = try!(Vec::from_wire(wire.opponents));
        let current_player = try!(Player::from_wire(wire.current_player));

        let observation = FeedObservation {
            board: board,
            opponents: opponents,
            current_player: current_player,
        };

        if observation.len() >= MIN_PLAYERS &&
           observation.len() <= MAX_PLAYERS
        {
            Ok(observation)
        } else {
            // HACK: This was left like this to support the old spec which
            // unfortunately required some invalid data to be sent.
            Ok(observation)
        }
    }
}

#[cfg(test)]
mod tests {
    use evolution_wire::{self as wire, FromWire};
    use interact::*;

    #[test]
    fn observed_from_wire_feeding() {
        let feeding = wire::Feeding {
            current_player: wire::Player {
                id: wire::NaturalPlus::new(1).unwrap(),
                species: vec![],
                bag: wire::Natural(0),
                cards: None,
            },
            watering_hole: wire::NaturalPlus::new(25).unwrap(),
            opponents: vec![
                wire::Player {
                    id: wire::NaturalPlus::new(2).unwrap(),
                    species: vec![],
                    bag: wire::Natural(0),
                    cards: None,
                },
                wire::Player {
                    id: wire::NaturalPlus::new(3).unwrap(),
                    species: vec![],
                    bag: wire::Natural(0),
                    cards: None,
                },
            ],
        };

        assert_eq!(1, *feeding.current_player.id);
        assert_eq!(2, *feeding.opponents[0].id);
        assert_eq!(3, *feeding.opponents[1].id);
        assert_eq!(25, *feeding.watering_hole);

        let observed = FeedObservation::from_wire(feeding).unwrap();

        assert_eq!(2, observed.opponents.len());
        assert_eq!(2, observed.opponents[0].id);
        assert_eq!(3, observed.opponents[1].id);
        assert_eq!(1, observed.current_player.id());
    }

    #[test]
    #[ignore]
    fn observed_from_wire_state() {
        unimplemented!()
    }
}
