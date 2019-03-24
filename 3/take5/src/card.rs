use std::fmt;
use configuration::Configuration;

/// Playing cards for the game.
///
/// Cards are intentionally only available by creating a whole deck. This
/// follows from reality, as you don't ever create a single card from a
/// deck.
///
/// Cards in take5 have two properties, the `face` and the `bull`. The face
/// value is a number between 1 and 104 inclusive. This number uniquely
/// identifies the card. The bull value is the number of points (remember
/// points are bad) that a card is worth.
///
/// # Examples
///
/// ```
/// use take5::Card;
///
/// let deck = Card::deck(|_| 3);
/// println!("{:?}", deck[10]);
/// ```
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize)]
pub struct Card(u8, u8);

impl Card {
    pub fn new(f: u8, b: u8) -> Self {
        Card(f, b)
    }
    /// Given a function mapping face values to bull values,
    /// returns all 104 cards of the take5 deck.
    ///
    /// # Panics
    ///
    /// This function panics if a card would have a bull value not in the
    /// range of `Configuration::bull_range()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use take5::{Card, Configuration};
    ///
    /// // A deck who's bull values are 2 for the lower half the deck and
    /// // three for the higher half.
    /// let deck = Card::deck(|face| if face < 52 { 4 } else { 3 });
    /// assert_eq!(deck.len(), Configuration::deck_size() as usize);
    /// assert_eq!(deck[30].bull(), 4);
    /// assert_eq!(deck[70].bull(), 3);
    /// ```
    pub fn deck<F>(f: F) -> Vec<Card>
        where F: Fn(u8) -> u8
    {
        (1..Configuration::deck_size() + 1)
            .map(|i| {
                let bull = f(i);
                if bull < Configuration::bull_range().0 ||
                   bull > Configuration::bull_range().1 {
                    panic!("Bull values must be in range [{}..{}] was {}.",
                           Configuration::bull_range().0,
                           Configuration::bull_range().1,
                           bull);
                }
                Card(i, bull)
            })
            .collect()
    }

    /// Returns the face value of this card.
    ///
    /// # Examples
    ///
    /// ```
    /// use take5::Card;
    ///
    /// let deck = Card::deck(|_| 3);
    /// assert_eq!(deck[0].face(), 1);
    /// ```
    pub fn face(&self) -> u8 {
        self.0
    }

    /// Returns the bull value of this card.
    ///
    /// # Examples
    ///
    /// ```
    /// use take5::Card;
    ///
    /// let deck = Card::deck(|_| 3);
    /// assert_eq!(deck[54].face(), 55);
    /// assert_eq!(deck[54].bull(), 3);
    /// ```
    pub fn bull(&self) -> u8 {
        self.1
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "f{}b{}", self.face(), self.bull())
    }
}
