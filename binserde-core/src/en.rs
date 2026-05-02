pub trait Encode {
    fn encode<E: Encoder>(&self, encoder: E) -> Result<(), E::Error>;
}

pub trait Encoder {
    type Error;
    type StructEncoder: StructEncoder<Error = Self::Error>;
    type SeqEncoder: SeqEncoder<Error = Self::Error>;
    type MapEncoder: MapEncoder<Error = Self::Error>;
    type TupleEncoder: TupleEncoder<Error = Self::Error>;
    fn encode_unit(self) -> Result<(), Self::Error>;
    fn encode_bool(self, value: bool) -> Result<(), Self::Error>;
    fn encode_u8(self, value: u8) -> Result<(), Self::Error>;
    fn encode_u16(self, value: u16) -> Result<(), Self::Error>;
    fn encode_u32(self, value: u32) -> Result<(), Self::Error>;
    fn encode_u64(self, value: u64) -> Result<(), Self::Error>;
    fn encode_u128(self, value: u128) -> Result<(), Self::Error>;
    fn encode_i8(self, value: i8) -> Result<(), Self::Error>;
    fn encode_i16(self, value: i16) -> Result<(), Self::Error>;
    fn encode_i32(self, value: i32) -> Result<(), Self::Error>;
    fn encode_i64(self, value: i64) -> Result<(), Self::Error>;
    fn encode_i128(self, value: i128) -> Result<(), Self::Error>;
    fn encode_f32(self, value: f32) -> Result<(), Self::Error>;
    fn encode_f64(self, value: f64) -> Result<(), Self::Error>;

    fn encode_option<T: Encode>(self, value: Option<&T>) -> Result<(), Self::Error>;

    fn encode_bytes(self, value: &[u8]) -> Result<(), Self::Error>;
    fn encode_string(self, value: &str) -> Result<(), Self::Error>;

    fn encode_byte_array<const N: usize>(self, value: &[u8; N]) -> Result<(), Self::Error>;

    fn encode_struct(self, name: &str, len: usize) -> Result<Self::StructEncoder, Self::Error>;

    fn encode_variant<T: Encode>(
        self,
        discriminant: Discriminant,
        variant_name: &str,
        value: &T,
    ) -> Result<(), Self::Error>;

    fn encode_seq(self, len: usize) -> Result<Self::SeqEncoder, Self::Error>;

    fn encode_map(self, len: usize) -> Result<Self::MapEncoder, Self::Error>;

    fn encode_tuple(self, len: usize) -> Result<Self::TupleEncoder, Self::Error>;
}

pub enum Discriminant {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    USize(usize),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    ISize(isize),
}

pub trait StructEncoder {
    type Error;
    fn encode_field<T: Encode>(&mut self, name: &str, value: &T) -> Result<(), Self::Error>;
    fn end(&mut self) -> Result<(), Self::Error>;
}

pub trait SeqEncoder {
    type Error;
    fn encode_element<T: Encode>(&mut self, element: &T) -> Result<(), Self::Error>;
    fn end(&mut self) -> Result<(), Self::Error>;
}

pub trait MapEncoder {
    type Error;
    fn encode_key<T: Encode>(&mut self, key: &T) -> Result<(), Self::Error>;
    fn encode_value<T: Encode>(&mut self, value: &T) -> Result<(), Self::Error>;
    fn end(&mut self) -> Result<(), Self::Error>;
}

pub trait TupleEncoder {
    type Error;
    fn encode_element<T: Encode>(&mut self, element: &T) -> Result<(), Self::Error>;
    fn end(&mut self) -> Result<(), Self::Error>;
}
