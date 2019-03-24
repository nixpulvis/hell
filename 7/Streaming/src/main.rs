extern crate serde_json;

use std::collections::HashMap;
use serde_json::*;
use std::io::{self, Read};

fn main() {
    let mut count = 0;
    let stdin = io::stdin();
    let stream_deserializer: StreamDeserializer<Value, _> = StreamDeserializer::new(stdin.bytes());

    for v in stream_deserializer {
        match v {
            Ok(value) => {
                println!("{:?}", value);
                count += 1;
            },
            Err(_) => {
                println!("Encountered a json parsing error, closing");
                return
            },
        }
    }
    let mut map = HashMap::new();
    map.insert("count", count);
    println!("{}", serde_json::to_string(&map).unwrap());
}
