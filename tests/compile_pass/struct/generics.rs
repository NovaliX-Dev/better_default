use better_default::BetterDefault;

#[derive(BetterDefault)]
struct Struct<T: Default> {
    field: T,
    field2: String
}

fn main() {
    
}
