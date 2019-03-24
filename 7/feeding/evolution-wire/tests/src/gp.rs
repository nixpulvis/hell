use evolution_wire::GP;

#[test]
fn serde_good() {
    assert_serde! {
        GP,
r###"
["population", 3, 4]
"###
    };
}

#[test]
#[should_panic]
fn serde_bad_key() {
    assert_serde! {
        GP,
r###"
["población", 3, 4]
"###
    };
}

#[test]
#[should_panic]
fn serde_bad_len() {
    assert_serde! {
        GP,
r###"
["population", 3]
"###
    };
}
