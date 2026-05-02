use binserde::{Discriminant, Encode, Encoder};

fn main() {
    // Example showing manual enum encoding with the Discriminant enum
    let _ = MyEnum::Variant1;
}

#[allow(unused)]
#[repr(u32)]
enum MyEnum {
    Variant1 = 5,
    Variant2(i32) = 16,
    Variant3(String) = 41,
}

impl Encode for MyEnum {
    fn encode<E: Encoder>(&self, encoder: E) -> Result<(), E::Error> {
        match self {
            MyEnum::Variant1 => encoder.encode_variant(Discriminant::U32(5), &()),
            MyEnum::Variant2(value) => encoder.encode_variant(Discriminant::U32(16), value),
            MyEnum::Variant3(value) => encoder.encode_variant(Discriminant::U32(41), value),
        }
    }
}
