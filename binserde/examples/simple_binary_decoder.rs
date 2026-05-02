use binserde_core::de::Decoder;
use binserde_core::de::{EnumDecoder, MapDecoder, SeqDecoder, StructDecoder, TupleDecoder};

struct SimpleBinaryDecoder {
    input: Vec<u8>,
    pos: usize,
}

#[allow(unused)]
impl SimpleBinaryDecoder {
    fn new(data: Vec<u8>) -> Self {
        SimpleBinaryDecoder {
            input: data,
            pos: 0,
        }
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), String> {
        if self.pos + buf.len() > self.input.len() {
            return Err("Unexpected EOF while reading".to_string());
        }
        buf.copy_from_slice(&self.input[self.pos..self.pos + buf.len()]);
        self.pos += buf.len();
        Ok(())
    }
}

impl Decoder for &mut SimpleBinaryDecoder {
    type Error = String;

    type EnumDecoder = Self;
    type StructDecoder = Self;
    type SeqDecoder = Self;
    type MapDecoder = Self;
    type TupleDecoder = Self;

    fn decode_unit(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn decode_bool(self) -> Result<bool, Self::Error> {
        let mut buf = [0u8; 1];
        self.read_exact(&mut buf)?;
        Ok(u8::from_le_bytes(buf) != 0)
    }

    fn decode_u8(self) -> Result<u8, Self::Error> {
        let mut buf = [0u8; 1];
        self.read_exact(&mut buf)?;
        Ok(u8::from_le_bytes(buf))
    }

    fn decode_u16(self) -> Result<u16, Self::Error> {
        let mut buf = [0u8; 2];
        self.read_exact(&mut buf)?;
        Ok(u16::from_le_bytes(buf))
    }

    fn decode_u32(self) -> Result<u32, Self::Error> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }

    fn decode_u64(self) -> Result<u64, Self::Error> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf)?;
        Ok(u64::from_le_bytes(buf))
    }

    fn decode_u128(self) -> Result<u128, Self::Error> {
        let mut buf = [0u8; 16];
        self.read_exact(&mut buf)?;
        Ok(u128::from_le_bytes(buf))
    }

    fn decode_i8(self) -> Result<i8, Self::Error> {
        let mut buf = [0u8; 1];
        self.read_exact(&mut buf)?;
        Ok(i8::from_le_bytes(buf))
    }

    fn decode_i16(self) -> Result<i16, Self::Error> {
        let mut buf = [0u8; 2];
        self.read_exact(&mut buf)?;
        Ok(i16::from_le_bytes(buf))
    }

    fn decode_i32(self) -> Result<i32, Self::Error> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        Ok(i32::from_le_bytes(buf))
    }

    fn decode_i64(self) -> Result<i64, Self::Error> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf)?;
        Ok(i64::from_le_bytes(buf))
    }

    fn decode_i128(self) -> Result<i128, Self::Error> {
        let mut buf = [0u8; 16];
        self.read_exact(&mut buf)?;
        Ok(i128::from_le_bytes(buf))
    }

    fn decode_f32(self) -> Result<f32, Self::Error> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        Ok(f32::from_le_bytes(buf))
    }

    fn decode_f64(self) -> Result<f64, Self::Error> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf)?;
        Ok(f64::from_le_bytes(buf))
    }

    fn decode_option<T: binserde_core::Decode>(self) -> Result<Option<T>, Self::Error> {
        let tag = self.decode_u8()?;
        if tag == 0 {
            Ok(None)
        } else {
            T::decode(self).map(Some)
        }
    }

    fn decode_bytes(self) -> Result<Vec<u8>, Self::Error> {
        let len = self.decode_u32()? as usize;
        let mut buf = vec![0u8; len];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }

    fn decode_string(self) -> Result<String, Self::Error> {
        let bytes = self.decode_bytes()?;
        String::from_utf8(bytes).map_err(|e| e.to_string())
    }

    fn decode_byte_array<const N: usize>(self) -> Result<[u8; N], Self::Error> {
        let mut buf = [0u8; N];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }

    fn decode_struct(self, _len: usize) -> Result<Self::StructDecoder, Self::Error> {
        Ok(self)
    }

    fn decode_variant(self) -> Result<Self::EnumDecoder, Self::Error> {
        Ok(self)
    }

    fn decode_seq(self) -> Result<Self::SeqDecoder, Self::Error> {
        Ok(self)
    }

    fn decode_map(self) -> Result<Self::MapDecoder, Self::Error> {
        Ok(self)
    }

    fn decode_tuple(self, _len: usize) -> Result<Self::TupleDecoder, Self::Error> {
        Ok(self)
    }
}

impl EnumDecoder for &mut SimpleBinaryDecoder {
    type Error = String;

