use serde_json as json;
use evolution_wire::*;

#[test]
fn serde() {
    assert_serde! {
        Integer,
        "-1"
    };

    assert_serde! {
        Integer,
        "-1.0",
        "-1"
    };

    assert_serde! {
        Integer,
        "0"
    };

    assert_serde! {
        Integer,
        "-0",
        "0"
    };

    assert_serde! {
        Integer,
        "1"
    };

    assert_serde! {
        Integer,
        "1.0",
        "1"
    };
}

#[test]
fn deserialize_err() {
    let json = "-1.5";
    assert!(json::from_str::<Integer>(&json).is_err());

    let json = "1.5";
    assert!(json::from_str::<Integer>(&json).is_err());
}

#[test]
#[ignore]
fn test_deserialize_large_rational() {
    let json = "1.00000000000000000000000000000000001";
    assert!(json::from_str::<Integer>(&json).is_err());
}
