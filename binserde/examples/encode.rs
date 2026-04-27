use binserde::{Encode, Encoder, EnumEncoder};

const fn const_func() -> u32 {
    41
}

const VAL: u8 = 16;

#[repr(u32)]
enum MyEnum {
    Variant1 = 5,
    Variant2(i32) = VAL as u32,
    Variant3(String) = const_func(),
}

impl Encode for MyEnum {
    fn encode<E: Encoder>(&self, encoder: E) -> Result<(), E::Error> {
        match self {
            MyEnum::Variant1 => {
                let mut v = encoder.encode_variant()?;
                v.encode_variant(1, "Variant1", &())?;
                v.end()
            }
            MyEnum::Variant2(value) => {
                let mut v = encoder.encode_variant()?;
                v.encode_variant(2, "Variant2", value)?;
                v.end()
            }
            MyEnum::Variant3(value) => {
                let mut v = encoder.encode_variant()?;
                v.encode_variant(3, "Variant3", value)?;
                v.end()
            }
        }
    }
}

fn main() {}
