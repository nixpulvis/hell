use serde_json as json;
use evolution_wire::*;

// TODO: Deserialize.

#[test]
fn serde_false() {
    assert_serde! {
        FeedChoice,
        "false"
    };
}

#[test]
fn serde_vegetarian() {
    assert_serde! {
        FeedChoice,
        "0"
    }
}

#[test]
fn serde_fat_tissue() {
    assert_serde! {
        FeedChoice,
        "[2,1]"
    };
}

#[test]
#[should_panic]
fn serde_fat_tissue_empty() {
    assert_serde! {
        FeedChoice,
        "[2,0]"
    };
}

#[test]
fn serde_carnivore() {
    assert_serde! {
        FeedChoice,
        "[5,3,2]"
    };
}

#[test]
fn serde_abstain() {
    let choice = FeedChoice::Abstain;
    let json = json::to_string(&choice).unwrap();
    let expected = "false";
    assert_eq!(expected, json);
}

#[test]
fn serde_feed() {
    let choice = FeedChoice::Feed(Natural(0));
    let json = json::to_string(&choice).unwrap();
    let expected = "0";
    assert_eq!(expected, json);
}

#[test]
fn serde_store() {
    let choice = FeedChoice::Store(Natural(2), NatPlus::new(1).unwrap());
    let json = json::to_string(&choice).unwrap();
    let expected = "[2,1]";
    assert_eq!(expected, json);
}

#[test]
fn serde_attack() {
    let choice = FeedChoice::Attack(Natural(5), Natural(3), Natural(2));
    let json = json::to_string(&choice).unwrap();
    let expected = "[5,3,2]";
    assert_eq!(expected, json);
}
