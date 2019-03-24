use serde_json as json;
use evolution_wire::*;

#[test]
fn carnivore() {
    assert_serde! {
        SpeciesCard,
        "[-8, \"carnivore\"]"
    };
}

#[test]
fn deserialize_err() {
    let json = "[\"carnivore\", 4]";
    assert!(json::from_str::<SpeciesCard>(&json).is_err());

    let json = "[\"carnivore\"]";
    assert!(json::from_str::<SpeciesCard>(&json).is_err());

    let json = "[4]";
    assert!(json::from_str::<SpeciesCard>(&json).is_err());

    let json = "[4, \"carnivore\", \"is not a cannibal\"]";
    assert!(json::from_str::<SpeciesCard>(&json).is_err());
}
