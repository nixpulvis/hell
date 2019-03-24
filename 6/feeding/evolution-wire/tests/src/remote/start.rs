use evolution_wire::remote::Start;

#[test]
fn serde() {
    assert_serde! {
        Start,
        r###"
[
    5,
    2, [],
    [
        [5, "carnivore"],
        [2, "burrowing"],
        [-3, "ambush"]
    ]
]
        "###
    };
}

#[test]
#[should_panic]
fn serde_bad() {
    assert_serde! {
        Start,
        r###"
[
    0.2,
    -1, [],
    [
        [7, "ambush"],
        [-3, "carnivore"],
        [2, "fat-tissue"]
    ]
]
        "###
    };
}
