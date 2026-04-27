use std::io::Write;

use binserde::{Encode, EnumEncoder, MapEncoder, SeqEncoder, StructEncoder};
use binserde_core::en::Encoder;

struct SimpleBinaryEncoder {
    output: Vec<u8>,
}

impl SimpleBinaryEncoder {
    fn new() -> Self {
        SimpleBinaryEncoder { output: Vec::new() }
    }

    fn into_bytes(self) -> Vec<u8> {
        self.output
    }
}

impl Encoder for &mut SimpleBinaryEncoder {
    type Error = String;
    type EnumEncoder = Self;
    type MapEncoder = Self;
    type SeqEncoder = Self;
    type StructEncoder = Self;
    fn encode_unit(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn encode_bool(self, value: bool) -> Result<(), Self::Error> {
        self.output
            .write_all(&[value as u8])
            .map_err(|e| e.to_string())
    }

    fn encode_u8(self, value: u8) -> Result<(), Self::Error> {
        self.output
            .write_all(&value.to_le_bytes())
            .map_err(|e| e.to_string())
    }

    fn encode_u16(self, value: u16) -> Result<(), Self::Error> {
        self.output
            .write_all(&value.to_le_bytes())
            .map_err(|e| e.to_string())
    }

    fn encode_u32(self, value: u32) -> Result<(), Self::Error> {
        self.output
            .write_all(&value.to_le_bytes())
            .map_err(|e| e.to_string())
    }

    fn encode_u64(self, value: u64) -> Result<(), Self::Error> {
        self.output
            .write_all(&value.to_le_bytes())
            .map_err(|e| e.to_string())
    }

    fn encode_u128(self, value: u128) -> Result<(), Self::Error> {
        self.output
            .write_all(&value.to_le_bytes())
            .map_err(|e| e.to_string())
    }

    fn encode_i8(self, value: i8) -> Result<(), Self::Error> {
        self.output
            .write_all(&value.to_le_bytes())
            .map_err(|e| e.to_string())
    }

    fn encode_i16(self, value: i16) -> Result<(), Self::Error> {
        self.output
            .write_all(&value.to_le_bytes())
            .map_err(|e| e.to_string())
    }

    fn encode_i32(self, value: i32) -> Result<(), Self::Error> {
        self.output
            .write_all(&value.to_le_bytes())
            .map_err(|e| e.to_string())
    }

    fn encode_i64(self, value: i64) -> Result<(), Self::Error> {
        self.output
            .write_all(&value.to_le_bytes())
            .map_err(|e| e.to_string())
    }

    fn encode_i128(self, value: i128) -> Result<(), Self::Error> {
        self.output
            .write_all(&value.to_le_bytes())
            .map_err(|e| e.to_string())
    }

    fn encode_f32(self, value: f32) -> Result<(), Self::Error> {
        self.output
            .write_all(&value.to_le_bytes())
            .map_err(|e| e.to_string())
    }

    fn encode_f64(self, value: f64) -> Result<(), Self::Error> {
        self.output
            .write_all(&value.to_le_bytes())
            .map_err(|e| e.to_string())
    }

    fn encode_some(self) -> Result<(), Self::Error> {
        self.output
            .write_all(&1u8.to_le_bytes())
            .map_err(|e| e.to_string())
    }

    fn encode_none(self) -> Result<(), Self::Error> {
        self.output
            .write_all(&0u8.to_le_bytes())
            .map_err(|e| e.to_string())
    }

    fn encode_bytes(self, value: &[u8]) -> Result<(), Self::Error> {
        self.encode_u32(value.len() as u32)?;
        self.output.write_all(value).map_err(|e| e.to_string())
    }

    fn encode_string(&mut self, value: &str) -> Result<(), Self::Error> {
        self.encode_bytes(value.as_bytes())
    }

    fn encode_byte_array<const N: usize>(&mut self, value: &[u8; N]) -> Result<(), Self::Error> {
        self.output.write_all(value).map_err(|e| e.to_string())
    }

    fn encode_variant<T: Encode>(&mut self) -> Result<Self, Self::Error> {
        self.encode_u32(0)?;
        Ok(self)
    }

    fn encode_seq(self, len: Option<usize>) -> Result<Self::SeqEncoder, Self::Error> {
        Ok(self)
    }

    fn encode_map(self, len: Option<usize>) -> Result<Self::MapEncoder, Self::Error> {
        Ok(self)
    }
}

impl EnumEncoder for SimpleBinaryEncoder {
    type Error = String;
    type Discriminant = u32;
    fn encode_variant<T: Encode>(
        &mut self,
        discriminant: Self::Discriminant,
        _variant_name: &str,
        value: &T,
    ) -> Result<(), Self::Error> {
        self.encode_u32(discriminant)?;
        value.encode(self)
    }
    fn end(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl MapEncoder for &mut SimpleBinaryEncoder {
    type Error = String;

    fn encode_key<T: Encode>(self, key: &T) -> Result<(), Self::Error> {
        key.encode(self)
    }

    fn encode_value<T: Encode>(self, value: &T) -> Result<(), Self::Error> {
        value.encode(self)
    }

    fn end(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl SeqEncoder for SimpleBinaryEncoder {
    type Error = String;
    fn encode_element<T: Encode>(&mut self, element: &T) -> Result<(), Self::Error> {
        element.encode(self)
    }

    fn end(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl StructEncoder for SimpleBinaryEncoder {
    type Error = String;
    fn encode_field<T: Encode>(&mut self, _name: &str, _value: &T) -> Result<(), Self::Error> {
        Ok(())
    }
    fn end(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

fn main() {
    let mut enc = SimpleBinaryEncoder::new();

    enc.encode_bool(true).unwrap();
    enc.encode_bytes(b"hello").unwrap();

    let bytes = enc.into_bytes();
    println!("Encoded {} bytes: {:?}", bytes.len(), bytes);
    enc = SimpleBinaryEncoder::new();
    enc.encode_byte_array(&[1, 2, 3]).unwrap();
    let bytes = enc.into_bytes();
    println!("Encoded {} bytes: {:?}", bytes.len(), bytes);
}
