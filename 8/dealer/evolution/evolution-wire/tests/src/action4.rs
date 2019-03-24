use serde_json as json;
use evolution_wire::Action4;

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
    ["population", 1, 2]
  ],
  [
    ["body", 1, 2]
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
    ["Population", 1, 2]
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