    fn decode_discriminant_u8(&mut self) -> Result<u8, Self::Error> {
        self.decode_u8()
    }
    fn decode_discriminant_u16(&mut self) -> Result<u16, Self::Error> {
        self.decode_u16()
    }
    fn decode_discriminant_u32(&mut self) -> Result<u32, Self::Error> {
        self.decode_u32()
    }
    fn decode_discriminant_u64(&mut self) -> Result<u64, Self::Error> {
        self.decode_u64()
    }
    fn decode_discriminant_u128(&mut self) -> Result<u128, Self::Error> {
        self.decode_u128()
    }
    fn decode_discriminant_usize(&mut self) -> Result<usize, Self::Error> {
        let v = self.decode_u64()?;
        Ok(v as usize)
    }
    fn decode_discriminant_i8(&mut self) -> Result<i8, Self::Error> {
        self.decode_i8()
    }
    fn decode_discriminant_i16(&mut self) -> Result<i16, Self::Error> {
        self.decode_i16()
    }
    fn decode_discriminant_i32(&mut self) -> Result<i32, Self::Error> {
        self.decode_i32()
    }
    fn decode_discriminant_i64(&mut self) -> Result<i64, Self::Error> {
        self.decode_i64()
    }
    fn decode_discriminant_i128(&mut self) -> Result<i128, Self::Error> {
        self.decode_i128()
    }
    fn decode_discriminant_isize(&mut self) -> Result<isize, Self::Error> {
        let v = self.decode_i64()?;
        Ok(v as isize)
    }

    fn decode_field<T: binserde_core::Decode>(&mut self) -> Result<T, Self::Error> {
        T::decode(&mut **self)
    }
    fn end(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl StructDecoder for &mut SimpleBinaryDecoder {
    type Error = String;
    fn decode_field<T: binserde_core::Decode>(&mut self) -> Result<T, Self::Error> {
        T::decode(&mut **self)
    }
    fn end(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl SeqDecoder for &mut SimpleBinaryDecoder {
    type Error = String;
    fn decode_len(&mut self) -> Result<usize, Self::Error> {
        let v = self.decode_u32()?;
        Ok(v as usize)
    }
    fn decode_element<T: binserde_core::Decode>(&mut self) -> Result<T, Self::Error> {
        T::decode(&mut **self)
    }
    fn end(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl MapDecoder for &mut SimpleBinaryDecoder {
    type Error = String;
    fn decode_key<T: binserde_core::Decode>(&mut self) -> Result<Option<T>, Self::Error> {
        todo!("MapDecoder::decode_key")
    }
    fn decode_value<T: binserde_core::Decode>(&mut self) -> Result<T, Self::Error> {
        T::decode(&mut **self)
    }
    fn end(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl TupleDecoder for &mut SimpleBinaryDecoder {
    type Error = String;
    fn decode_element<T: binserde_core::Decode>(&mut self) -> Result<T, Self::Error> {
        T::decode(&mut **self)
    }
    fn end(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

fn main() {
    println!(
        "SimpleBinaryDecoder example — run tests with: cargo test --example simple_binary_decoder"
    );
}

#[cfg(test)]
mod tests {
    use binserde_core::de::Decode;

    use super::*;

    #[test]
    fn test_decode_struct() {
        struct TestStruct {
            v_i32: i32,
            b: bool,
            v_inner_struct: InnerStruct,
        }

        struct InnerStruct {
            v_u32: u32,
            v_array: [u8; 4],
        }

        impl Decode for InnerStruct {
            fn decode<D: Decoder>(dec: D) -> Result<Self, D::Error> {
                let mut s = dec.decode_struct(2)?;
                let v_u32 = StructDecoder::decode_field::<u32>(&mut s)?;
                let v_array = StructDecoder::decode_field::<[u8; 4]>(&mut s)?;
                StructDecoder::end(&mut s)?;
                Ok(InnerStruct { v_u32, v_array })
            }
        }

        impl Decode for TestStruct {
            fn decode<D: Decoder>(dec: D) -> Result<Self, D::Error> {
                let mut s = dec.decode_struct(3)?;
                let v_i32 = StructDecoder::decode_field::<i32>(&mut s)?;
                let b = StructDecoder::decode_field::<bool>(&mut s)?;
                let v_inner_struct = StructDecoder::decode_field::<InnerStruct>(&mut s)?;
                StructDecoder::end(&mut s)?;
                Ok(TestStruct {
                    v_i32,
                    b,
                    v_inner_struct,
                })
            }
        }

        let buf = vec![
            0xfe, 0xff, 0xff, 0xff, // i32, value: -2
            1,    // bool, value: true,
            0, 1, 0, 0, // u32, value: 256
            0, 1, 2, 3, // array, value: [0, 1, 2, 3]
        ];

        let mut dec = SimpleBinaryDecoder::new(buf);
        let test_struct = TestStruct::decode(&mut dec).unwrap();
        assert_eq!(test_struct.v_i32, -2);
        assert_eq!(test_struct.b, true);
        assert_eq!(test_struct.v_inner_struct.v_u32, 256);
        assert_eq!(test_struct.v_inner_struct.v_array, [0, 1, 2, 3]);
    }

    #[test]
    fn test_decode_strucr_new_type() {
        struct SNewType(String, u32);
        impl Decode for SNewType {
            fn decode<D: Decoder>(decoder: D) -> Result<Self, D::Error> {
                let mut s = decoder.decode_struct(2)?;
                let field_string = s.decode_field::<String>()?;
                let field_u32 = s.decode_field::<u32>()?;
                s.end()?;
                Ok(SNewType(field_string, field_u32))
            }
        }
    }
}
