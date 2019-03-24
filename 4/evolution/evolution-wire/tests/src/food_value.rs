use serde_json as json;
use evolution_wire::*;

#[test]
fn serde() {
    assert_serde! {
        FoodValue,
        "-8"
    };

    assert_serde! {
        FoodValue,
        "-8.0",
        "-8"
    };

    assert_serde! {
        FoodValue,
        "-0",
        "0"
    };

    assert_serde! {
        FoodValue,
        "0"
    };

    assert_serde! {
        FoodValue,
        "8"
    };

    assert_serde! {
        FoodValue,
        "8.0",
        "8"
    };
}

#[test]
fn deserialize_err() {
    let json = "-9";
    assert!(json::from_str::<FoodValue>(&json).is_err());

    let json = "9";
    assert!(json::from_str::<FoodValue>(&json).is_err());

    let json = "6.66667";
    assert!(json::from_str::<FoodValue>(&json).is_err());
}
