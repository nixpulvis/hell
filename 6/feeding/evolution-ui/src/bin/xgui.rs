extern crate serde_json;
extern crate evolution;
extern crate evolution_wire;
extern crate evolution_server;

use std::io;
use std::io::prelude::*;
use std::process::{Command, Stdio};
use serde_json as json;
use evolution_wire::{self as wire, FromWire, ToWire};

pub fn main() {
    let mut reader = io::stdin();
    let wire = json::from_reader::<_, wire::Configuration>(&mut reader).expect("invalid JSON");
    let dealer = evolution_server::Dealer::from_wire(wire).expect("invalid dealer configuration");
    let configuration = json::to_string::<wire::Configuration>(&dealer.to_wire()).unwrap();

    let dealer_sub = Command::new("target/debug/xgui_dealer")
                              .stdin(Stdio::piped())
                              .stdout(Stdio::piped())
                              .spawn()
                              .unwrap();
    dealer_sub.stdin.unwrap().write_all(&configuration.as_bytes());

    let player_sub = Command::new("target/debug/xgui_player")
                              .stdin(Stdio::piped())
                              .stdout(Stdio::piped())
                              .spawn()
                              .unwrap();
    player_sub.stdin.unwrap().write_all(&configuration.as_bytes());

    // write nothing, as per the spec
    println!("");
}
