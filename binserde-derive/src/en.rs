use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

pub(crate) fn derive_encode_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let name = &derive_input.ident;

    let expanded = match derive_input.data {
        Data::Struct(d_struct) => {
            let r = encode_impl_for_struct(d_struct);
            r
        }
        Data::Enum(_d_enum) => TokenStream::new(),
        Data::Union(_d_union) => {
            unimplemented!("Union types are not supported yet")
        }
    };

    let impl_block = quote! {
      impl binserde::Encode for #name {
          fn encode<E: binserde::Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
              #expanded
              Ok(())
          }
      }
    };
    proc_macro::TokenStream::from(impl_block)
}

fn encode_impl_for_struct(d_struct: syn::DataStruct) -> TokenStream {
    let fields = d_struct.fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_ty = &field.ty;
        quote! {
            <#field_ty as binserde::Encode>::encode(&self.#field_name, encoder)?;
        }
    });
    quote! {
        #( #fields )*
    }
}

fn _encode_impl_for_enum(d_enum: syn::DataEnum) -> TokenStream {
    let _variants = &d_enum.variants;
    let _expanded = quote! {};
    TokenStream::new()
}
