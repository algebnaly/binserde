use std::ops::Deref;

pub use binserde_core::{
    Discriminant,
    de::{Decode, Decoder, EnumDecoder, MapDecoder, SeqDecoder, StructDecoder, TupleDecoder},
    en::{Encode, Encoder, MapEncoder, SeqEncoder, StructEncoder, TupleEncoder},
};

#[derive(Debug, Clone)]
pub struct ByteBuf(Vec<u8>);

impl ByteBuf {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl Encode for ByteBuf {
    fn encode<E: Encoder>(&self, encoder: E) -> Result<(), E::Error> {
        encoder.encode_bytes(&self.0)
    }
}

impl From<&[u8]> for ByteBuf {
    fn from(bytes: &[u8]) -> Self {
        Self(bytes.to_vec())
    }
}

impl<const N: usize> From<&[u8; N]> for ByteBuf {
    fn from(bytes: &[u8; N]) -> Self {
        Self(bytes.to_vec())
    }
}

impl AsRef<[u8]> for ByteBuf {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Deref for ByteBuf {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl Decode for ByteBuf {
    fn decode<D: Decoder>(decoder: D) -> Result<Self, D::Error> {
        let v = decoder.decode_bytes()?;
        Ok(Self(v))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ByteArray<const N: usize>([u8; N]);

impl<const N: usize> ByteArray<N> {
    pub const fn zero() -> Self {
        Self([0; N])
    }
}

impl<const N: usize> Encode for ByteArray<N> {
    fn encode<E: Encoder>(&self, encoder: E) -> Result<(), E::Error> {
        encoder.encode_byte_array(&self.0)
    }
}

impl<const N: usize> Decode for ByteArray<N> {
    fn decode<D: Decoder>(decoder: D) -> Result<Self, D::Error> {
        let v = decoder.decode_byte_array()?;
        Ok(Self(v))
    }
}

pub use binserde_derive::{Decode, Encode};
