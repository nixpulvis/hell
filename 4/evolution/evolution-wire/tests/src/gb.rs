use evolution_wire::GB;

#[test]
fn serde_good() {
    assert_serde! {
        GB,
r###"
["body", 3, 4]
"###
    };
}

#[test]
#[should_panic]
fn serde_bad_key() {
    assert_serde! {
        GB,
r###"
["cuerpo", 3, 4]
"###
    };
}

#[test]
#[should_panic]
fn serde_bad_len() {
    assert_serde! {
        GB,
r###"
["body", 3]
"###
    };
}
