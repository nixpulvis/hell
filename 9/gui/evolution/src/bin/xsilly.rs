extern crate serde_json;
extern crate evolution_wire;
#[macro_use]
extern crate evolution;

use evolution_wire as wire;
use evolution::interact::*;
use evolution::silly::*;

test_harness!(action_observation = wire::Choice => ActionObservation, {
    if let Ok(Some(choice)) = Silly.choose(&action_observation) {
        let wire_choice = ToWire::<wire::Action4>::to_wire(&choice);
        println!("{}", json::to_string(&wire_choice).unwrap());
    }
});
