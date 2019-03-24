use serde_json as json;
use evolution_wire::*;

#[test]
fn serde() {
    assert_serde! {
        Feeding,
        r###"
[
    [
        ["id", 1],
        ["species", []],
        ["bag", 0]
    ],
    3,
    [
        [
            ["id", 2],
            ["species", []],
            ["bag", 0]
        ],
        [
            ["id", 3],
            ["species", []],
            ["bag", 0]
        ]
    ]
]
        "###
    }
}

#[test]
fn test_deserialize_empty() {
    let json = "[[[\"id\",10],
                  [\"species\",[]],
                  [\"bag\",0]],
                 7,
                 []]";
    let feeding = json::from_str::<Feeding>(&json).unwrap();
    assert_eq!(10, *feeding.current_player.id);
    assert_eq!(0, feeding.opponents.len());
    assert_eq!(0, feeding.opponents.len());
}

#[test]
fn test_deserialize_opponent() {
    let json = "[[[\"id\",10],
                  [\"species\",[]],
                  [\"bag\",0]],
                 7,
                 [[[\"id\",2],
                   [\"species\",[]],
                   [\"bag\",0]]]]";
    let feeding = json::from_str::<Feeding>(&json).unwrap();
    assert_eq!(2, *feeding.opponents[0].id);
}

#[test]
fn test_deserialize_invalid_watering_hole() {
    let json = "[[[\"id\",10],
                  [\"species\",[]],
                  [\"bag\",0]],
                 0,
                 []]";
    assert!(json::from_str::<Feeding>(&json).is_err());
}
