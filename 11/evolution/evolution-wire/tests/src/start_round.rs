use evolution_wire::StartRound;

#[test]
fn serde_good() {
    assert_serde! {
        StartRound,
r###"
[
  [
    [
      [
        ["id", 12],
        ["species", []],
        ["bag", 0],
        ["cards", [
          [3, "scavenger"]
        ]]
      ]
    ],
    25, [
      [7, "carnivore"],
      [3, "fat-tissue"],
      [3, "burrowing"]
    ]
  ],
  [
    [0, [],
      [],
      [],
      []
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
        StartRound,
r###"
[]
"###
    };
}
