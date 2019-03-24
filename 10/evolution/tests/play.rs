#[macro_use]
extern crate log;
extern crate evolution;
extern crate evolution_wire as wire;
extern crate evolution_logger;

use std::thread;
use std::time::Duration;
use wire::{ToWire, FromWire, Channel};
use evolution::interact::*;
use evolution::game::*;
use evolution::silly::*;

#[test]
fn one_action_response() {
    let mut channels = vec![];
    let mut clients = vec![];

    for _ in 0..3 {
        let accepter = thread::spawn(move || {
            Channel::accept_from_socket_addr("127.0.0.1:1337").expect("failed to accept")
        });
        thread::sleep(Duration::from_millis(10));

        clients.push(thread::spawn(move || {
            let mut channel = Channel::connect_to_socket_addr("client".into(), "127.0.0.1:1337").expect("failed to connect");

            let observation = {
                let s: wire::remote::Start = channel.recv().expect("failed to recv");
                let o: (wire::remote::LOB, wire::remote::LOB) = channel.recv().expect("failed to recv");
                ActionObservation::from_wire((s, o)).expect("invalid action observation")
            };
            let choice = Silly.choose(&observation).unwrap().expect("no valid choice");
            let wire_choice = ToWire::<wire::remote::Action4>::to_wire(&choice);
            channel.send(&wire_choice).expect("failed to send");
        }));

        channels.push(accepter.join().unwrap());
    }

    let mut game = Game::<Channel>::new(channels).expect("invalid game");
    game.play();

    // All players cheated (disconnected early).
    assert_eq!(0, game.players().len());

    for client in clients {
        client.join().expect("failed to join");
    }
}

#[test]
fn two_action_responses() {
    let mut channels = vec![];
    let mut clients = vec![];

    for i in 0..3 {
        let accepter = thread::Builder::new().name(format!("acc{}", i)).spawn(move || {
            Channel::accept_from_socket_addr("127.0.0.1:1337").expect("failed to accept")
        }).expect("failed to spawn thread");
        thread::sleep(Duration::from_millis(10));

        clients.push(thread::Builder::new().name(format!("cli{}", i)).spawn(move || {
            let mut channel = Channel::connect_to_socket_addr("client".into(), "127.0.0.1:1337").expect("failed to connect");

            // Start up message.
            channel.send(&"info").expect("failed to send");
            channel.recv::<String>().expect("failed to recv");

            let observation = {
                let s: wire::remote::Start = channel.recv().expect("failed to recv");
                let o: (wire::remote::LOB, wire::remote::LOB) = channel.recv().expect("failed to recv");
                ActionObservation::from_wire((s, o)).expect("invalid action observation")
            };
            let choice = Silly.choose(&observation).unwrap().expect("no valid choice");
            let wire_choice = ToWire::<wire::remote::Action4>::to_wire(&choice);
            channel.send(&wire_choice).expect("failed to send");
        }).expect("failed to spawn thread"));

        channels.push(accepter.join().unwrap());
    }

    // Send each client a sign up success message.
    for channel in channels.iter_mut() {
        let _info = channel.recv::<String>().expect("failed to rec info");
        channel.send(&"ok").expect("failed to send \"ok\" to client");
    }

    debug!("starting game");
    let mut game = Game::<Channel>::new(channels).expect("invalid game");
    game.play();

    // All players cheated (disconnected early).
    assert_eq!(0, game.players().len());

    for client in clients {
        client.join().expect("failed to join");
    }
}

