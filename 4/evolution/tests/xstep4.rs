#![feature(plugin)]
#![plugin(evolution_test)]
#![allow(plugin_as_library)]

extern crate evolution;
#[macro_use]
extern crate evolution_test;
extern crate evolution_wire;

each_test!("tests/start_round_json", evolution_wire::StartRound, "xstep4");
