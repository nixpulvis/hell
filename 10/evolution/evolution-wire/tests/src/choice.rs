use serde_json as json;
use evolution_wire::Choice;

#[test]
fn serde_with_species() {
    assert_serde! {
        Choice,
        r###"
[
    [
        ["id", 5],
        ["species", [
            [
                ["food", 1],
                ["body", 2],
                ["population", 1],
                ["traits", ["carnivore"]]
            ]
        ]],
        ["bag", 10]
    ],
    [
        [
            [
                ["food", 1],
                ["body", 5],
                ["population", 3],
                ["traits", ["fat-tissue"]],
                ["fat-food", 4]
            ],
            [
                ["food", 3],
                ["body", 4],
                ["population", 5],
                ["traits", []]
            ]
        ]
    ],
    [
        [
            [
                ["food", 2],
                ["body", 4],
                ["population", 2],
                ["traits", ["carnivore"]]
            ]
        ]
    ]
]
        "###
    };
}

#[test]
fn serde_without_species() {
    assert_serde! {
        Choice,
        r###"
[
    [
        ["id", 5],
        ["species", []],
        ["bag", 10]
    ],
    [],
    []
]
        "###
    };
}

#[test]
fn deserialize_err() {
    let json = "[]";
    assert!(json::from_str::<Choice>(&json).is_err());

    // "cards" is no a vaild pair here.
    let json = r###"
[
    [
        ["id", 5],
        ["species", []],
        ["bag", 10],
        ["cards"]
    ],
    [],
    []
]
    "###;
    assert!(json::from_str::<Choice>(&json).is_err());
}
