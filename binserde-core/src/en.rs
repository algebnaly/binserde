// goal: define some reasonable default encode procedure, so that encoder did not to be too magic.
pub trait Encode {
    fn encode<E: Encoder>(&self, encoder: E) -> Result<(), E::Error>;
}

pub trait Encoder {
    type Error;
    type EnumEncoder: EnumEncoder<Error = Self::Error>;
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

    fn encode_some(self) -> Result<(), Self::Error>;
    fn encode_none(self) -> Result<(), Self::Error>;

    fn encode_bytes(self, value: &[u8]) -> Result<(), Self::Error>;
    fn encode_string(self, value: &str) -> Result<(), Self::Error>;

    fn encode_byte_array<const N: usize>(self, value: &[u8; N]) -> Result<(), Self::Error>;

    fn encode_variant(self) -> Result<Self::EnumEncoder, Self::Error>;

    fn encode_seq(self, len: Option<usize>) -> Result<Self::SeqEncoder, Self::Error>;

    fn encode_map(self, len: Option<usize>) -> Result<Self::MapEncoder, Self::Error>;

    fn encode_tuple(self, len: usize) -> Result<Self::TupleEncoder, Self::Error>;
}

// keep user from accidentally implementing EnumDiscriminant
mod sealed_mod {
    pub trait Sealed {}
}

pub trait EnumDiscriminant: sealed_mod::Sealed {}

macro_rules! impl_enum_discriminant {
    ($($ty:ty),*) => {
        $(
            impl sealed_mod::Sealed for $ty {}
            impl EnumDiscriminant for $ty {}
        )*
    };
}

impl_enum_discriminant!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

pub trait EnumEncoder {
    type Error;
    fn encode_variant<D: EnumDiscriminant, T: Encode>(
        &mut self,
        discriminant: D,
        variant_name: &str,
        value: &T,
    ) -> Result<(), Self::Error>;
    fn end(&mut self) -> Result<(), Self::Error>;
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
