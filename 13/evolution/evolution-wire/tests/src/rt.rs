use serde_json as json;
use evolution_wire::RT;

#[test]
fn serde() {
    assert_serde! {
        RT,
        "[1, 2, 3]"
    };
}

#[test]
fn deserialize_err() {
    let json = "[1, 2]";
    assert!(json::from_str::<RT>(&json).is_err());

    let json = "[1, 2, 3, 4]";
    assert!(json::from_str::<RT>(&json).is_err());
}
