use std::cmp::PartialOrd;
use std::cmp::Ordering;
use std::cmp::Ordering::*;
use std::ops::Range;
use object::*;

const NOT_CARNIVORES: [Trait; 15] = [Trait::Ambush, Trait::Burrowing,
    Trait::Climbing, Trait::Cooperation, Trait::FatTissue, Trait::Fertile,
    Trait::Foraging, Trait::HardShell, Trait::Herding, Trait::Horns,
    Trait::LongNeck, Trait::PackHunting, Trait::Scavenger,
    Trait::Symbiosis, Trait::WarningCall];

const CARNIVORE_CARD_BOUNDS: Range<i64> = Range {
    start: -8,
    end: 9,
};

const NON_CARNIVORE_CARD_BOUNDS: Range<i64> = Range {
    start: -3,
    end: 4,
};

/// A card in evolution.
///
/// There are are generally 17 carnivores and 7 of each other trait
/// in a deck which makes 122 cards.
///
/// Cards has both a trait, and a number of food tokens the card is worth.
/// The card's food values range from -8 to 8 for carnivores and from -3
/// to 3 for all other cards.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Card(pub i64, pub Trait);

impl Card {
    /// Returns a new deck of all the cards for a game. This is the main
    /// way to create cards for the game, otherwise we expect to get messages
    /// containing serialized cards, and deserialize them.
    ///
    /// # Examples
    ///
    /// ```
    /// use evolution::object::*;
    ///
    /// let deck = Card::deck();
    /// assert_eq!(122, deck.len());
    /// ```
    pub fn deck() -> Vec<Card> {
        let mut cards = Vec::new();

        for food_value in CARNIVORE_CARD_BOUNDS {
            cards.push(Card(food_value, Trait::Carnivore));
        }

        for food_value in NON_CARNIVORE_CARD_BOUNDS {
            for trait_type in NOT_CARNIVORES.iter() {
                cards.push(Card(food_value, *trait_type));
            }
        }

        cards.sort();
        cards
    }

    /// The trait of this card. If this card is used to evolve a species,
    /// that species will get this trait.
    pub fn trait_type(&self) -> Trait {
        self.1
    }

    /// The number of food tokens this card is worth (may be negative)
    /// if this card is used during the dealing phase.
    pub fn food_value(&self) -> i64 {
        self.0
    }

    /// Returns true if this card has the carnivore trait.
    pub fn is_carnivore(&self) -> bool {
        self.trait_type() == Trait::Carnivore
    }

    /// Returns true if this card does not have the carnivore trait.
    pub fn is_non_carnivore(&self) -> bool {
        !self.is_carnivore()
    }
}

// Matthias's code below and inline.
//
// (define (<-card c1 c2)
//   (match-define (card f1 t1) c1)
//   (match-define (card f2 t2) c2)
//   (or (<-trait t1 t2) (and (equal? t1 t2) (< f1 f2))))
//
// (define (<-card c1 c2)
//
// Here the notion of `<-card` is encoded in the type system as
// `impl Ord for Card` and that trait requires a a function named `cmp`,
// which takes in two `&Card`s.
impl Ord for Card {
    fn cmp(&self, other: &Card) -> Ordering {
        //   (match-define (card f1 t1) c1)
        //   (match-define (card f2 t2) c2)
        //
        // This is a straightforward translation. `let` in Rust destructures
        // just as `match-define` does. In fact our data models for the game
        // must be practically identical.
        let Card(f1, t1) = *self;
        let Card(f2, t2) = *other;
        // This line I'll line up to show how cool it is that these two
        // different languages encode the idea of order in a similar, yet
        // different way.
        //
        // (or (<-trait t1 t2) (and (equal? t1 t2) (< f1 f2))))
           match t1.cmp(&t2) { Equal => f1.cmp(&f2), o => o }
        // Rust's is shorter :P
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Card) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(feature = "wire")]
mod wire;

#[cfg(test)]
mod tests {
    use object::*;

    impl Card {
        pub fn mock(food_value: i64, trait_type: Trait) -> Self {
            Card(food_value, trait_type)
        }
    }

    #[test]
    fn deck_length() {
        let deck = Card::deck();
        assert_eq!(122, deck.len());
    }

    #[test]
    fn deck_carnivore_length() {
        let deck = Card::deck();
        let count = deck.into_iter().filter(Card::is_carnivore).count();
        assert_eq!(17, count);
    }

    #[test]
    fn deck_non_carnivore_length() {
        let deck = Card::deck();
        let count = deck.into_iter().filter(Card::is_non_carnivore).count();
        assert_eq!(105, count);
    }

    #[test]
    fn deck_non_carnivore_counts() {
        use std::collections::HashMap;

        let deck = Card::deck();
        let map = deck.into_iter().filter(|card| {
            card.trait_type() != Trait::Carnivore
        }).fold(HashMap::new(), |mut map, card| {
            *map.entry(card.trait_type()).or_insert(0) += 1;
            map
        });
        for (_, count) in map {
            assert_eq!(7, count);
        }
    }

    #[test]
    fn deck_no_non_carnivores_outside_bounds() {
        let deck = Card::deck();
        for Card(f, t) in deck.into_iter() {
            if f < super::NON_CARNIVORE_CARD_BOUNDS.start ||
               f >= super::NON_CARNIVORE_CARD_BOUNDS.end
            {
                assert_eq!(Trait::Carnivore, t);
            }
        }
    }

    #[test]
    fn ordering_of_trait_takes_precedence() {
        assert!(Card(3, Trait::Burrowing) < Card(-2, Trait::WarningCall));
        assert!(Card(2, Trait::Climbing) > Card(2, Trait::Carnivore));
    }

    #[test]
    fn ordering_of_value_is_used_if_traits_are_equal() {
        assert!(Card(-3, Trait::Climbing) < Card(2, Trait::Climbing));
        assert!(Card(0, Trait::Climbing) > Card(-1, Trait::Climbing));
    }

    #[test]
    fn ordering_of_equivalent_cards() {
        let a = Card(2, Trait::FatTissue);
        let b = Card(2, Trait::FatTissue);
        assert!(a == b);
        assert!(a <= b);
        assert!(a >= b);
    }

    #[test]
    fn deck_is_sorted() {
        let deck = Card::deck();

        // NOTE: Rust should have a `sorted(Ordering)`, and
        // `sorted_by(FnMut(&T, &T) -> Ordering)` method.
        //
        // NOTE: "refutable pattern in `for` loop binding: `[]` not covered"
        // rustc 1.8.0-nightly (b94cd7a5b 2016-02-01)
        for window in deck.windows(2) {
            // NOTE: rustc panic on slice pattern `let [c1, c2] = window;`.
            // thread 'rustc' panicked at 'assertion failed: `(left == right)`
            // (left: `3`, right: `1`)', ../src/librustc/middle/check_match.rs:1060
            let (c1, c2) = (&window[0], &window[1]);
            assert!(c1 <= c2);
        }
    }
}
