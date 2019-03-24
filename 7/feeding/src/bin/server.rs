#[macro_use]
extern crate log;
extern crate itertools;
extern crate evolution_logger;
extern crate evolution_wire as wire;
extern crate evolution;

use std::env;
use std::cmp;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;
use itertools::Itertools;
use evolution_logger::Logger;
use wire::Channel;
use evolution::game::*;

fn main() {
    Logger::init().expect("logger failed to start");

    let channels = accept_clients(parse_args());
    // TODO: <refactor> A struct for the player notion above it's state would help.
    info!("Playing game with {}.", channels.iter().enumerate().map(|(i, channel)| {
        format!("{}=>{}", i + 1, channel.info())
    }).join(" "));
    let mut game = Game::<Channel>::new(channels).expect("failed to create game");
    game.play();
    game.print_scores();
}

/// Wait for `n` clients to connect, returning at most `MAX_PLAYERS`
/// channels.
fn accept_clients(n: usize) -> Vec<Channel> {
    let listener = TcpListener::bind("127.0.0.1:1337").unwrap();

    let channels = Arc::new(Mutex::new(vec![]));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Clone the arc, this internally bumps the reference count.
                let channels = channels.clone();

                // Spawn a new thread to establish a connection with the client.
                thread::spawn(move || {
                    // Take the lock before establishing the connection.
                    let mut channels = channels.lock().expect("failed to get lock");
                    // TODO: <question> Can we try to establish a connection
                    // if we are full of players?
                    match Channel::accept_from_tcp_stream(stream) {
                        Ok(channel) => {
                            // Push the client's channel into the channels.
                            info!("Player connected with info `{}`.", channel.info());
                            channels.push(channel);
                        }
                        Err(e) => {
                            warn!("couldn't establish channel: {:?}", e);
                        }
                    }
                });
            }
            Err(e) => {
                warn!("connecting failed: {:?}", e);
            }
        }

        // Check if we've gotten enough, or time has run up?
        // TODO: <needed> Check time.
        let channels = channels.lock().expect("failed to get lock");
        if channels.len() >= n {
            debug!("Channel count: {}", channels.len());
            break
        }
    }

    let mut channels = channels.lock().expect("failed to get lock");
    let num = cmp::min(channels.len(), MAX_PLAYERS);
    let vec = channels.drain(0..num).collect::<Vec<_>>();
    assert!(vec.len() >= MIN_PLAYERS);
    vec
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
