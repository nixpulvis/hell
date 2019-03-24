use serde::{Deserialize, Deserializer};
use serde::de::Error;
use serde::de::impls::VecVisitor;
use super::StartingHand;

impl Deserialize for StartingHand {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer,
    {
        let cards = try!(deserializer.visit_seq(VecVisitor::new()));
        let len = cards.len();
        match StartingHand::new(cards) {
            Ok(hand) => Ok(hand),
            Err(_) => Err(Error::length_mismatch(len)),
        }
    }
}
