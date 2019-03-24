use serde_json as json;
use evolution_wire::*;

#[test]
#[should_panic]
fn serde_extinct() {
    assert_serde! {
        Species,
        r###"
[
    ["food", 0],
    ["body", 0],
    ["population", 0],
    ["traits", []]
]
        "###
    };
}

#[test]
fn serde_without_fat() {
    assert_serde! {
        Species,
        "[[\"food\",3],
          [\"body\",4],
          [\"population\",5],
          [\"traits\",[\"carnivore\"]]]"
    };
}

#[test]
fn serde_with_fat() {
    assert_serde! {
        Species,
        "[[\"food\",3],
          [\"body\",4],
          [\"population\",5],
          [\"traits\",[\"fat-tissue\"]],
          [\"fat-food\", 4]]"
    };
}

#[test]
fn serde_with_empty_fat() {
    assert_serde! {
        Species,
        "[[\"food\",3],
          [\"body\",4],
          [\"population\",5],
          [\"traits\",[\"fat-tissue\"]]]"
    };
}

#[test]
fn serde_with_zero_fat_without_fat_tissue() {
    assert_serde! {
        Species,
        "[[\"food\",3],
          [\"body\",4],
          [\"population\",5],
          [\"traits\",[]],
          [\"fat-food\",0]]",
        "[[\"food\",3],
          [\"body\",4],
          [\"population\",5],
          [\"traits\",[]]]"
    };
}

#[test]
fn deserialize_err_food() {
    let json = "[[\"food\",27],
                 [\"body\",1],
                 [\"population\",1],
                 [\"traits\",[]]]";
    assert!(json::from_str::<Species>(json).is_err());
    let json = "[[\"food\",-1],
                 [\"body\",1],
                 [\"population\",1],
                 [\"traits\",[]]]";
    assert!(json::from_str::<Species>(json).is_err());
}

#[test]
fn deserialize_err_body() {
    let json = "[[\"food\",2],
                 [\"body\",143],
                 [\"population\",1],
                 [\"traits\",[]]]";
    assert!(json::from_str::<Species>(json).is_err());
    let json = "[[\"food\",2],
                 [\"body\",-1],
                 [\"population\",1],
                 [\"traits\",[]]]";
    assert!(json::from_str::<Species>(json).is_err());
}

#[test]
fn deserialize_err_population() {
    let json = "[[\"food\",2],
                 [\"body\",2],
                 [\"population\",34],
                 [\"traits\",[]]]";
    assert!(json::from_str::<Species>(json).is_err());
    let json = "[[\"food\",0],
                 [\"body\",1],
                 [\"population\",-1],
                 [\"traits\",[]]]";
    assert!(json::from_str::<Species>(json).is_err());
}

#[test]
fn deserialize_err_traits() {
    let json = "[[\"food\",3],
                 [\"body\",4],
                 [\"population\",5],
                 [\"traits\",[\"carnivore\",
                              \"long-neck\",
                              \"hard-shell\",
                              \"scavenger\"]]]";
    assert!(json::from_str::<Species>(json).is_err());
}

#[test]
fn deserialize_err_fat_tissue() {
    let json = "[[\"food\",3],
                 [\"body\",4],
                 [\"population\",5],
                 [\"traits\",[\"fat-tissue\"]],
                 [\"fat-food\", 8]]";
    assert!(json::from_str::<Species>(json).is_err());
}
