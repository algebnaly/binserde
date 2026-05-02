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

    fn decode_option<T: Decode>(self) -> Result<Option<T>, Self::Error>;

    fn decode_struct(self, len: usize) -> Result<Self::StructDecoder, Self::Error>;

    fn decode_variant(self) -> Result<Self::EnumDecoder, Self::Error>;
    fn decode_seq(self) -> Result<Self::SeqDecoder, Self::Error>;
    fn decode_map(self) -> Result<Self::MapDecoder, Self::Error>;
    fn decode_tuple(self, len: usize) -> Result<Self::TupleDecoder, Self::Error>;
}

pub trait EnumDecoder {
    type Error;

    fn decode_discriminant_u8(&mut self) -> Result<u8, Self::Error>;
    fn decode_discriminant_u16(&mut self) -> Result<u16, Self::Error>;
    fn decode_discriminant_u32(&mut self) -> Result<u32, Self::Error>;
    fn decode_discriminant_u64(&mut self) -> Result<u64, Self::Error>;
    fn decode_discriminant_u128(&mut self) -> Result<u128, Self::Error>;
    fn decode_discriminant_usize(&mut self) -> Result<usize, Self::Error>;
    fn decode_discriminant_i8(&mut self) -> Result<i8, Self::Error>;
    fn decode_discriminant_i16(&mut self) -> Result<i16, Self::Error>;
    fn decode_discriminant_i32(&mut self) -> Result<i32, Self::Error>;
    fn decode_discriminant_i64(&mut self) -> Result<i64, Self::Error>;
    fn decode_discriminant_i128(&mut self) -> Result<i128, Self::Error>;
    fn decode_discriminant_isize(&mut self) -> Result<isize, Self::Error>;

    fn decode_field<T: Decode>(&mut self) -> Result<T, Self::Error>;
    fn end(&mut self) -> Result<(), Self::Error>;
}

pub trait StructDecoder {
    type Error;
    fn decode_field<T: Decode>(&mut self) -> Result<T, Self::Error>;
    fn end(&mut self) -> Result<(), Self::Error>;
}

pub trait SeqDecoder {
    type Error;
    fn decode_len(&mut self) -> Result<usize, Self::Error>;
    fn decode_element<T: Decode>(&mut self) -> Result<T, Self::Error>;
    fn end(&mut self) -> Result<(), Self::Error>;
}

pub trait MapDecoder {
    type Error;
    fn decode_length(&mut self) -> Result<usize, Self::Error> {
        unimplemented!()
    }
    fn decode_key<T: Decode>(&mut self) -> Result<Option<T>, Self::Error>;
    fn decode_value<T: Decode>(&mut self) -> Result<T, Self::Error>;
    fn end(&mut self) -> Result<(), Self::Error>;
}

pub trait TupleDecoder {
    type Error;
    fn decode_length(&mut self) -> Result<usize, Self::Error> {
        unimplemented!()
    }
    fn decode_element<T: Decode>(&mut self) -> Result<T, Self::Error>;
    fn end(&mut self) -> Result<(), Self::Error>;
}
