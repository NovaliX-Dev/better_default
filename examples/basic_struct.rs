use better_default::BetterDefault;

#[derive(BetterDefault)]
struct Struct {
    #[default(10)]
    field1: u32,

    #[default("aaaaaa".to_string())]
    field2: String,
}

#[derive(BetterDefault)]
struct Struct2(u32, String);

#[derive(BetterDefault)]
#[default(1: "aaaa")]
struct Struct2WithLifetime<'l>(u32, &'l str);

fn main() {}
