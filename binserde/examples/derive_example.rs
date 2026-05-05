use binserde::{Decode, Encode};

fn main() {}

#[allow(unused)]
#[derive(Encode, Decode)]
#[repr(u32)]
enum MyEnum {
    Variant1 = 5,
    Variant2(i32) = 16,
    Variant3(String, u32) = 41,
}

#[allow(unused)]
#[derive(Encode, Decode)]
#[repr(u8)]
enum MyColor {
    Red = 1,
    Green = 4,
    Blue = 9,
    #[binserde(catch_all)]
    Purple(u8),
}
