extern crate serde_json;
extern crate evolution_wire;
extern crate itertools;

macro_rules! assert_serde {
    {$ty:ty, $json:expr} => {{
        use itertools::Itertools;
        use serde_json as json;
        let deserialized = json::from_str::<$ty>($json).unwrap();
        let serialized = json::to_string::<$ty>(&deserialized).unwrap();
        assert_eq!($json.split_whitespace().join(""),
                   serialized.split_whitespace().join(""));
    }};
    {$ty:ty, $json:expr, $expected:expr} => {{
        use itertools::Itertools;
        use serde_json as json;
        let deserialized = json::from_str::<$ty>($json).unwrap();
        let serialized = json::to_string::<$ty>(&deserialized).unwrap();
        assert_eq!($expected.split_whitespace().join(""),
                   serialized.split_whitespace().join(""));
    }};
}

mod src;
