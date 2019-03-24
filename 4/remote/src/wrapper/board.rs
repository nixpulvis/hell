use take5;
use serde::{Deserialize, Deserializer};
use serde::de::Error;
use serde::de::impls::VecVisitor;
use wrapper::Stack;

create_wrapper!(Board);

impl Board {
    pub fn new(stacks: [Stack; 4]) -> Self {
        let mut wrapped = take5::Board::default();
        for (stack, id) in stacks.into_iter().zip(take5::StackId::all().into_iter()) {
            wrapped[*id].extend_from_slice(stack);
        }
        Board(wrapped)
    }

    pub fn take(self, id: take5::StackId) -> take5::Stack {
        self.0[id].clone()
    }
}

impl Deserialize for Board {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer,
    {
        let mut stacks = try!(deserializer.visit_seq(VecVisitor::new()));
        if stacks.len() != 4 {
            return Err(Error::length_mismatch(stacks.len()))
        }
        let stacks_array = [
            stacks.remove(0),
            stacks.remove(0),
            stacks.remove(0),
            stacks.remove(0),
        ];
        Ok(Board::new(stacks_array))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json as json;
    use take5;
    use wrapper::{Card, Stack};

    fn board() -> Board {
        let mut deck = Card::deck(|_| 2);
        let stacks = [
            Stack::new(vec![deck.remove(0)]),
            Stack::new(vec![deck.remove(0)]),
            Stack::new(vec![deck.remove(0)]),
            Stack::new(vec![deck.remove(0)]),
        ];
        Board::new(stacks)
    }

    #[test]
    fn test_decode() {
        let json = "[[[4,2],[5,2]],[[9,2],[7,2]],[[10,2],[11,2]],[[12,2],[13,2]]]";
        let board = json::from_str::<Board>(json).unwrap();
        assert_eq!(4, board[take5::StackId::A].bulls())
    }

    #[test]
    fn test_decode_small() {
        let json = "[[[4,2],[5,2]],[[9,2],[7,2]]]";
        let board = json::from_str::<Board>(json);
        assert!(board.is_err());
    }

    #[test]
    fn test_decode_large() {
        let json = "[[[4,2],[5,2]],[[9,2],[7,2]],[[4,2],[5,2]],[[4,2],[5,2]],[[4,2],[5,2]],[[4,2],[5,2]]]";
        let board = json::from_str::<Board>(json);
        assert!(board.is_err());
    }

    #[test]
    fn test_decode_empty() {
        let json = "[]";
        let board = json::from_str::<Board>(json);
        assert!(board.is_err());
    }

    #[test]
    fn test_decode_bad_card() {
        let json = "[[[4,2],[5,2],[6,4]],[[9,2],[4,'5'][7,2]],[[10,2],[11,2]],[[12,2],[13,2]]]";
        let board = json::from_str::<Board>(json);
        assert!(board.is_err());
    }

    #[test]
    fn test_decode_bad_stack() {
        let json = "[[[4,2],[5,2],'hello'],[[9,2],[4,'5'][7,2]],[[10,2],[11,2]],[[12,2],[13,2]]]";
        let board = json::from_str::<Board>(json);
        assert!(board.is_err());
    }

    #[test]
    fn test_encode() {
        let json = json::to_string(&board()).unwrap();
        assert_eq!("[[[1,2]],[[2,2]],[[3,2]],[[4,2]]]", json);
    }
}
