//! Interface with external take5 protocol.
//!
//! This crate wraps the needed structures from `take5` and provides
//! JSON serialization methods for them. This allows us to read and write
//! to the network protocol for playing the game.
#![feature(custom_derive, plugin)]

extern crate serde;
extern crate serde_json;
extern crate take5;

pub use client::Client;
pub use game_state::GameState;

/// Messages between clients and servers.
pub mod message;

mod client;
mod error;
mod game_state;
mod iter;
mod player_proxy;
mod starting_hand;
mod wrapper;

#[cfg(test)]
mod tests {
    use std::io::prelude::*;
    use std::net::{TcpListener, TcpStream};
    use std::thread;
    use std::time::Duration;
    use serde_json as json;
    use wrapper::{Card, Stack};
    use message::RequestMessage;

    #[test]
    fn test_card_decode_bad_values() {
        let json = "[0,2]";
        assert!(json::from_str::<Card>(json).is_err());
        let json = "[105,2]";
        assert!(json::from_str::<Card>(json).is_err());
        let json = "[1,1]";
        assert!(json::from_str::<Card>(json).is_err());
        let json = "[1,8]";
        assert!(json::from_str::<Card>(json).is_err());
    }

    #[test]
    fn test_stack_decode_empty() {
        let json = "[]";
        assert!(json::from_str::<Stack>(json).is_err());
    }

    #[test]
    fn test_decode_over_tcp() {
        thread::spawn(move|| {
            let listener = TcpListener::bind("127.0.0.1:45678").unwrap();
            for stream in listener.incoming() {
                match stream {
                    Ok(mut stream) => {
                        thread::spawn(move|| {
                            let json = "[\"start-round\",[[1,2],[2,2],[3,2],[4,2],[5,2],[6,2],[7,2],[8,2],[9,2],[10,2]]]";
                            let string = String::from(json);
                            for line in string.lines() {
                                stream.write(line.as_bytes()).unwrap();
                            }
                        });
                    }
                    Err(e) => {
                        panic!("{}", e);
                    }
                }
            }
            drop(listener);
        });

        // Giving time for server to spin up.
        thread::sleep(Duration::from_millis(25));
        let stream = TcpStream::connect("127.0.0.1:45678").unwrap();
        assert!(json::from_reader::<_, RequestMessage>(stream).is_ok());
    }
}
