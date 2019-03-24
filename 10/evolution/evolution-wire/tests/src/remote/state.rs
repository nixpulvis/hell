use evolution_wire::remote::State;

#[test]
fn serde() {
    assert_serde! {
        State,
        r###"
[
    1,
    [
        [
            ["food", 0],
            ["body", 0],
            ["population", 1],
            ["traits", []]
        ]
    ],
    [],
    9,
    [
        [
            [
                ["food", 0],
                ["body", 0],
                ["population", 1],
                ["traits", []]
            ]
        ],
        [
            [
                ["food", 0],
                ["body", 0],
                ["population", 1],
                ["traits", []]
            ]
        ]
    ]
]
        "###
    };
}

#[test]
#[should_panic]
fn serde_bad() {
    assert_serde! {
        State,
        r###"
[
    0.4,
    [
        [
            ["food", 0],
            ["body", 0],
            ["population", 1],
            ["traits"]
        ]
    ],
    [],
    9,
    [
        [
            [
                ["population", 1],
                ["food", 0],
                ["body", 0],
                ["traits"]
            ]
        ],
        [
            [
                ["food", 0],
                ["body", 0],
                ["population", 1],
                ["traits"]
            ]
        ]
    ]
]
        "###
    };
}
