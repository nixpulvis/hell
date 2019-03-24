use serde_json as json;
use evolution_wire::{Natural, BT};
use evolution_wire::remote::{Action4, GP};

#[test]
fn serde_empty() {
    assert_serde! {
        Action4,
r###"
[
  0,
  [],
  [],
  [],
  []
]
"###
    };
}

#[test]
fn serde_non_empty() {
    assert_serde! {
        Action4,
r###"
[
  0,
  [
    [1, 2]
  ],
  [
    [1, 2]
  ],
  [
    [1],
    [1, 2],
    [1, 2, 3]
  ],
  [
    [1, 2, 3],
    [1, 2, 3]
  ]
]
"###
    };
}

#[test]
fn deserialize_err() {
    let json = "[]";
    assert!(json::from_str::<Action4>(&json).is_err());

    let json =
r###"
[
  [], 0, []
]
"###;
    assert!(json::from_str::<Action4>(&json).is_err());

    let json =
r###"
[
  1,
  [1, 2, 3],
  [1, 2, 3]
]
"###;
    assert!(json::from_str::<Action4>(&json).is_err());

    let json =
r###"
[
  0,
  [
    ["population", 1, 2]
  ],
  [
    ["body", 1]
  ],
  [
    [1],
    [],
  ],
  [
    [1, 2]
  ]
]
"###;
    assert!(json::from_str::<Action4>(&json).is_err());
}

#[test]
fn sanity_check() {
    let expected = Action4(Natural(0), vec![
        GP {
            board_index: Natural(0),
            card_index: Natural(3)
        }
    ], vec![], vec![
        BT(vec![
            Natural(1),
            Natural(2)
        ])
    ], vec![]);

    let string = json::to_string(&expected).expect("failed to make string");
    let actual = json::from_str::<Action4>(&string).expect("failed to make Action4");
    assert_eq!(expected.0, actual.0);
}
