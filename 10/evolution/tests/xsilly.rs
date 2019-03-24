#![feature(plugin)]
#![plugin(evolution_test)]
#![allow(plugin_as_library)]

extern crate evolution;
#[macro_use]
extern crate evolution_test;
extern crate evolution_wire;

each_test!("tests/choice_json", evolution_wire::Choice, "xsilly");
