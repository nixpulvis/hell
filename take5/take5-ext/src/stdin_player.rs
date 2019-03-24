use std::fmt;
use std::io::{self, BufRead, Write};
use take5::{Board, StackId, Card, Player};
use take5::player::Id;

/// A player who makes decisions from STDIN.
///
/// Creating an `StdinPlayer` is done with `StdinPlayer::new()`.
#[derive(Debug)]
pub struct StdinPlayer {
    id: Id,
    hand: Vec<Card>,
    pile: Vec<Card>,
}

impl StdinPlayer {
    /// Creates a new AI player with the given ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use take5_ext::StdinPlayer;
    ///
    /// let _ = StdinPlayer::new(1);
    /// ```
    pub fn new(id: Id) -> Self {
        StdinPlayer {
            id: id,
            hand: Vec::new(),
            pile: Vec::new(),
        }
    }
}

impl Player for StdinPlayer {
    fn id(&self) -> Id {
        self.id
    }

    fn hand(&self) -> &[Card] {
        &self.hand
    }

    fn pile(&self) -> &[Card] {
        &self.pile
    }

    fn draw(&mut self, card: Card) {
        self.hand.push(card)
    }

    fn look_at_board(&mut self, board: &Board) {
        println!("Player: {} looks at board:\n{}", self, board);
    }

    fn look_at_played(&mut self, played: &[(Card, Id)]) {
        println!("Player: {} looks at played: {:?}", self, played);
    }

    fn look_at_taken(&mut self, taken: (&[Card], Id)) {
        println!("TAKEN: {:?}", taken);
    }

    fn play(&mut self) -> Card {
        print!("Your hand is: ");
        for card in self.hand() {
            print!("{}, ", card);
        }
        print!("\nPlay a card: ");
        io::stdout().flush().expect("error writing to stdout.");
        let stdin = io::stdin();
        let line = stdin.lock()
                        .lines()
                        .next()
                        .expect("no more lines in stdin")
                        .expect("error reading from stdin.");
        match line.parse() {
            Ok(c) if c < self.hand.len() => self.hand.remove(c),
            Ok(c) => {
                println!("choice must be in range [0, {}], given {}.",
                         self.hand.len() - 1,
                         c);
                self.play()
            }
            Err(e) => {
                println!("{}, try again.", e);
                self.play()
            }
        }
    }

    fn choose(&self) -> StackId {
        print!("Your hand is: ");
        for card in self.hand() {
            print!("{}, ", card);
        }
        print!("\nChoose a stack (A, B, C, or D): ");
        io::stdout().flush().expect("error writing to stdout.");
        let stdin = io::stdin();
        let line = stdin.lock()
                        .lines()
                        .next()
                        .expect("no more lines in stdin")
                        .expect("error reading from stdin.");
        match &line[..] {
            "A" => StackId::A,
            "B" => StackId::B,
            "C" => StackId::C,
            "D" => StackId::D,
            n @ _ => {
                println!("choice must be one of A, B, C, or D, given {}.", n);
                self.choose()
            }
        }
    }

    fn take(&mut self, cards: Vec<Card>) {
        self.pile.extend(cards);
    }
}

impl fmt::Display for StdinPlayer {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "STDIN Player {}", self.id())
    }
}
