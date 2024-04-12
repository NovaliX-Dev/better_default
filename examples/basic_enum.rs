#![allow(dead_code)]

use better_default::BetterDefault;

#[derive(BetterDefault)]
enum Enum {
    #[default]
    Variant(u32, String),

    Variant2,

    Variant3,
}

#[derive(BetterDefault)]
enum Enum2 {
    #[default(first: 0, second: "aaaaaa".to_string())]
    Variant {
        first: u32,
        second: String,
    },

    Variant2,

    Variant3,
}

fn main() {}
