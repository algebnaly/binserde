use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

pub(crate) fn derive_decode_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let name = &derive_input.ident;

    let expanded = match derive_input.data {
        Data::Struct(d_struct) => decode_impl_for_struct(d_struct),
        Data::Enum(_d_enum) => TokenStream::new(),
        Data::Union(_d_union) => {
            unimplemented!("Union types are not supported yet")
        }
    };
    eprintln!("expanded: {}", expanded);

    let impl_block = quote! {
      impl binserde::Decode for #name {
          fn decode<D: binserde::Decoder>(mut decoder: D) -> Result<Self, D::Error> {
              #expanded
          }
      }
    };
    proc_macro::TokenStream::from(impl_block)
}

fn decode_impl_for_struct(d_struct: syn::DataStruct) -> TokenStream {
    let field_names = d_struct.fields.iter().map(|field| {
        let field_name = &field.ident;
        field_name.clone()
    });
    let fields = d_struct.fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_ty = &field.ty;
        quote! {
            let #field_name = <#field_ty as binserde::Decode>::decode(&mut decoder)?;
        }
    });
    quote! {
        #( #fields )*
        Ok(Self {
            #( #field_names ),*
            })
    }
}
