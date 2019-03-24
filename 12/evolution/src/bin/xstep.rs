extern crate serde_json;
extern crate evolution_wire;
#[macro_use]
extern crate evolution;

use evolution_wire as wire;
use evolution::game::*;
use evolution::silly::*;

test_harness!(mut game = wire::Configuration => Game, {
    step::Feed(&mut game, &mut Auto(&mut Silly)).step().unwrap();
    println!("{}", json::to_string(&game.to_wire()).unwrap());
});
