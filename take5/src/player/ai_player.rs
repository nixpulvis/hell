use std::fmt;
use card::Card;
use board::{Board, StackId};
use player::{Player, Id};

/// A computer player, implemented as a simple AI.
///
/// Creating an `AiPlayer` is done with `AiPlayer::new()`.
#[derive(Debug)]
pub struct AiPlayer {
    id: Id,
    hand: Vec<Card>,
    pile: Vec<Card>,
    smallest_stack: StackId,
}

impl AiPlayer {
    /// Creates a new AI player with the given ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use take5::player::AiPlayer;
    ///
    /// let _ = AiPlayer::new(1);
    /// ```
    pub fn new(id: Id) -> Self {
        AiPlayer {
            id: id,
            hand: Vec::new(),
            pile: Vec::new(),
            smallest_stack: StackId::A,
        }
    }
}

impl Player for AiPlayer {
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
        for id in StackId::all().iter() {
            if board[*id].bulls() < board[self.smallest_stack].bulls() {
                self.smallest_stack = *id;
            }
        }
    }

    fn look_at_played(&mut self, _played: &[(Card, Id)]) {
        // Do nothing.
    }

    fn look_at_taken(&mut self, _taken: (&[Card], Id)) {
        // Do nothing.
    }

    fn play(&mut self) -> Card {
        self.hand.sort_by(|a, b| b.cmp(a));
        self.hand.pop().expect("player ran out of cards.")
    }

    fn choose(&self) -> StackId {
        self.smallest_stack
    }

    fn take(&mut self, cards: Vec<Card>) {
        self.pile.extend(cards);
    }
}

impl fmt::Display for AiPlayer {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "AI Player {}", self.id())
    }
}
