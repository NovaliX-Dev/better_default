use better_default::BetterDefault;

#[derive(BetterDefault, Debug, Eq, PartialEq)]
enum Enum2 {
    #[default(first: 1)]
    Variant {
        first: u32,
        second: String,
    },

    Variant2,

    Variant3,
}

fn main() {
    assert_eq!(Enum2::Variant { first: 1, second: String::new() }, Enum2::default())
}
