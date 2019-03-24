use take5;
use serde::{Deserialize, Deserializer};
use serde::de::Error;
use serde::de::impls::VecVisitor;

create_wrapper!(Card);

impl Deserialize for Card {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer,
    {
        let values = try!(deserializer.visit_seq(VecVisitor::new()));
        if values.len() != 2 {
            return Err(Error::length_mismatch(values.len()))
        }
        let face = values[0];
        let bull = values[1];
        if face < 1 || face > 104 {
            return Err(Error::invalid_value("face value not in range"))
        }
        if bull < 2 || bull > 7 {
            return Err(Error::invalid_value("bull value not in range"))
        }
        Ok(Card(take5::Card::new(face, bull)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json as json;
    use take5;

    impl Card {
        pub fn deck<F>(f: F) -> Vec<Card> where F: Fn(u8) -> u8 {
            take5::Card::deck(f).into_iter().map(|c| c.into()).collect()
        }
    }

    #[test]
    fn test_decode() {
        let json = "[4,2]";
        let card = json::from_str::<Card>(json).unwrap();
        assert_eq!(4, card.face());
        assert_eq!(2, card.bull());
    }

    #[test]
    fn test_decode_too_many_elements() {
        let json = "[4,2,3]";
        let card = json::from_str::<Card>(json);
        assert!(card.is_err());
    }

    #[test]
    fn test_decode_too_few_elements() {
        let json = "[4]";
        let card = json::from_str::<Card>(json);
        assert!(card.is_err());
    }

    #[test]
    fn test_decode_too_small_numbers() {
        let json = "[0,2]";
        let card = json::from_str::<Card>(json);
        assert!(card.is_err());
        let json = "[1,1]";
        let card = json::from_str::<Card>(json);
        assert!(card.is_err());
    }

    #[test]
    fn test_decode_too_large_numbers() {
        let json = "[105,2]";
        let card = json::from_str::<Card>(json);
        assert!(card.is_err());
        let json = "[1,8]";
        let card = json::from_str::<Card>(json);
        assert!(card.is_err());
    }

    #[test]
    fn test_decode_not_an_array() {
        let json = "{\"face\": 3, \"bull\": 2}";
        let card = json::from_str::<Card>(json);
        assert!(card.is_err());
    }

    #[test]
    fn test_encode() {
        let card: Card = take5::Card::new(4, 2).into();
        let json = json::to_string(&card).unwrap();
        assert_eq!("[4,2]", json);
    }
}
