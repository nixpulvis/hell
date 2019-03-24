use std::fs::File;
use std::io::Read;
use std::path::Path;
use rand::{self, Rng};
use take5::{Card, Player, Dealer, Configuration};

/// A dealer who reads bull values from a configuration file.
#[derive(Debug, Default)]
pub struct CustomDealer {
    deck: Vec<Card>,
    bull_vals: Vec<u8>,
}

impl CustomDealer {
    /// Creates a new CustomDealer using the config file with the given filename
    ///
    /// # Examples
    /// ```
    /// use take5_ext::CustomDealer;
    ///
    /// let _ = CustomDealer::new("../take5-cli/bull.conf");
    /// ```
    pub fn new<R>(filename: R) -> CustomDealer where R: AsRef<Path> {
        let mut file = File::open(filename).expect("File opening failed");
        let mut file_contents = String::new();
        let mut bull_values = Vec::new();

        file.read_to_string(&mut file_contents).expect("File reading failed");
        for s in file_contents.trim().split(',') {
            bull_values.push(s.trim().parse().expect("Invalid bull value found in conf file"));
        }

        CustomDealer {
            deck: Vec::new(),
            bull_vals: bull_values,
        }
    }
}

impl Dealer for CustomDealer {
    fn open(&mut self) {
        self.deck = Card::deck(|i| self.bull_vals[(i - 1) as usize]);
        rand::thread_rng().shuffle(&mut self.deck)
    }

    fn deal(&mut self) -> Card {
        self.deck.pop().expect("dealer ran out of cards.")
    }

    fn deals(&mut self, players: &mut [Box<Player>]) {
        for _ in 0..Configuration::turn_count() {
            for player in players.iter_mut() {
                player.draw(self.deal());
            }
        }
    }
}