#[test]
fn play_one_round() {
    evolution_logger::Logger::init();
    let mut channels = vec![];
    let mut clients = vec![];

    for i in 0..3 {
        let accepter = thread::Builder::new().name(format!("acc{}", i)).spawn(move || {
            Channel::accept_from_socket_addr("127.0.0.1:1337").expect("failed to accept")
        }).expect("failed to spawn thread");
        thread::sleep(Duration::from_millis(10));

        clients.push(thread::Builder::new().name(format!("cli{}", i)).spawn(move || {
            let mut channel = Channel::connect_to_socket_addr("client".into(), "127.0.0.1:1337").expect("failed to connect");

            // Start up message.
            channel.send(&"info").expect("failed to send");
            channel.recv::<String>().expect("failed to recv");
            debug!("established connection to game");

            let observation = {
                let s: wire::remote::Start = channel.recv().expect("failed to recv");
                debug!("got start message");
                let o: (wire::remote::LOB, wire::remote::LOB) = channel.recv().expect("failed to recv");
                debug!("got opponents message");
                ActionObservation::from_wire((s, o)).expect("invalid action observation")
            };
            let choice = Silly.choose(&observation).unwrap().expect("no valid choice");
            let wire_choice = ToWire::<wire::remote::Action4>::to_wire(&choice);
            channel.send(&wire_choice).expect("failed to send");
            debug!("sent choice");
            while let Ok(either) = channel.recv_either::<wire::remote::State, wire::remote::Start>() {
                match either {
                    wire::Either::Left(state) => {
                        debug!("got state message");
                        let observation = FeedObservation::from_wire(state).expect("failed to get feed observation");
                        let choice = Silly.choose(&observation).unwrap().expect("no valid choice");
                        channel.send(&choice.to_wire()).expect("failed to send");
                        debug!("sent choice");
                    },
                    wire::Either::Right(_) => break,
                }
            }
        }).expect("failed to spawn thread"));

        channels.push(accepter.join().unwrap());
    }

    // Send each client a sign up success message.
    for channel in channels.iter_mut() {
        let _info = channel.recv::<String>().expect("failed to rec info");
        channel.send(&"ok").expect("failed to send \"ok\" to client");
    }

    let mut game = Game::<Channel>::new(channels).expect("invalid game");
    game.play();

    assert_eq!(0, game.players().len());

    for client in clients {
        client.join().expect("failed to join");
    }
}

#[test]
fn game() {
    evolution_logger::Logger::init();
    let mut channels = vec![];
    let mut clients = vec![];

    for i in 0..4 {
        let accepter = thread::Builder::new().name(format!("acc{}", i)).spawn(move || {
            Channel::accept_from_socket_addr("127.0.0.1:1337").expect("failed to accept")
        }).expect("failed to spawn thread");
        thread::sleep(Duration::from_millis(10));

        clients.push(thread::Builder::new().name(format!("cli{}", i)).spawn(move || {
            let mut channel = Channel::connect_to_socket_addr("client".into(), "127.0.0.1:1337").expect("failed to connect");

            // Start up message.
            channel.send(&"info").expect("failed to send");
            channel.recv::<String>().expect("failed to recv");
            debug!("established connection to game");

            let mut start: Option<wire::remote::Start> = None;
            start = Some(channel.recv().expect("failed to recv"));
            debug!("got initial start message");

            loop {
                let observation = {
                    let o: (wire::remote::LOB, wire::remote::LOB) = channel.recv().expect("failed to recv");
                    // debug!("got opponents message");
                    match start {
                        Some(s) => {
                            start = None;
                            ActionObservation::from_wire((s, o)).expect("invalid action observation")
                        },
                        None => panic!("no start message set!")
                    }
                };
                debug!("action observation updated: domain: {}; hand: {}",
                    observation.current_player.domain().len(),
                    observation.current_player.hand().len(),
                );
                let choice = Silly.choose(&observation).unwrap().expect("no valid choice");
                let wire_choice = ToWire::<wire::remote::Action4>::to_wire(&choice);
                channel.send(&wire_choice).expect("failed to send");
                debug!("sent choice: {:?}", wire_choice);
                while let Ok(either) = channel.recv_either::<wire::remote::State, wire::remote::Start>() {
                    match either {
                        wire::Either::Left(state) => {
                            // debug!("got state message");
                            let observation = FeedObservation::from_wire(state).expect("failed to get feed observation");
                            let choice = Silly.choose(&observation).unwrap().expect("no valid choice");
                            // channel.send(&choice.to_wire()).expect("failed to send");
                            // debug!("sent choice");
                            let wire_choice = choice.to_wire();
                            channel.send(&wire_choice).expect("failed to send");
                            debug!("sent feed choice: {:?}", wire_choice);
                        },
                        wire::Either::Right(new_start) => {
                            start = Some(new_start);
                            debug!("end feeding");
                            break
                        },
                    }
                }
            }
        }).expect("failed to spawn thread"));

        channels.push(accepter.join().unwrap());
    }

    // Send each client a sign up success message.
    for channel in channels.iter_mut() {
        let _info = channel.recv::<String>().expect("failed to rec info");
        channel.send(&"ok").expect("failed to send \"ok\" to client");
    }

    let mut game = Game::<Channel>::new(channels).expect("invalid game");
    game.play();

    assert_eq!(4, game.players().len());
    for player in game.players() {
        assert_eq!(1, player.score());
    }

    for client in clients {
        client.join().expect("failed to join");
    }
}
