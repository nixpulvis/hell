use evolution_wire::*;

#[test]
fn serde() {
    assert_serde! {
        Configuration,
        "[[[[\"id\", 5],
            [\"species\", []],
             [\"bag\", 10]],
            [[\"id\", 5],
             [\"species\", []],
             [\"bag\", 10]],
            [[\"id\", 5],
             [\"species\", []],
             [\"bag\", 10]]],
          1,
          []]"
    };
}
