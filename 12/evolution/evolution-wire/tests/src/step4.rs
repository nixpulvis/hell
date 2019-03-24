use serde_json as json;
use evolution_wire::Step4;

#[test]
fn serde_step4_empty() {
    assert_serde! {
        Step4,
        "[]"
    };
}

#[test]
fn serde_step4_non_empty() {
    assert_serde! {
        Step4,
r###"
[
  [
    0,
    [],
    [],
    [],
    []
  ],
  [
    0,
    [["population", 1, 2]],
    [],
    [],
    []
  ]
]
"###
    };
}

#[test]
fn deserialize_step4_err() {
    let json = "[1]";
    assert!(json::from_str::<Step4>(&json).is_err());
}
