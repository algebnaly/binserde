pub trait Decode: Sized {
    fn decode<D: Decoder>(decoder: D) -> Result<Self, D::Error>;
}

pub trait Decoder {
    type Error;
    type EnumDecoder: EnumDecoder<Error = Self::Error>;
    type StructDecoder: StructDecoder<Error = Self::Error>;
    type SeqDecoder: SeqDecoder<Error = Self::Error>;
    type MapDecoder: MapDecoder<Error = Self::Error>;
    type TupleDecoder: TupleDecoder<Error = Self::Error>;

    fn decode_unit(self) -> Result<(), Self::Error>;
    fn decode_bool(self) -> Result<bool, Self::Error>;
    fn decode_u8(self) -> Result<u8, Self::Error>;
    fn decode_u16(self) -> Result<u16, Self::Error>;
    fn decode_u32(self) -> Result<u32, Self::Error>;
    fn decode_u64(self) -> Result<u64, Self::Error>;
    fn decode_u128(self) -> Result<u128, Self::Error>;
    fn decode_i8(self) -> Result<i8, Self::Error>;
    fn decode_i16(self) -> Result<i16, Self::Error>;
    fn decode_i32(self) -> Result<i32, Self::Error>;
    fn decode_i64(self) -> Result<i64, Self::Error>;
    fn decode_i128(self) -> Result<i128, Self::Error>;
    fn decode_f32(self) -> Result<f32, Self::Error>;
    fn decode_f64(self) -> Result<f64, Self::Error>;

    fn decode_bytes(self) -> Result<Vec<u8>, Self::Error>;
    fn decode_string(self) -> Result<String, Self::Error>;
    fn decode_byte_array<const N: usize>(self) -> Result<[u8; N], Self::Error>;

    fn decode_some(self) -> Result<(), Self::Error>;
    fn decode_none(self) -> Result<(), Self::Error>;

    fn decode_struct(self, name: &str, len: usize) -> Result<Self::StructDecoder, Self::Error>;

    fn decode_variant(self) -> Result<Self::EnumDecoder, Self::Error>;
    fn decode_seq(self, len: Option<usize>) -> Result<Self::SeqDecoder, Self::Error>;
    fn decode_map(self, len: Option<usize>) -> Result<Self::MapDecoder, Self::Error>;
    fn decode_tuple(self, len: usize) -> Result<Self::TupleDecoder, Self::Error>;
}

pub trait EnumDecoder {
    type Error;
    fn decode_variant<T: Decode>(
        &mut self,
    ) -> Result<(crate::en::Discriminant, String, T), Self::Error>;
    fn end(&mut self) -> Result<(), Self::Error>;
}

pub trait StructDecoder {
    type Error;
    fn decode_field<T: Decode>(&mut self) -> Result<T, Self::Error>;
    fn end(&mut self) -> Result<(), Self::Error>;
}

pub trait SeqDecoder {
    type Error;
    fn decode_element<T: Decode>(&mut self) -> Result<Option<T>, Self::Error>;
    fn end(&mut self) -> Result<(), Self::Error>;
}

pub trait MapDecoder {
    type Error;
    fn decode_key<T: Decode>(&mut self) -> Result<Option<T>, Self::Error>;
    fn decode_value<T: Decode>(&mut self) -> Result<T, Self::Error>;
    fn end(&mut self) -> Result<(), Self::Error>;
}

pub trait TupleDecoder {
    type Error;
    fn decode_element<T: Decode>(&mut self) -> Result<T, Self::Error>;
    fn end(&mut self) -> Result<(), Self::Error>;
}
