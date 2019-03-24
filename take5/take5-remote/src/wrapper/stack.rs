use take5;
use serde::{Deserialize, Deserializer};
use serde::de::Error;
use serde::de::impls::VecVisitor;
use wrapper::Card;

create_wrapper!(Stack);

impl Stack {
    pub fn new(cards: Vec<Card>) -> Self {
        let mut wrapped = take5::Stack::default();
        for card in cards {
            wrapped.push(card.into());
        }
        Stack(wrapped)
    }
}

impl Deserialize for Stack {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer,
    {
        let cards = try!(deserializer.visit_seq(VecVisitor::new()));
        if cards.len() < 1 {
            return Err(Error::length_mismatch(cards.len()))
        }
        Ok(Stack::new(cards))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json as json;
    use wrapper::Card;

    #[test]
    fn test_decode() {
        let json = "[[4,2],[5,4]]";
        let stack: Stack = json::from_str(json).unwrap();
        assert!(!stack.full());
        assert_eq!(6, stack.bulls());
    }

    #[test]
    fn test_empty_decode() {
        let json = "[]";
        assert!(json::from_str::<Stack>(json).is_err());
    }

    #[test]
    fn test_decode_bad_card() {
        let json = "[[4,2],[5,4,6]]";
        let stack = json::from_str::<Stack>(json);
        assert!(stack.is_err());
    }

    #[test]
    fn test_decode_err() {
        let json = "[[4,2],[5,4],'hello world']";
        let stack = json::from_str::<Stack>(json);
        assert!(stack.is_err());
    }

    #[test]
    fn test_encode() {
        let mut deck = Card::deck(|_| 2);
        let stack = Stack::new(vec![deck.remove(0), deck.remove(0)]);
        let json = json::to_string(&stack).unwrap();
        assert_eq!("[[1,2],[2,2]]", json);
    }
}
