#![allow(dead_code)]

use better_default::BetterDefault;

#[derive(BetterDefault)]
enum Enum<T> {
    #[default]
    Variant(T)
}

#[derive(BetterDefault)]
enum Enum2<T> {
    #[default]
    Variant {
        field: T
    }
}

#[derive(BetterDefault)]
struct Struct<T> {
    field: T
}

fn main() {}