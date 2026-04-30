use binserde::Encode;

fn main() {}

#[allow(unused)]
#[derive(Encode)]
#[repr(u32)]
enum MyEnum {
    Variant1 = 5,
    Variant2(i32) = 16,
    Variant3(String, u32) = 41,
}
