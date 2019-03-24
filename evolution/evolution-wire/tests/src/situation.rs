use serde_json as json;
use evolution_wire::*;

#[test]
fn deserialize_attacker_defender() {
    let json = "[[[\"food\",0],
                  [\"body\",1],
                  [\"population\",1],
                  [\"traits\",[]]],
                 [[\"food\",0],
                  [\"body\",1],
                  [\"population\",1],
                  [\"traits\",[]]],
                 false,
                 false]";
    let situation = json::from_str::<Situation>(&json).unwrap();
    assert_eq!(0, *situation.attacker.food);
    assert_eq!(0, situation.defender.traits.len());
}

#[test]
fn deserialize_left_right() {
    let json = "[[[\"food\",0],
                  [\"body\",1],
                  [\"population\",1],
                  [\"traits\",[]]],
                 [[\"food\",0],
                  [\"body\",1],
                  [\"population\",1],
                  [\"traits\",[]]],
                 [[\"food\",1],
                  [\"body\",1],
                  [\"population\",2],
                  [\"traits\",[]]],
                 false]";
    let situation = json::from_str::<Situation>(&json).unwrap();
    assert_eq!(2, *situation.left.unwrap().population);
    assert_eq!(None, situation.right);
}
