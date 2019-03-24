use serde_json as json;
use evolution_wire::*;

#[test]
fn serde_trait() {
    assert_serde! {
        Trait,
        "\"carnivore\""
    }
}

#[test]
fn deserialize_trait_err() {
    let json = "\"Carnivore\"";
    assert!(json::from_str::<Trait>(json).is_err());
}

#[test]
fn serde_lot() {
    assert_serde! {
        LOT,
        "[\"carnivore\"]"
    }
}

#[test]
fn deserialize_lot_err() {
    let json = "[\"carnivore\", \"fat-tissue\", \"symbiosis\", \"horns\"]";
    assert!(json::from_str::<LOT>(json).is_err());
}
