use std::io::Write;
use std::net::{ToSocketAddrs, TcpStream};
use serde_json as json;
use take5::*;

use error::Error;
use game_state::GameState;
use iter::DeserializeJsonIter;
use message::RequestMessage;
use player_proxy::PlayerProxy;

/// A client who reads messages from a server and plays take5.
///
/// The server is expected to communicate over a TCP stream, sending
/// JSON data in a specific form. See the `message` module for more
/// information on the data format. There is currently no connect, or
/// finish messages defined, so the client cannot gracefully setup or
/// teardown a connection. This means that this client will happily exit
/// even if it didn't play a full game.
///
/// See the `take5` documentation for details about the `Player` argument
/// for new clients.
///
/// # Examples
///
/// ```
/// # extern crate take5;
/// # extern crate take5_remote;
/// # fn main() {
/// use std::process::exit;
/// use take5::player::AiPlayer;
/// use take5_remote::Client;
///
/// // Create an AI.
/// let player = Box::new(AiPlayer::new(1));
/// // Create a client connected to localhost on port 45678.
/// let mut client = Client::new("127.0.0.1:45678", player, true).unwrap_or_else(|_| {
///     println!("Couldn't connect to server");
///     exit(0);
/// });
/// // Start the client.
/// match client.start() {
///     Ok(_) => println!("Game Over"),
///     Err(_) => panic!("Something bad happened while playing."),
/// }
/// # }
/// ```
pub struct Client {
    player: PlayerProxy,
    stream: TcpStream,
    newline: bool,
    current_state: GameState,
}

impl Client {
    /// Given a server's address and a player, try to connect to the server
    /// and get ready to play take5. Returns `Err` when connecting to the
    /// server fails.
    ///
    /// The `newline` argument is a *hack* because our test server doesn't
    /// support parsing response messages without a dilimiter. This is made
    /// difficult because of the two response message types `TakeTurn` and
    /// `Choose` because the are both encoded as JSON arrays without any
    /// distinguishing element at the start. It is however technically
    /// possible to parse because a `TakeTurn` contains a `Card` which
    /// always has a number as the first element, and `Choose` contains
    /// a `Stack` which always has an array as the first element.
    /// Parsing two tokens ahead isn't supported *easily* by our parser,
    /// so we opted for a `\n` seperator for the purposes of testing, as
    /// reading a `ResponseMessage` wasn't part of this assignment anyway.
    pub fn new<S>(server: S, player: Box<Player>, newline: bool) -> Result<Self, Error>
        where S: ToSocketAddrs
    {
        Ok(Client {
            player: PlayerProxy { player: player },
            stream: try!(TcpStream::connect(server)),
            newline: newline,
            current_state: GameState::StartingRound,
        })
    }

    /// Starts the client listening for messages from the server. When
    /// the client gets a message it parses it, and if it's valid it handles
    /// the message. If the data is not valid, then the server ignores the
    /// message. Message handling consists of two cases. The server sent a
    /// valid message at the correct time, or at an invalid time. An example
    /// of an invalid timing for a message would be a `Choose` message before
    /// a `TakeTurn` message in a round. See `GameState` for more information
    /// about the state machine at play here.
    ///
    /// This function can return `Err` if an underlying IO error occurs.
    pub fn start(&mut self) -> Result<(), Error> {
        let mut reader = try!(self.stream.try_clone());
        for request in DeserializeJsonIter::new(&mut reader) {
            if let Ok(message) = request {
                try!(self.handle_message(message));
            }
        }
        Ok(())
    }

    /// Returns the current state of the game. See `GameState` for more
    /// information about the game states. This value is updated when
    /// a message is handled inside the game loop from `start`.
    pub fn current_state(&self) -> &GameState {
        &self.current_state
    }

    fn handle_message(&mut self, message: RequestMessage) -> Result<(), Error> {
        println!("In state {:?} got message:\n{:?}", self.current_state, message);
        let mut json = match self.current_state.next_state(&message) {
            Ok(next_state) => {
                self.current_state = next_state;
                let response = self.player.process_message(message);
                json::to_string(&response).expect("message failed to serialize to JSON")
            },
            Err(_) => {
                "false".to_string()
            }
        };
        if self.newline { json.push('\n') }
        println!("Sending result: {:?}\n", json);
        try!(self.stream.write(json.as_bytes()));
        Ok(())
    }
}
