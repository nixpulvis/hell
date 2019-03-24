use std::fmt;
use card::Card;
use board::{Board, StackId};

pub use self::ai_player::AiPlayer;

/// A unique identifier for a player in the game.
pub type Id = u8;

/// A player draws and plays cards in an attempt to win the game.
///
/// Implementations of `Player` must give some way to create themselves,
/// this is not spelled out in the interface of a player as this function
/// might be vary different for different types of players.
pub trait Player: fmt::Display {
    /// Returns the id of a player.
    ///
    /// # Examples
    ///
    /// ```
    /// use take5::Player;
    /// use take5::player::AiPlayer;
    ///
    /// let ai = AiPlayer::new(1);
    /// assert_eq!(ai.id(), 1);
    /// ```
    fn id(&self) -> Id;

    /// Returns the set of cards the player is holding.
    ///
    /// # Examples
    ///
    /// ```
    /// use take5::{Card, Player};
    /// use take5::player::AiPlayer;
    ///
    /// let mut deck = Card::deck(|_| 3);
    /// let mut ai = AiPlayer::new(1);
    ///
    /// // Starts without any cards in it's hand.
    /// assert!(ai.hand().is_empty());
    /// // Draws a card.
    /// ai.draw(deck.pop().unwrap());
    /// // Now the player has one card in it's hand.
    /// assert_eq!(ai.hand().len(), 1);
    /// ```
    fn hand(&self) -> &[Card];

    /// Returns the set of cards the player has taken, these will count
    /// towards their score.
    ///
    /// # Examples
    ///
    /// ```
    /// use take5::{Card, Player};
    /// use take5::player::AiPlayer;
    ///
    /// let mut deck = Card::deck(|_| 3);
    /// let mut ai = AiPlayer::new(1);
    ///
    /// // Starts without any cards in it's pile.
    /// assert!(ai.pile().is_empty());
    /// // Takes five cards.
    /// deck.split_off(5);
    /// ai.take(deck);
    /// // Now the player has five cards in it's pile.
    /// assert_eq!(ai.pile().len(), 5);
    /// ```
    fn pile(&self) -> &[Card];

    /// Draws a card into the player's hand.
    ///
    /// # Examples
    ///
    /// ```
    /// use take5::{Card, Player};
    /// use take5::player::AiPlayer;
    ///
    /// let mut deck = Card::deck(|_| 3);
    /// let mut ai = AiPlayer::new(1);
    ///
    /// // Draw 10 cards.
    /// for _ in 0..10 {
    ///     ai.draw(deck.pop().unwrap());
    /// }
    /// assert_eq!(ai.hand().len(), 10);
    /// ```
    fn draw(&mut self, Card);

    /// Gives a player the board to look at.
    ///
    /// # Examples
    ///
    /// ```
    /// use take5::{Card, Board, Player, StackId};
    /// use take5::player::AiPlayer;
    ///
    /// let mut deck = Card::deck(|_| 3);
    /// let mut ai = AiPlayer::new(1);
    ///
    /// // Setup a game.
    /// let mut board = Board::default();
    /// for stack in &mut board {
    ///     stack.push(deck.pop().unwrap());
    /// }
    ///
    /// // Look at the board.
    /// ai.look_at_board(&board);
    /// ```
    fn look_at_board(&mut self, &Board);

    /// Gives a player the played cards to look at. The played cards are
    /// given as pairs of the card, and the `Id` of the player who played
    /// the card.
    ///
    /// # Examples
    ///
    /// ```
    /// use take5::{Card, Board, Player, StackId};
    /// use take5::player::AiPlayer;
    ///
    /// let mut deck = Card::deck(|_| 3);
    /// let mut ai = AiPlayer::new(1);
    ///
    /// // Setup a game.
    /// let mut played = Vec::new();
    /// for i in 1..6 {
    ///     played.push((deck.remove(0), i));
    /// }
    ///
    /// // Look at the played cards.
    /// ai.look_at_played(&played);
    /// ```
    fn look_at_played(&mut self, &[(Card, Id)]);

