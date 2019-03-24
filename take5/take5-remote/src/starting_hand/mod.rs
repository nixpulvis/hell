use take5::Configuration;
use wrapper::Card;

/// A hand which must have `take5::Configuration::turn_count()` cards
/// in it.
#[derive(Debug, Serialize)]
pub struct StartingHand(Vec<Card>);

impl StartingHand {
    pub fn new(cards: Vec<Card>) -> Result<Self, err::Error> {
        if cards.len() != Configuration::turn_count() {
            Err(err::Error::CardLength(cards.len()))
        } else {
            Ok(StartingHand(cards))
        }
    }

    pub fn take(self) -> Vec<Card> {
        self.0
    }
}

mod err;
mod de;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json as json;
    use wrapper::Card;

    #[test]
    fn test_new() {
        let mut cards = Card::deck(|_| 3);
        cards.split_off(10);
        let hand = StartingHand::new(cards);
        assert!(hand.is_ok());
    }

    #[test]
    fn test_new_too_few() {
        let mut cards = Card::deck(|_| 3);
        cards.split_off(9);
        let hand = StartingHand::new(cards);
        assert!(hand.is_err());
    }

    #[test]
    fn test_new_too_many() {
        let mut cards = Card::deck(|_| 3);
        cards.split_off(11);
        let hand = StartingHand::new(cards);
        assert!(hand.is_err());
    }

    #[test]
    fn test_encode_starting_hand() {
        let mut cards = Card::deck(|_| 2);
        cards.split_off(10);
        let hand = StartingHand::new(cards).unwrap();
        let json = json::to_string(&hand).unwrap();
        assert_eq!("[[1,2],[2,2],[3,2],[4,2],[5,2],[6,2],[7,2],[8,2],[9,2],[10,2]]", json);
    }

    #[test]
    fn test_decode_starting_hand() {
        let json = "[[1,2],[2,2],[3,2],[4,2],[5,2],[6,2],[7,2],[8,2],[9,2],[10,2]]";
        let hand = json::from_str::<StartingHand>(json).unwrap();
        assert_eq!(10, hand.take().len());
    }

    #[test]
    fn test_decode_starting_hand_too_few() {
        let json = "[[1,2],[2,2],[3,2],[4,2],[5,2],[6,2],[7,2],[8,2],[9,2]]";
        let result = json::from_str::<StartingHand>(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_decode_starting_hand_too_many() {
        let json = "[[1,2],[2,2],[3,2],[4,2],[5,2],[6,2],[7,2],[8,2],[9,2],[10,2],[11,2]]";
        let result = json::from_str::<StartingHand>(json);
        assert!(result.is_err());
    }
}
