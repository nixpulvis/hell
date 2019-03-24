use serde_json as json;
use evolution_wire::*;

#[test]
fn serde_without_cards() {
    assert_serde! {
        Player,
        "[[\"id\",2],
          [\"species\",[[[\"food\",0],
                         [\"body\",1],
                         [\"population\",1],
                         [\"traits\",[\"carnivore\"]]]]],
          [\"bag\",10]]"
    }
}

#[test]
fn serde_with_cards() {
    assert_serde! {
        Player,
        "[[\"id\", 5],
          [\"species\", []],
          [\"bag\", 10],
          [\"cards\", [[8, \"carnivore\"],
                       [3, \"fat-tissue\"]]]]"
    };
}

#[test]
fn serde_with_empty_cards() {
    assert_serde! {
        Player,
        "[[\"id\", 5],
          [\"species\", []],
          [\"bag\", 10],
          [\"cards\", []]]",
        "[[\"id\", 5],
          [\"species\", []],
          [\"bag\", 10]]"
    };
}

#[test]
fn deserialize_err_id() {
    let json = "[[\"id\",0],
                 [\"species\",[]],
                 [\"bag\",0]]";
    assert!(json::from_str::<Player>(json).is_err());
    let json = "[[\"id\",-10],
                 [\"species\",[]],
                 [\"bag\",0]]";
    assert!(json::from_str::<Player>(json).is_err());
}

#[test]
fn deserialize_err_order() {
    let json = "[[\"bag\",0],
                 [\"species\",[]],
                 [\"id\",-10]]";
    assert!(json::from_str::<Player>(json).is_err());
}
