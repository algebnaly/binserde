use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Attribute, Data, DeriveInput, parse_macro_input};

use crate::enum_helper::{discriminants_expr, parse_repr};

pub(crate) fn derive_encode_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let name = &derive_input.ident;

    let expanded = match derive_input.data {
        Data::Struct(d_struct) => encode_impl_for_struct(d_struct),
        Data::Enum(d_enum) => encode_impl_for_enum(d_enum, &derive_input.attrs),
        Data::Union(_d_union) => {
            unimplemented!("Union types are not supported yet")
        }
    };

    let impl_block = quote! {
      impl binserde::Encode for #name {
          fn encode<E: binserde::Encoder>(&self, encoder: E) -> Result<(), E::Error> {
              #expanded
          }
      }
    };
    proc_macro::TokenStream::from(impl_block)
}

fn encode_impl_for_struct(d_struct: syn::DataStruct) -> TokenStream {
    let field_count = d_struct.fields.iter().count();

    let encode_fields = d_struct.fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        quote! {
            binserde::StructEncoder::encode_field(&mut s, &self.#field_name)?;
        }
    });

    quote! {
        let mut s = encoder.encode_struct(#field_count)?;
        #( #encode_fields )*
        binserde::StructEncoder::end(&mut s)
    }
}

fn encode_impl_for_enum(d_enum: syn::DataEnum, attrs: &[Attribute]) -> TokenStream {
    let disc_variant = parse_repr(attrs);

    let discriminants = d_enum
        .variants
        .iter()
        .map(|variant| variant.discriminant.clone().map(|(_, expr)| expr));
    let disc_exprs = discriminants_expr(discriminants);

    let variants = d_enum
        .variants
        .iter()
        .zip(disc_exprs.iter())
        .map(|(variant, disc_expr)| {
            let variant_name = &variant.ident;
            let disc = quote! { binserde::Discriminant::#disc_variant(#disc_expr) };

            match &variant.fields {
                syn::Fields::Unit => {
                    quote! {
                        Self::#variant_name => {
                            encoder.encode_variant(
                                #disc, &(),
                            )
                        }
                    }
                }
                syn::Fields::Unnamed(_) => {
                    let fields: Vec<_> = variant
                        .fields
                        .iter()
                        .enumerate()
                        .map(|(i, _)| format_ident!("_f{}", i))
                        .collect();
                    quote! {
                        Self::#variant_name(#(#fields),*) => {
                            encoder.encode_variant(
                                #disc, &(#(#fields),*),
                            )
                        }
                    }
                }
                syn::Fields::Named(_) => {
                    let field_names: Vec<_> = variant
                        .fields
                        .iter()
                        .map(|f| f.ident.as_ref().unwrap())
                        .collect();
                    let refs: Vec<_> = field_names.iter().map(|n| quote! { & #n }).collect();
                    quote! {
                        Self::#variant_name { #(#field_names),* } => {
                            encoder.encode_variant(#disc, &(#(#refs),*))
                        }
                    }
                }
            }
        });

    quote! {
        match self {
            #( #variants ),*
        }
    }
}

