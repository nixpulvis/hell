use std::path::Path;
use std::fs::File;
use std::io::Read;
use rustc_serialize::json;

static mut CONFIG: Configuration = Configuration {
    turn_count: 10,
    stack_size: 5,
    win_score: 66,
    deck_size: 104,
    bull_range: (1, 7),
};

/// Global configuration for the game.
#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Configuration {
    turn_count: usize,
    stack_size: usize,
    win_score: u32,
    deck_size: u8,
    bull_range: (u8, u8),
}

impl Configuration {
    /// Loads a configuration from a file and sets the global `CONFIG` to it.
    ///
    /// This the default configuration as a file. All fields are required.
    ///
    /// ```json
    /// {
    ///     "turn_count" : 10,
    ///     "stack_size" : 5,
    ///     "win_score" : 66,
    ///     "deck_size" : 104,
    ///     "bull_range" : [1, 7]
    /// }
    /// ```
    ///
    /// # Unsafety
    ///
    /// This function should be called before creating any other structures
    /// from within this crate. This function mutates a static variable
    /// so calls to this after the values of the config have been used may
    /// lead to inconsistent results.
    pub unsafe fn load<R>(filename: R) where R: AsRef<Path> {
        let mut file = File::open(filename).unwrap();
        let mut config_string = String::new();
        file.read_to_string(&mut config_string).unwrap();
        CONFIG = json::decode(&config_string).unwrap();
    }

    /// Returns the number of turns per round for the game.
    pub fn turn_count() -> usize {
        unsafe { CONFIG.turn_count }
    }

    /// Returns the size of the stacks on the board.
    pub fn stack_size() -> usize {
        unsafe { CONFIG.stack_size }
    }

    /// Returns the score at which the game ends after a round, unless
    /// there's a tie.
    pub fn win_score() -> u32 {
        unsafe { CONFIG.win_score }
    }

    /// Returns the number of cards to put in the decks.
    pub fn deck_size() -> u8 {
        unsafe { CONFIG.deck_size }
    }

    /// Returns the allowable range of bull values for cards in the form
    /// `(lower, upper)`, where the values are inclusive.
    pub fn bull_range() -> (u8, u8) {
        unsafe { CONFIG.bull_range }
    }
}
