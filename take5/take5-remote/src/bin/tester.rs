extern crate serde_json;
extern crate take5_remote;

use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::thread;
use serde_json::{self as json, Value};
use take5_remote::message::RequestMessage;

#[derive(Debug)]
enum Error {
    InvalidTest(String),
    TestFailed(String),
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Error::InvalidTest(ref s) => write!(f, "invalid test: {}", s),
            Error::TestFailed(ref s) => write!(f, "test failed: {}", s),
            Error::Io(ref e) => e.fmt(f),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

fn read_file<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    let mut file = try!(File::open(path));
    let mut buf = String::new();
    try!(file.read_to_string(&mut buf));
    Ok(buf)
}

fn open_test(id: &str) -> Result<(String, String), Error> {
    println!("Loading test: {}", id);
    let requests = try!(read_file(format!("../test/{}-in.json", id)));
    let responses = try!(read_file(format!("../test/{}-out.json", id)));
    Ok((requests, responses))
}

fn handle_client(id: &str, mut writer: TcpStream) -> Result<(), Error> {
    let (test_requests, test_responses) = try!(open_test(id));
    let mut test_response_lines = test_responses.lines();

    let reader = try!(writer.try_clone());
    for line in test_requests.lines() {
        // Skip empty lines as "" isn't valid JSON.
        if line == "" { continue }
        match json::from_str::<RequestMessage>(line) {
            Ok(_) => {
                try!(writer.write(line.as_bytes()));
                // Return early if we are out of expected lines in the out file.
                let expected_result = match test_response_lines.next() {
                    Some(e) => e,
                    None => return Err(Error::InvalidTest("ran out of expected messages in out.json".to_string())),
                };
                // Return early if the expected json is invalid.
                let expected_json: Value = match json::from_str(&expected_result) {
                    Ok(j) => j,
                    Err(_) => return Err(Error::InvalidTest("out.json file is invalid".to_string())),
                };
                // Read 1 line, and ensure there are no more than one line in the buffer.
                let mut actual_result = String::new();
                let mut buf = BufReader::new(&reader);
                try!(buf.read_line(&mut actual_result));
                // Return early if the client sends back bad JSON.
                let actual_json: Value = match json::from_str(&actual_result) {
                    Ok(j) => j,
                    Err(_) => return Err(Error::TestFailed("client sent bad json".to_string())),
                };
                // Retrun early if the test fails to meet expectations.
                if expected_json != actual_json {
                    return Err(Error::TestFailed(format!("expected {:?} but got {:?}", expected_json, actual_json)))
                }
            },
            Err(_) => {
                // Send bad stuff too, but the client should do nothing.
                try!(writer.write(line.as_bytes()));
            },
        }
    };
    Ok(())
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:45678").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    println!("Starting game.");
                    let id = env::args().nth(1).unwrap();
                    match handle_client(&id, stream) {
                        Ok(_) => println!("Game ended successfully."),
                        Err(e) => println!("Game ended with an error: {}.", e),
                    }
                });
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    drop(listener);
}
