use std::io::Write;

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

impl Encoder for SimpleBinaryEncoder {
    type Error = String;

    fn encode_unit(&mut self) -> Result<(), Self::Error> {
        // unit types encode to nothing
        Ok(())
    }

    fn encode_bool(&mut self, value: bool) -> Result<(), Self::Error> {
        self.output
            .write_all(&[value as u8])
            .map_err(|e| e.to_string())
    }

    fn encode_u8(&mut self, value: u8) -> Result<(), Self::Error> {
        self.output
            .write_all(&value.to_le_bytes())
            .map_err(|e| e.to_string())
    }

    fn encode_u16(&mut self, value: u16) -> Result<(), Self::Error> {
        self.output
            .write_all(&value.to_le_bytes())
            .map_err(|e| e.to_string())
    }

    fn encode_u32(&mut self, value: u32) -> Result<(), Self::Error> {
        self.output
            .write_all(&value.to_le_bytes())
            .map_err(|e| e.to_string())
    }

    fn encode_u64(&mut self, value: u64) -> Result<(), Self::Error> {
        self.output
            .write_all(&value.to_le_bytes())
            .map_err(|e| e.to_string())
    }

    fn encode_u128(&mut self, value: u128) -> Result<(), Self::Error> {
        self.output
            .write_all(&value.to_le_bytes())
            .map_err(|e| e.to_string())
    }

    fn encode_i8(&mut self, value: i8) -> Result<(), Self::Error> {
        self.output
            .write_all(&value.to_le_bytes())
            .map_err(|e| e.to_string())
    }

    fn encode_i16(&mut self, value: i16) -> Result<(), Self::Error> {
        self.output
            .write_all(&value.to_le_bytes())
            .map_err(|e| e.to_string())
    }

    fn encode_i32(&mut self, value: i32) -> Result<(), Self::Error> {
        self.output
            .write_all(&value.to_le_bytes())
            .map_err(|e| e.to_string())
    }

    fn encode_i64(&mut self, value: i64) -> Result<(), Self::Error> {
        self.output
            .write_all(&value.to_le_bytes())
            .map_err(|e| e.to_string())
    }

    fn encode_i128(&mut self, value: i128) -> Result<(), Self::Error> {
        self.output
            .write_all(&value.to_le_bytes())
            .map_err(|e| e.to_string())
    }

    fn encode_f32(&mut self, value: f32) -> Result<(), Self::Error> {
        self.output
            .write_all(&value.to_le_bytes())
            .map_err(|e| e.to_string())
    }

    fn encode_f64(&mut self, value: f64) -> Result<(), Self::Error> {
        self.output
            .write_all(&value.to_le_bytes())
            .map_err(|e| e.to_string())
    }

    fn encode_bytes(&mut self, value: &[u8]) -> Result<(), Self::Error> {
        self.encode_u32(value.len() as u32)?;
        self.output.write_all(value).map_err(|e| e.to_string())
    }

    fn encode_string(&mut self, value: &str) -> Result<(), Self::Error> {
        self.encode_bytes(value.as_bytes())
    }

    fn encode_byte_array<const N: usize>(&mut self, value: &[u8; N]) -> Result<(), Self::Error> {
        self.output.write_all(value).map_err(|e| e.to_string())
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
