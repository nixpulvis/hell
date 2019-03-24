#[macro_use]
extern crate log;
extern crate evolution_logger;
extern crate evolution;

use std::env;
use evolution_logger::*;
use evolution::game::*;
use evolution::silly::*;

fn main() {
    Logger::init().expect("logger failed to start");

    let player_count = parse_args();
    let mut game = Game::<Silly>::new(player_count).unwrap();
    game.play();
    game.print_scores();
}

/// Get the requested number of players from the command line arguments.
fn parse_args() -> usize {
    let mut args = env::args();
    if let Some(s) = args.nth(1) {
        if let Ok(n) = s.parse() {
            n
        } else {
            panic!("not given a valid number");
        }
    } else {
        panic!("not given a number of players");
    }
}
