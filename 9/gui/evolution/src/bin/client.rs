#[macro_use]
extern crate log;
extern crate evolution_logger;
extern crate evolution_wire as wire;
extern crate evolution;

use std::env;
use evolution_logger::Logger;
use wire::{ToWire, FromWire, Channel};
use evolution::interact::*;
use evolution::silly::*;

fn main() {
    Logger::init().expect("logger failed to start");
    debug!("starting client...");
    let mut channel = establish_channel();
    let mut next_start: Option<wire::remote::Start> = None;
    next_start = Some(channel.recv().expect("failed to recv"));
    debug!("got initial start {:?}", next_start);

    loop {
        // Block for another message.
        let observation = {
            let o: (wire::remote::LOB, wire::remote::LOB) = channel.recv().expect("failed to recv");
            debug!("got obvs {:?}", o);
            ActionObservation::from_wire((next_start.expect("no start message"), o)).expect("invalid action observation")
        };
        next_start = None;
        let choice = Silly.choose(&observation).unwrap().expect("no valid choice");
        let wire_choice = ToWire::<wire::remote::Action4>::to_wire(&choice);
        channel.send(&wire_choice).expect("failed to send");
        debug!("send choice {:?}", choice);

        while let Ok(either) = channel.recv_either::<wire::remote::Start, wire::remote::State>() {
            match either {
                wire::Either::Left(start) => {
                    debug!("got new start {:?}", start);
                    next_start = Some(start);
                    debug!("breaking feed loop");
                    break;
                },
                wire::Either::Right(state) => {
                    debug!("got state {:?}", state);
                    let observation = FeedObservation::from_wire(state).expect("invalid feed observation");
                    debug!("got obvs {:?}", observation);
                    let choice = Silly.choose(&observation).unwrap().expect("no valid choice");
                    let wire_choice = choice.to_wire();
                    channel.send(&wire_choice).expect("failed to send");
                    debug!("send choice {:?}", choice);
                },
            }
        }
    }
}

fn establish_channel() -> Channel {
    let info = parse_args();
    let socket_addr = "127.0.0.1:1337";
    if let Ok(channel) = Channel::connect_to_socket_addr(info, socket_addr) {
        channel
    } else {
        panic!("failed to connect to channel");
    }
}

fn parse_args() -> String {
    let mut args = env::args();
    if let Some(s) = args.nth(1) {
        s
    } else {
        panic!("not given argument");
    }
}
