pub use binserde_core::{
    Discriminant,
    de::{Decode, Decoder, EnumDecoder, MapDecoder, SeqDecoder, StructDecoder, TupleDecoder},
    en::{Encode, Encoder, MapEncoder, SeqEncoder, StructEncoder, TupleEncoder},
};
pub use binserde_derive::{Decode, Encode};
