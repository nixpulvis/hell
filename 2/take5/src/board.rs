use std::{fmt, ops};
use std::slice::{Iter, IterMut};
use itertools::Itertools;
use card::Card;
use stack::Stack;

/// A type representing all of the stacks in the board.
///
/// The board consists of four stacks, `A`, `B`, `C`, and `D`. These are
/// accessed by indexing the board with the appropriate variant of this
/// enum. For example `board[StackId::A]`. You can also iterate over the
/// stacks on the board with `for stack in &board ...`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum StackId {
    A,
    B,
    C,
    D,
}

impl StackId {
    /// Returns a static array of the variants of `StackId`. This is mainly
    /// useful for iterating over the possible stacks.
    ///
    /// # Examples
    ///
    /// ```
    /// use take5::{Card, Board, StackId};
    ///
    /// let mut deck = Card::deck(|_| 3);
    ///
    /// // Setup a board.
    /// let mut board = Board::default();
    /// for stack in &mut board {
    ///     stack.push(deck.pop().unwrap());
    /// }
    ///
    /// // Print each stack in the board.
    /// for stack in &mut board {
    ///     println!("{:?}", stack);
    /// }
    /// ```
    pub fn all() -> [StackId; 4] {
        [StackId::A, StackId::B, StackId::C, StackId::D]
    }
}

/// A board represents the playing field of take5, with 4 stacks of cards.
///
/// The best way to think of the board is as an array of four stacks.
/// Access to the stacks is done via indexing with `StackId`s, and stacks
/// act much other collections, containing cards.
///
/// # Examples
///
/// ```
/// use take5::{Board, Card};
///
/// let mut deck = Card::deck(|_| 3);
/// let mut board = Board::default();
///
/// // Push two cards onto each stack.
/// for stack in &mut board {
///     stack.push(deck.remove(0));
///     stack.push(deck.remove(0));
/// }
///
/// // Print the stacks total bull value.
/// for stack in &board {
///     println!("{}", stack.bulls());
/// }
/// ```
#[derive(Debug, Serialize)]
pub struct Board([Stack; 4]);

impl Board {
    /// Returns the id of the stack which has the largest face value,
    /// smaller than the given card. If there is no such card this
    /// function returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use take5::{Card, Board, StackId};
    ///
    /// let mut deck = Card::deck(|_| 3);
    /// let mut board = Board::default();
    /// for stack in &mut board {
    ///     stack.push(deck.remove(1));
    /// }
    /// let card0 = deck.remove(0);
    /// let card1 = deck.remove(0);
    /// assert_eq!(board.closest_smaller(&card0), None);
    /// assert_eq!(board.closest_smaller(&card1), Some(StackId::D));
    /// ```
    pub fn closest_smaller(&self, card: &Card) -> Option<StackId> {
        let mut larger_than = Vec::new();
        for id in StackId::all().iter() {
            let last = self[*id].last().expect("stack doesn't have any cards.");
            if card.face() > last.face() {
                larger_than.push((*id, last.face()));
            }
        }
        if larger_than.len() > 0 {
            larger_than.sort_by(|a, b| a.1.cmp(&b.1));
            Some(larger_than.last().expect("larger_than must have an element.").0)
        } else {
            None
        }
    }

    /// Clears the board, removing all cards from all stacks.
    ///
    /// # Examples
    ///
    /// ```
    /// use take5::{Card, Board, StackId};
    ///
    /// let mut deck = Card::deck(|_| 3);
    ///
    /// // Setup a board.
    /// let mut board = Board::default();
    /// for stack in &mut board {
    ///     stack.push(deck.remove(1));
    /// }
    /// assert!(!board[StackId::A].is_empty());
    /// board.clear();
    /// assert!(board[StackId::A].is_empty());
    /// ```
    pub fn clear(&mut self) {
        for stack in self {
            stack.clear();
        }
    }

    fn id(id: StackId) -> usize {
        match id {
            StackId::A => 0,
            StackId::B => 1,
            StackId::C => 2,
            StackId::D => 3,
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Board([Stack::default(), Stack::default(), Stack::default(), Stack::default()])
    }
}

impl ops::Index<StackId> for Board {
    type Output = Stack;

    fn index(&self, index: StackId) -> &Stack {
        &self.0[Board::id(index)]
    }
}

impl ops::IndexMut<StackId> for Board {
    fn index_mut(&mut self, index: StackId) -> &mut Stack {
        &mut self.0[Board::id(index)]
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.into_iter().join("\n"))
    }
}

impl<'a> IntoIterator for &'a Board {
    type Item = &'a Stack;
    type IntoIter = Iter<'a, Stack>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut Board {
    type Item = &'a mut Stack;
    type IntoIter = IterMut<'a, Stack>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}
