use serde_json as json;
use evolution_wire::BT;

#[test]
fn serde_variant_1() {
    assert_serde! {
        BT,
        "[1]"
    };
}

#[test]
fn serde_variant_2() {
    assert_serde! {
        BT,
        "[1, 2]"
    }
}

#[test]
fn serde_variant_3() {
    assert_serde! {
        BT,
        "[1, 2, 3]"
    }
}

#[test]
fn serde_variant_4() {
    assert_serde! {
        BT,
        "[1, 2, 3, 4]"
    }
}

#[test]
fn deserialize_err() {
    let json = "[]";
    assert!(json::from_str::<BT>(&json).is_err());

    let json = "[1, 2, 3, 4, 5]";
    assert!(json::from_str::<BT>(&json).is_err());
}
