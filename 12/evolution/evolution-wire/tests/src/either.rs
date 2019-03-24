use evolution_wire::{Nat, NatPlus, Either, SpeciesCard, Trait};
use serde_json as json;

#[test]
fn single_variant_match() {
    let wire = r###"
[-2, "burrowing"]
               "###;
    let json = json::from_str(wire).unwrap();
    let either = Either::<SpeciesCard, Nat>::from_value(json).unwrap();
    assert!(either.is_left());
    assert!(!either.is_right());
}

#[test]
fn multi_variant_match() {
    let wire = "2";
    let json = json::from_str(wire).unwrap();
    let either = Either::<Nat, NatPlus>::from_value(json);
    assert!(either.is_err());
}

#[test]
fn no_variant_match() {
    let wire = "[]";
    let json = json::from_str(wire).unwrap();
    let either = Either::<SpeciesCard, NatPlus>::from_value(json);
    assert!(either.is_err());
}

#[test]
fn deserialize_preserves_values() {
    let wire = r###"
[-2, "burrowing"]
               "###;
    let json = json::from_str(wire).unwrap();
    let either = Either::<SpeciesCard, Nat>::from_value(json);
    match either {
        Ok(Either::Left(SpeciesCard(food_value, trait_type))) => {
            assert_eq!(-2, *food_value);
            assert_eq!(Trait::Burrowing, trait_type);
        },
        _ => panic!("either was error or right variant"),
    }
}
