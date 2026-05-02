use std::io::Write;

use binserde::{Discriminant, Encode, MapEncoder, SeqEncoder, StructEncoder};
use binserde_core::en::{Encoder, TupleEncoder};

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
    type MapEncoder = Self;
    type SeqEncoder = Self;
    type StructEncoder = Self;
    type TupleEncoder = Self;

    fn encode_unit(self) -> Result<(), Self::Error> {
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

    fn encode_option<T: Encode>(self, value: Option<&T>) -> Result<(), Self::Error> {
        match value {
            Some(v) => {
                self.output
                    .write_all(&1u8.to_le_bytes())
                    .map_err(|e| e.to_string())?;
                v.encode(self)
            }
            None => self
                .output
                .write_all(&0u8.to_le_bytes())
                .map_err(|e| e.to_string()),
        }
    }

    fn encode_bytes(self, value: &[u8]) -> Result<(), Self::Error> {
        self.encode_u32(value.len() as u32)?;
        self.output.write_all(value).map_err(|e| e.to_string())
    }

    fn encode_string(self, value: &str) -> Result<(), Self::Error> {
        self.encode_bytes(value.as_bytes())
    }

    fn encode_byte_array<const N: usize>(self, value: &[u8; N]) -> Result<(), Self::Error> {
        self.output.write_all(value).map_err(|e| e.to_string())
    }

    fn encode_struct(self, _name: &str, _len: usize) -> Result<Self::StructEncoder, Self::Error> {
        Ok(self)
    }

    fn encode_variant<T: Encode>(
        self,
        _discriminant: Discriminant,
        _variant_name: &str,
        value: &T,
    ) -> Result<(), Self::Error> {
        encode_discriminant(self, _discriminant)?;
        value.encode(self)
    }

    fn encode_seq(self, _len: usize) -> Result<Self::SeqEncoder, Self::Error> {
        Ok(self)
    }

    fn encode_map(self, _len: usize) -> Result<Self::MapEncoder, Self::Error> {
        Ok(self)
    }

    fn encode_tuple(self, _len: usize) -> Result<Self::TupleEncoder, Self::Error> {
        Ok(self)
    }
}

fn encode_discriminant(enc: &mut SimpleBinaryEncoder, d: Discriminant) -> Result<(), String> {
    match d {
        Discriminant::U8(v) => enc.encode_u128(v as u128),
        Discriminant::U16(v) => enc.encode_u128(v as u128),
        Discriminant::U32(v) => enc.encode_u128(v as u128),
        Discriminant::U64(v) => enc.encode_u128(v as u128),
        Discriminant::U128(v) => enc.encode_u128(v as u128),
        Discriminant::USize(v) => enc.encode_u128(v as u128),
        Discriminant::I8(v) => enc.encode_u128(v as u128),
        Discriminant::I16(v) => enc.encode_u128(v as u128),
        Discriminant::I32(v) => enc.encode_u128(v as u128),
        Discriminant::I64(v) => enc.encode_u128(v as u128),
        Discriminant::I128(v) => enc.encode_u128(v as u128),
        Discriminant::ISize(v) => enc.encode_u128(v as u128),
    }
}

impl SeqEncoder for &mut SimpleBinaryEncoder {
    type Error = String;

    fn encode_element<T: Encode>(&mut self, element: &T) -> Result<(), Self::Error> {
        element.encode(&mut **self)
    }

    fn end(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl MapEncoder for &mut SimpleBinaryEncoder {
    type Error = String;

    fn encode_key<T: Encode>(&mut self, key: &T) -> Result<(), Self::Error> {
        key.encode(&mut **self)
    }

    fn encode_value<T: Encode>(&mut self, value: &T) -> Result<(), Self::Error> {
        value.encode(&mut **self)
    }

    fn end(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl StructEncoder for &mut SimpleBinaryEncoder {
    type Error = String;

    fn encode_field<T: Encode>(&mut self, _name: &str, value: &T) -> Result<(), Self::Error> {
        value.encode(&mut **self)
    }

    fn end(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl TupleEncoder for &mut SimpleBinaryEncoder {
    type Error = String;

    fn encode_element<T: Encode>(&mut self, element: &T) -> Result<(), Self::Error> {
        element.encode(&mut **self)
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
}
