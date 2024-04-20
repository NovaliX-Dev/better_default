#![allow(dead_code)]

use better_default::BetterDefault;

#[derive(BetterDefault)]
enum Enum2 {
    #[default]
    Variant {
        #[default(1)]
        first: u32,
        second: String,
    },

    Variant2,

    Variant3,
}

fn main() {}
