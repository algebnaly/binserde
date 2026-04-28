use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

pub(crate) fn derive_decode_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let name = &derive_input.ident;

    let expanded = match derive_input.data {
        Data::Struct(d_struct) => decode_impl_for_struct(d_struct, name),
        Data::Enum(d_enum) => decode_impl_for_enum(d_enum),
        Data::Union(_d_union) => {
            unimplemented!("Union types are not supported yet")
        }
    };
    let impl_block = quote! {
      impl binserde::Decode for #name {
          fn decode<D: binserde::Decoder>(decoder: D) -> Result<Self, D::Error> {
              #expanded
          }
      }
    };
    proc_macro::TokenStream::from(impl_block)
}

fn decode_impl_for_enum(_d_enum: syn::DataEnum) -> TokenStream {
    quote! { todo!("enum Decode not yet implemented") }
}

fn decode_impl_for_struct(d_struct: syn::DataStruct, name: &syn::Ident) -> TokenStream {
    let field_count = d_struct.fields.iter().count();
    let struct_name = name.to_string();

    let field_names: Vec<_> = d_struct
        .fields
        .iter()
        .map(|field| field.ident.as_ref().unwrap().clone())
        .collect();
    let field_tys: Vec<_> = d_struct.fields.iter().map(|field| &field.ty).collect();

    let field_idents = field_names.clone();

    let decode_fields = field_names.iter().zip(field_tys.iter()).map(|(name, ty)| {
        quote! {
            let #name = binserde::StructDecoder::decode_field::<#ty>(&mut s)?;
        }
    });

    quote! {
        let mut s = decoder.decode_struct(#struct_name, #field_count)?;
        #( #decode_fields )*
        binserde::StructDecoder::end(&mut s)?;
        Ok(Self { #( #field_idents ),* })
    }
}
