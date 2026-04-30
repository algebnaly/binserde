use binserde::{Discriminant, Encode, Encoder, EnumEncoder};

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
            MyEnum::Variant1 => {
                let mut v = encoder.encode_variant()?;
                EnumEncoder::encode_variant(&mut v, Discriminant::U32(5), "Variant1", &())?;
                v.end()
            }
            MyEnum::Variant2(value) => {
                let mut v = encoder.encode_variant()?;
                EnumEncoder::encode_variant(&mut v, Discriminant::U32(16), "Variant2", value)?;
                v.end()
            }
            MyEnum::Variant3(value) => {
                let mut v = encoder.encode_variant()?;
                EnumEncoder::encode_variant(&mut v, Discriminant::U32(41), "Variant3", value)?;
                v.end()
            }
        }
    }
}
