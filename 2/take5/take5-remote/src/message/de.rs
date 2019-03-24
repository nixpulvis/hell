use serde::{Deserialize, Deserializer};
use serde::de::{Visitor, SeqVisitor, Error};
use super::RequestMessage;

struct RequestMessageVisitor;

impl RequestMessageVisitor {
    fn visit_start_round<V>(&mut self, mut visitor: V) -> Result<RequestMessage, V::Error>
        where V: SeqVisitor
    {
        match try!(visitor.visit()) {
            Some(cards) => {
                try!(visitor.end());
                Ok(RequestMessage::StartRound(cards))
            },
            None => {
                Err(Error::syntax("second element must be valid cards"))
            },
        }
    }

    fn visit_take_turn<V>(&mut self, mut visitor: V) -> Result<RequestMessage, V::Error>
        where V: SeqVisitor
    {
        match try!(visitor.visit()) {
            Some(board) => {
                try!(visitor.end());
                Ok(RequestMessage::TakeTurn(board))
            },
            None => {
                Err(Error::syntax("second element must be valid board"))
            },
        }
    }

    fn visit_choose<V>(&mut self, mut visitor: V) -> Result<RequestMessage, V::Error>
        where V: SeqVisitor
    {
        match try!(visitor.visit()) {
            Some(board) => {
                try!(visitor.end());
                Ok(RequestMessage::Choose(board))
            },
            None => {
                Err(Error::syntax("second element must be valid board"))
            },
        }
    }
}

impl Visitor for RequestMessageVisitor {
    type Value = RequestMessage;

    fn visit_seq<V>(&mut self, mut visitor: V) -> Result<Self::Value, V::Error>
        where V: SeqVisitor
    {
        match try!(visitor.visit::<String>()) {
            Some(message_type) => {
                match &message_type[..] {
                    "start-round" => {
                        self.visit_start_round(visitor)
                    },
                    "take-turn" => {
                        self.visit_take_turn(visitor)
                    },
                    "choose" => {
                        self.visit_choose(visitor)
                    },
                    _ => Err(Error::syntax("invalid message type")),
                }
            },
            None => Err(Error::syntax("first element must be a string"))
        }
    }
}

impl Deserialize for RequestMessage {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        deserializer.visit(RequestMessageVisitor)
    }
}

#[cfg(test)]
mod tests {
    use serde_json as json;
    use take5::StackId;
    use message::RequestMessage;

    #[test]
    fn test_request_decode_start_round() {
        let json = "[\"start-round\",[[1,2],[2,2],[3,2],[4,2],[5,2],[6,2],[7,2],[8,2],[9,2],[10,2]]]";
        let msg = json::from_str::<RequestMessage>(json).unwrap();
        match msg {
            RequestMessage::StartRound(cards) => {
                assert_eq!(cards.take().len(), 10);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn test_request_decode_start_round_wrong_number_of_cards() {
        let json = "[\"start-round\",[[1,2],[2,2]]]";
        assert!(json::from_str::<RequestMessage>(json).is_err());
    }

    #[test]
    fn test_request_decode_take_turn() {
        let json = "[\"take-turn\",[[[2,2]],[[2,2]],[[2,2]],[[2,2]]]]";
        let msg = json::from_str::<RequestMessage>(json).unwrap();
        match msg {
            RequestMessage::TakeTurn(board) => {
                assert_eq!(board[StackId::A].len(), 1);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn test_request_decode_take_turn_wrong_number_of_stacks() {
        let json1 = "[\"take-turn\",[[2,2],[2,2],[2,2]]";
        let json2 = "[\"take-turn\",[[2,2],[2,2],[2,2],[2,2],[2,2]]";
        assert!(json::from_str::<RequestMessage>(json1).is_err());
        assert!(json::from_str::<RequestMessage>(json2).is_err());
    }

    #[test]
    fn test_request_decode_choose() {
        let json = "[\"choose\",[[[2,2]],[[2,2]],[[2,2]],[[2,2]]]]";
        let msg = json::from_str::<RequestMessage>(json).unwrap();
        match msg {
            RequestMessage::Choose(board) => {
                assert_eq!(board[StackId::A].len(), 1);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn test_request_decode_choose_wrong_number_of_stacks() {
        let json1 = "[\"choose\",[[[2,2]],[[2,2]],[[2,2]]]";
        let json2 = "[\"choose\",[[[2,2]],[[2,2]],[[2,2]],[[2,2]],[[2,2]]]";
        assert!(json::from_str::<RequestMessage>(json1).is_err());
        assert!(json::from_str::<RequestMessage>(json2).is_err());
    }
}
