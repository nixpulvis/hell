extern crate serde_json;
extern crate evolution_wire;
#[macro_use]
extern crate evolution;

use evolution_wire as wire;
use evolution::interact::*;
use evolution::silly::*;

test_harness!(feed_observation = wire::Feeding => FeedObservation, {
    if let Ok(Some(choice)) = Silly.choose(&feed_observation) {
        println!("{}", json::to_string(&choice.to_wire()).unwrap());
    }
});
