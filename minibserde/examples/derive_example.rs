use minibserde::{Decode, Encode};

fn main() {}

#[allow(unused)]
#[derive(Encode, Decode)]
#[repr(u32)]
enum MyEnum {
    Variant1 = 5,
    Variant2(i32) = 16,
    Variant3(String, u32) = 41,
}

const VAL: u8 = 42;
const fn foo() -> u8 {
    64
}

#[allow(unused)]
#[derive(Encode, Decode)]
#[repr(u8)]
enum MyColor {
    Red = 1,
    Green = VAL,
    Blue = foo(),
    #[minibserde(catch_all)]
    Purple(u8),
}
