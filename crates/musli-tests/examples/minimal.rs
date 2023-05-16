use musli::{Decode, Encode, Mode};

enum Packed {}
impl Mode for Packed {}

#[derive(Encode, Decode)]
#[musli(default_field_name = "name")]
#[musli(mode = Packed, encode_only, packed, default_field_name = "index")]
struct Person<'a> {
    name: &'a str,
    age: u32,
}

fn main() {}
