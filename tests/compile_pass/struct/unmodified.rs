use better_default::BetterDefault;

#[derive(BetterDefault, Debug, Eq, PartialEq)]
struct Struct {
    field: u32,
    field2: String
}

fn main() {
    assert_eq!(Struct { field: 0, field2: String::new() }, Struct::default())
}
