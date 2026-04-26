pub trait Decode {
    type Value;
    fn decode<D: Decoder>(decoder: D) -> Result<Self::Value, D::Error>;
}

pub trait Decoder {
    type Error;
    fn decode_unit(&mut self) -> Result<(), Self::Error>;
    fn decode_bool(&mut self) -> Result<bool, Self::Error>;
    fn decode_u8(&mut self) -> Result<u8, Self::Error>;
    fn decode_u16(&mut self) -> Result<u16, Self::Error>;
    fn decode_u32(&mut self) -> Result<u32, Self::Error>;
    fn decode_u64(&mut self) -> Result<u64, Self::Error>;
    fn decode_u128(&mut self) -> Result<u128, Self::Error> {
        unimplemented!()
    }
    fn decode_i8(&mut self) -> Result<i8, Self::Error>;
    fn decode_i16(&mut self) -> Result<i16, Self::Error>;
    fn decode_i32(&mut self) -> Result<i32, Self::Error>;
    fn decode_i64(&mut self) -> Result<i64, Self::Error>;
    fn decode_i128(&mut self) -> Result<i128, Self::Error> {
        unimplemented!()
    }

    fn decode_f32(&mut self) -> Result<f32, Self::Error>;
    fn decode_f64(&mut self) -> Result<f64, Self::Error>;

    fn decode_bytes(&mut self) -> Result<Vec<u8>, Self::Error>;
    fn decode_byte_array(&mut self, len: usize) -> Result<Vec<u8>, Self::Error>;
}

impl<'a, D: Decoder> Decoder for &'a mut D {
    type Error = D::Error;
    
    fn decode_unit(&mut self) -> Result<(), Self::Error> {
        (**self).decode_unit()
    }
    
    fn decode_bool(&mut self) -> Result<bool, Self::Error> {
        (**self).decode_bool()
    }
    
    fn decode_u8(&mut self) -> Result<u8, Self::Error> {
        (**self).decode_u8()
    }
    
    fn decode_u16(&mut self) -> Result<u16, Self::Error> {
        (**self).decode_u16()
    }
    
    fn decode_u32(&mut self) -> Result<u32, Self::Error> {
        (**self).decode_u32()
    }
    
    fn decode_u64(&mut self) -> Result<u64, Self::Error> {
        (**self).decode_u64()
    }
    
    fn decode_u128(&mut self) -> Result<u128, Self::Error> {
        (**self).decode_u128()
    }
    
    fn decode_i8(&mut self) -> Result<i8, Self::Error> {
        (**self).decode_i8()
    }
    
    fn decode_i16(&mut self) -> Result<i16, Self::Error> {
        (**self).decode_i16()
    }
    
    fn decode_i32(&mut self) -> Result<i32, Self::Error> {
        (**self).decode_i32()
    }
    
    fn decode_i64(&mut self) -> Result<i64, Self::Error> {
        (**self).decode_i64()
    }
    
    fn decode_i128(&mut self) -> Result<i128, Self::Error> {
        (**self).decode_i128()
    }
    
    fn decode_f32(&mut self) -> Result<f32, Self::Error> {
        (**self).decode_f32()
    }
    
    fn decode_f64(&mut self) -> Result<f64, Self::Error> {
        (**self).decode_f64()
    }
    
    fn decode_bytes(&mut self) -> Result<Vec<u8>, Self::Error> {
        (**self).decode_bytes()
    }
    
    fn decode_byte_array(&mut self, len: usize) -> Result<Vec<u8>, Self::Error> {
        (**self).decode_byte_array(len)
    }
}