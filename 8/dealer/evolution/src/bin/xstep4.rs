extern crate serde_json;
extern crate evolution_wire;
#[macro_use]
extern crate evolution;

use evolution_wire as wire;
use evolution::game::*;
use evolution::interact::*;
use evolution::silly::*;

test_harness!(StartRound(mut game, mut action_choices) = wire::StartRound => StartRound, {
    unsafe {
        let game: *mut Game<Silly> = &mut game;

        for (i, _) in (&*game).players().iter().enumerate() {
            step::Action(&mut *game, &mut action_choices[i]).step().unwrap();
        }

        step::Reveal(&mut *game).step().unwrap();

        while !(&*game).turn_is_over() {
            step::Feed(&mut *game, &mut Auto(&mut Silly)).step().unwrap();
        }
    }

    println!("{}", json::to_string(&game.to_wire()).unwrap());
});

/// The start round wire type is really a `Game`, and a bunch of action choices.
struct StartRound(Game<Silly>, Vec<ActionChoice>);

impl wire::FromWire<wire::StartRound> for StartRound {
    fn from_wire(wire: wire::StartRound) -> Result<StartRound, ()> {
        let game = try!(Game::from_wire(wire.configuration));
        let action_choices = try!(Vec::from_wire(wire.step_actions));
        Ok(StartRound(game, action_choices))
    }
}
