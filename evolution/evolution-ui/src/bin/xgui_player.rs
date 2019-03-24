extern crate serde_json;
extern crate evolution;
extern crate evolution_wire;
extern crate evolution_server;
extern crate evolution_ui;

use std::io;
use serde_json as json;
use evolution_wire::{self as wire, FromWire};
use evolution_ui::*;

pub fn main() {
    let mut reader = io::stdin();
    let wire = json::from_reader::<_, wire::Configuration>(&mut reader).expect("invalid JSON");
    let dealer = evolution_server::Dealer::from_wire(wire).expect("invalid dealer configuration");

    evolution_ui::run(|| { PlayerWidget::new(dealer.game().unwrap().current_player()) }, "Current Player");
}
