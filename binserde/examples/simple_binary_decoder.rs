use binserde_core::de::Decoder;

struct SimpleBinaryDecoder {
    input: Vec<u8>,
    pos: usize,
}

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

impl Decoder for SimpleBinaryDecoder {
    type Error = String;

    fn decode_unit(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn decode_bool(&mut self) -> Result<bool, Self::Error> {
        let mut buf = [0u8; 1];
        self.read_exact(&mut buf)?;
        Ok(u8::from_le_bytes(buf) != 0)
    }

    fn decode_u8(&mut self) -> Result<u8, Self::Error> {
        let mut buf = [0u8; 1];
        self.read_exact(&mut buf)?;
        Ok(u8::from_le_bytes(buf))
    }

    fn decode_u16(&mut self) -> Result<u16, Self::Error> {
        let mut buf = [0u8; 2];
        self.read_exact(&mut buf)?;
        Ok(u16::from_le_bytes(buf))
    }

    fn decode_u32(&mut self) -> Result<u32, Self::Error> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }

    fn decode_u64(&mut self) -> Result<u64, Self::Error> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf)?;
        Ok(u64::from_le_bytes(buf))
    }

    fn decode_u128(&mut self) -> Result<u128, Self::Error> {
        let mut buf = [0u8; 16];
        self.read_exact(&mut buf)?;
        Ok(u128::from_le_bytes(buf))
    }

    fn decode_i8(&mut self) -> Result<i8, Self::Error> {
        let mut buf = [0u8; 1];
        self.read_exact(&mut buf)?;
        Ok(i8::from_le_bytes(buf))
    }

    fn decode_i16(&mut self) -> Result<i16, Self::Error> {
        let mut buf = [0u8; 2];
        self.read_exact(&mut buf)?;
        Ok(i16::from_le_bytes(buf))
    }

    fn decode_i32(&mut self) -> Result<i32, Self::Error> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        Ok(i32::from_le_bytes(buf))
    }

    fn decode_i64(&mut self) -> Result<i64, Self::Error> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf)?;
        Ok(i64::from_le_bytes(buf))
    }

    fn decode_i128(&mut self) -> Result<i128, Self::Error> {
        let mut buf = [0u8; 16];
        self.read_exact(&mut buf)?;
        Ok(i128::from_le_bytes(buf))
    }

    fn decode_f32(&mut self) -> Result<f32, Self::Error> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        Ok(f32::from_le_bytes(buf))
    }

    fn decode_f64(&mut self) -> Result<f64, Self::Error> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf)?;
        Ok(f64::from_le_bytes(buf))
    }

    fn decode_bytes(&mut self) -> Result<Vec<u8>, Self::Error> {
        // Length is stored as u32 little‑endian.
        let len = self.decode_u32()? as usize;
        let mut buf = vec![0u8; len];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }

    fn decode_byte_array(&mut self, len: usize) -> Result<Vec<u8>, Self::Error> {
        let mut buf = vec![0u8; len];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }
}

fn main() {
    let v = vec![1, 2, 3, 4];
    let mut dec = SimpleBinaryDecoder::new(v.clone());

    let dec_v = (&mut dec).decode_byte_array(4).unwrap();

    assert_eq!(v, dec_v);
    println!("Decoder test passed");
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
            type Value = InnerStruct;
            fn decode<D: Decoder>(mut dec: D) -> Result<Self::Value, D::Error> {
                let v_u32 = dec.decode_u32()?;
                let v_buf = dec.decode_byte_array(4)?;
                let v_array = v_buf.try_into().unwrap();
                Ok(InnerStruct { v_u32, v_array })
            }
        }
        
        impl Decode for TestStruct {
            type Value = TestStruct;
            fn decode<D: Decoder>(mut dec: D) -> Result<Self::Value, D::Error> {
                let v_i32 = dec.decode_i32()?;
                let b = dec.decode_bool()?;
                let v_inner_struct = InnerStruct::decode(dec)?;
                Ok(TestStruct { v_i32, b, v_inner_struct })
            }
        }
        
        let buf = vec![0xfe,0xff,0xff,0xff,// i32, value: -2
            1,// bool, value: true,
            0,1,0,0,// u32, value: 256
            0,1,2,3// array, value: [0, 1, 2, 3]
        ];
        
        let mut dec = SimpleBinaryDecoder::new(buf.clone());
        let test_struct = TestStruct::decode(&mut dec).unwrap();
        assert_eq!(test_struct.v_i32, -2);
        assert_eq!(test_struct.b, true);
        assert_eq!(test_struct.v_inner_struct.v_u32, 256);
        assert_eq!(test_struct.v_inner_struct.v_array, [0, 1, 2, 3]);
        println!("Decoder test passed");
    }
}
