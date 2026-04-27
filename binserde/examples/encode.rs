use binserde::{Encode, Encoder};

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
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        match self {
            MyEnum::Variant1 => encoder.encode_u32(5)?,
            MyEnum::Variant2(value) => {
                encoder.encode_u32(VAL as u32)?;
                encoder.encode_i32(*value)?
            }
            MyEnum::Variant3(value) => {
                encoder.encode_u32(const_func())?;
                encoder.encode_string(value)?
            }
        }
        Ok(())
    }
}

fn main() {}
