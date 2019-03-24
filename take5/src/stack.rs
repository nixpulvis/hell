use std::{ops, fmt};
use card::Card;
use configuration::Configuration;

/// A stack of cards you can put cards on and take all of.
///
/// Don't think of this as a general purpose computer science stack, it's
/// a more specific construct. This stack can only have 5 cards in it,
/// and often gives up all 5 cards and replaces them with a new empty stack.
#[derive(Debug, Default, Clone, Serialize)]
pub struct Stack(Vec<Card>);

impl Stack {
    /// Returns true when this stack has 5 elements in it.
    ///
    /// # Examples
    ///
    /// ```
    /// use take5::{Card, Stack, Configuration};
    ///
    /// let mut deck = Card::deck(|_| 3);
    /// let mut stack = Stack::default();
    /// assert!(!stack.full());
    /// for _ in 0..Configuration::stack_size() {
    ///     stack.push(deck.pop().unwrap());
    /// }
    /// assert!(stack.full());
    /// ```
    pub fn full(&self) -> bool {
        self.0.len() == Configuration::stack_size()
    }

    /// Returns the sum of the bull values of cards in the stack.
    ///
    /// # Examples
    ///
    /// ```
    /// use take5::{Card, Stack};
    ///
    /// let mut deck = Card::deck(|_| 3);
    /// let mut stack = Stack::default();
    /// for _ in 0..5 {
    ///     stack.push(deck.pop().unwrap());
    /// }
    /// assert!(stack.bulls() > 0);
    /// ```
    pub fn bulls(&self) -> u32 {
        self.0.iter().fold(0, |a, c| a + c.bull() as u32)
    }

    /// Add a card to the top of the stack.
    ///
    /// # Panics
    ///
    /// This function panics if pushing onto a full stack.
    ///
    /// # Examples
    ///
    /// ```
    /// use take5::{Card, Stack, Configuration};
    ///
    /// let mut deck = Card::deck(|_| 3);
    /// let mut stack = Stack::default();
    /// for _ in 0..Configuration::stack_size() {
    ///     stack.push(deck.pop().unwrap());
    /// }
    /// assert_eq!(stack.len(), Configuration::stack_size());
    /// ```
    pub fn push(&mut self, card: Card) {
        if self.full() {
            panic!("Attempt to push onto a full stack.");
        }
        self.0.push(card);
    }

    /// Returns the stack as a vector of cards, and creates a new empty
    /// vector for the stack.
    ///
    /// # Examples
    ///
    /// ```
    /// use take5::{Card, Stack};
    ///
    /// let mut deck = Card::deck(|_| 3);
    /// let mut stack = Stack::default();
    /// for _ in 0..5 {
    ///     stack.push(deck.pop().unwrap());
    /// }
    /// let cards = stack.give();
    /// assert_eq!(cards.len(), 5);
    /// assert_eq!(stack.len(), 0);
    /// ```
    pub fn give(&mut self) -> Vec<Card> {
        let mut vec = Vec::new();
        ::std::mem::swap(&mut vec, &mut self.0);
        vec
    }
}

impl ops::Deref for Stack {
    type Target = Vec<Card>;

    fn deref(&self) -> &Vec<Card> {
        &self.0
    }
}

impl ops::DerefMut for Stack {
    fn deref_mut(&mut self) -> &mut Vec<Card> {
        &mut self.0
    }
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for card in self.iter() {
            try!(write!(f, "{} ", card));
        }
        Ok(())
    }
}
