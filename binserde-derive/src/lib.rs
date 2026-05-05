use proc_macro::TokenStream;

use crate::{de::derive_decode_impl, en::derive_encode_impl};

mod de;
mod en;
mod enum_helper;

#[proc_macro_derive(Encode, attributes(binserde))]
pub fn derive_encode(input: TokenStream) -> TokenStream {
    derive_encode_impl(input)
}

#[proc_macro_derive(Decode, attributes(binserde))]
pub fn derive_decode(input: TokenStream) -> TokenStream {
    derive_decode_impl(input)
}
