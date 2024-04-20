#![allow(dead_code)]

use better_default::BetterDefault;

#[derive(BetterDefault)]
enum Enum<T: Default> {
    #[default]
    Variant(T)
}

#[derive(BetterDefault)]
enum Enum2<T: Default> {
    #[default]
    Variant {
        field: T
    }
}

fn main() {}