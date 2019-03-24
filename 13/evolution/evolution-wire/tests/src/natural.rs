use serde_json as json;
use evolution_wire::*;

#[test]
fn serde_natural() {
    assert_serde! {
        Natural,
        "1"
    }

    assert_serde! {
        Natural,
        "1.0",
        "1"
    }

    assert_serde! {
        Natural,
        "-0",
        "0"
    }
}

#[test]
fn deserialize_err_natural() {
    let json = "-1";
    assert!(json::from_str::<Natural>(&json).is_err());
    let json = "1.5";
    assert!(json::from_str::<Natural>(&json).is_err());
}

#[test]
fn serde_natural_plus() {
    assert_serde! {
        NaturalPlus,
        "1"
    }
}

#[test]
fn deserialize_err_natural_plus() {
    let json = "0";
    assert!(json::from_str::<NaturalPlus>(&json).is_err());
}

#[test]
fn serde_nat() {
    assert_serde! {
        Nat,
        "0"
    }

    assert_serde! {
        Nat,
        "1"
    }
}

#[test]
fn deserialize_err_nat() {
    let json = "8";
    assert!(json::from_str::<Nat>(&json).is_err());
}

#[test]
fn serde_nat_plus() {
    assert_serde! {
        NatPlus,
        "1"
    }
}

#[test]
fn deserialize_err_nat_plus() {
    let json = "0";
    assert!(json::from_str::<NatPlus>(&json).is_err());
    let json = "8";
    assert!(json::from_str::<NatPlus>(&json).is_err());
}
