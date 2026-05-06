use std::ops::{Deref, DerefMut};

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

impl From<Vec<u8>> for ByteBuf {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl From<&[u8]> for ByteBuf {
    fn from(bytes: &[u8]) -> Self {
        Self(bytes.to_vec())
    }
}

impl From<&str> for ByteBuf {
    fn from(value: &str) -> Self {
        Self::from(value.as_bytes())
    }
}

impl<const N: usize> From<&[u8; N]> for ByteBuf {
    fn from(bytes: &[u8; N]) -> Self {
        Self(bytes.to_vec())
    }
}

impl Deref for ByteBuf {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl DerefMut for ByteBuf {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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

impl<const N: usize> Default for ByteArray<N> {
    fn default() -> Self {
        Self::zero()
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

impl<const N: usize> Deref for ByteArray<N> {
    type Target = [u8; N];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> DerefMut for ByteArray<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub use binserde_derive::{Decode, Encode};