    /// Gives a player the taken cards to look at. The taken cards are
    /// given as a collection of cards and the `Id` of the player taking
    /// them.
    ///
    /// # Examples
    ///
    /// ```
    /// use take5::{Card, Board, Player, StackId};
    /// use take5::player::AiPlayer;
    ///
    /// let mut deck = Card::deck(|_| 3);
    /// let mut ai = AiPlayer::new(1);
    ///
    /// // Setup a game.
    /// let mut taken = Vec::new();
    /// for i in 0..5 {
    ///     taken.push(deck.remove(0));
    /// }
    ///
    /// // Look at taken cards.
    /// ai.look_at_taken((&taken, 1));
    /// ```
    fn look_at_taken(&mut self, (&[Card], Id));

    /// Returns a card the player wishes to play.
    ///
    /// # Panics
    ///
    /// Player implementations are allowed to panic if there are no cards
    /// in any stack of the board. The board can be expected to have at least
    /// one card in each stack. This is the responsibility of the dealer
    /// to maintain.
    ///
    /// Player implementations may also panic if asked to play a card when
    /// they are out of cards. The dealer should never ask this of a player
    /// since the number of turns in a round is fixed to 10.
    ///
    /// # Examples
    ///
    /// ```
    /// use take5::{Card, Board, Player, StackId};
    /// use take5::player::AiPlayer;
    ///
    /// let mut deck = Card::deck(|_| 3);
    /// let mut ai = AiPlayer::new(1);
    ///
    /// // Draw a card.
    /// let card = deck.pop().unwrap();
    /// let face = card.face();
    /// ai.draw(card);
    /// // With only one card, the player must play it.
    /// assert_eq!(ai.play().face(), face);
    /// ```
    fn play(&mut self) -> Card;

    /// If a played card is lower than all cards on the board, return
    /// the `StackId` of the stack you wish to take.
    ///
    /// # Panics
    ///
    /// Player implementations are allowed to panic if there are no cards
    /// in any stack of the board. The board can be expected to have at least
    /// one card in each stack. This is the responsibility of the dealer
    /// to maintain.
    ///
    /// # Examples
    ///
    /// ```
    /// use take5::{Card, Board, Player, StackId};
    /// use take5::player::AiPlayer;
    ///
    /// let mut deck = Card::deck(|_| 3);
    /// let mut ai = AiPlayer::new(1);
    ///
    /// // Setup a board.
    /// let mut board = Board::default();
    /// for stack in &mut board {
    ///     stack.push(deck.pop().unwrap());
    /// }
    ///
    /// // Print the stack this player chose.
    /// println!("{}", board[ai.choose()]);
    /// ```
    fn choose(&self) -> StackId;

    /// Take a set of cards for the player to add to their pile.
    ///
    /// # Examples
    ///
    /// ```
    /// use take5::{Card, Player};
    /// use take5::player::AiPlayer;
    ///
    /// let mut deck = Card::deck(|_| 3);
    /// let mut ai = AiPlayer::new(1);
    ///
    /// // Takes five cards.
    /// let len = ai.pile().len();
    /// deck.split_off(5);
    /// ai.take(deck);
    /// assert_eq!(ai.pile().len(), len + 5);
    /// ```
    fn take(&mut self, Vec<Card>);

    /// Returns the current score given the pile of cards this player
    /// has taken. Score is calculated by summing each of the player's
    /// card's bull value in the pile.
    ///
    /// # Examples
    ///
    /// ```
    /// use take5::{Card, Player};
    /// use take5::player::AiPlayer;
    ///
    /// let mut deck = Card::deck(|_| 3);
    /// let mut ai = AiPlayer::new(1);
    ///
    /// // Takes five cards.
    /// deck.split_off(5);
    /// ai.take(deck);
    /// assert_eq!(ai.score(), 15);
    /// ```
    fn score(&self) -> u32 {
        self.pile().iter().fold(0, |s, c| s + c.bull() as u32)
    }
}

mod ai_player;
