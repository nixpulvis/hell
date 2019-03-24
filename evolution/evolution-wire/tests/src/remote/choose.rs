use evolution_wire::remote::Choose;

#[test]
fn serde() {
    assert_serde! {
        Choose,
        r###"
[
    [
        [
            [
                ["food", 0],
                ["body", 0],
                ["population", 1],
                ["traits", []]
            ]
        ]
    ],
    [
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
        Choose,
        r###"
[
    [
        [
            ["food", 0],
            ["body", 0],
            ["traits", []],
            ["population", 1]
        ]
    ],
    [
        ["food", 0],
        ["body", 0],
        ["population", 1],
        ["traits", []]
    ],
]
        "###
    };
}
