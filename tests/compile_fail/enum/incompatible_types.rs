#![allow(dead_code)]

use better_default::BetterDefault;

#[derive(BetterDefault)]
enum Enum {
    #[default(0: "aaa")]
    Variant(u32, String),

    Variant2,

    Variant3,
}

#[derive(BetterDefault)]
enum Enum2 {
    #[default]
    Variant{
        #[default("aaaa")]
        field1: u32,

        field2: String
    },

    Variant2,

    Variant3,
}

fn main() {}
