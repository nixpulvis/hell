extern crate take5;
extern crate take5_remote;
extern crate take5_ext;

use take5_remote::Client;
use take5::player::AiPlayer;

// TODO: Arg parse a `-n` argument for `newline = true`.
fn main() {
    let player = Box::new(AiPlayer::new(1));
    let mut client = Client::new("127.0.0.1:45678", player, true).unwrap_or_else(|e| {
        panic!("{}", e);
    });
    match client.start() {
        Ok(_) => println!("Game Over"),
        Err(_) => panic!("Something bad happened while playing."),
    }
}
