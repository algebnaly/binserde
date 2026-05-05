use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Attribute, Data, DeriveInput, Error as SynError, parse_macro_input};

use crate::enum_helper::{
    DiscriminantType, discriminant_type_name, discriminants_expr, is_catch_all_variant, parse_repr,
    parse_repr_type, type_matches_discriminant,
};

pub(crate) fn derive_encode_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let name = &derive_input.ident;

    let expanded = match derive_input.data {
        Data::Struct(d_struct) => Ok(encode_impl_for_struct(d_struct)),
        Data::Enum(d_enum) => encode_impl_for_enum(d_enum, &derive_input.attrs),
        Data::Union(_d_union) => {
            unimplemented!("Union types are not supported yet")
        }
    };

    let expanded = match expanded {
        Ok(tokens) => tokens,
        Err(e) => return proc_macro::TokenStream::from(e.into_compile_error()),
    };

    let impl_block = quote! {
      impl ::binserde::Encode for #name {
          fn encode<E: ::binserde::Encoder>(&self, encoder: E) -> Result<(), E::Error> {
              #expanded
          }
      }
    };
    proc_macro::TokenStream::from(impl_block)
}

fn encode_impl_for_struct(d_struct: syn::DataStruct) -> TokenStream {
    let field_count = d_struct.fields.iter().count();

    let encode_fields = d_struct.fields.iter().enumerate().map(|(i, field)| {
        let field_ref = if let Some(name) = &field.ident {
            quote! { &self.#name }
        } else {
            let idx = syn::Index::from(i);
            quote! { &self.#idx }
        };
        quote! {
            ::binserde::StructEncoder::encode_field(&mut s, #field_ref)?;
        }
    });

    quote! {
        let mut s = encoder.encode_struct(#field_count)?;
        #( #encode_fields )*
        ::binserde::StructEncoder::end(&mut s)
    }
}

fn encode_impl_for_enum(
    d_enum: syn::DataEnum,
    attrs: &[Attribute],
) -> Result<TokenStream, SynError> {
    let disc_variant = parse_repr(attrs);
    let discriminant_type = parse_repr_type(attrs).unwrap_or(DiscriminantType::USize);

    let catch_all_count = d_enum
        .variants
        .iter()
        .filter(|v| is_catch_all_variant(v))
        .count();

    if catch_all_count > 1 {
        let second = d_enum
            .variants
            .iter()
            .filter(|v| is_catch_all_variant(v))
            .nth(1)
            .unwrap();
        return Err(SynError::new(
            second.ident.span(),
            "only one #[binserde(catch_all)] variant is allowed",
        ));
    }

    let catch_all_position = d_enum.variants.iter().position(|v| is_catch_all_variant(v));

    let discriminants = d_enum
        .variants
        .iter()
        .map(|variant| variant.discriminant.clone().map(|(_, expr)| expr));
    let disc_exprs = discriminants_expr(discriminants);

    let variants: Vec<TokenStream> = d_enum
        .variants
        .iter()
        .zip(disc_exprs.iter())
        .enumerate()
        .map(|(i, (variant, disc_expr))| {
            let is_catch_all = catch_all_position == Some(i);
            let disc = quote! {
                ::binserde::Discriminant::#disc_variant(#disc_expr)
            };

            gen_variant_encode_arm(
                variant,
                disc,
                &disc_variant,
                is_catch_all,
                &discriminant_type,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(quote! {
        match self {
            #( #variants ),*
        }
    })
}

fn gen_variant_encode_arm(
    variant: &syn::Variant,
    disc: TokenStream,
    disc_variant: &TokenStream,
    is_catch_all: bool,
    discriminant_type: &DiscriminantType,
) -> Result<TokenStream, SynError> {
    let variant_name = &variant.ident;

    match &variant.fields {
        syn::Fields::Unit => Ok(quote! {
            Self::#variant_name => {
                encoder.encode_variant(#disc, &())
            }
        }),
        syn::Fields::Unnamed(_) => {
            if is_catch_all {
                let field_count = variant.fields.iter().count();
                match field_count {
                    1 => {
                        let ty = &variant.fields.iter().next().unwrap().ty;
                        if !type_matches_discriminant(ty, discriminant_type) {
                            return Err(SynError::new_spanned(
                                ty.clone(),
                                format!(
                                    "catch_all variant payload type must be {}, as #[repr(...)]",
                                    discriminant_type_name(discriminant_type),
                                ),
                            ));
                        }
                        let f0 = format_ident!("_f0");
                        let disc_val = quote! {
                            ::binserde::Discriminant::#disc_variant(*#f0)
                        };
                        Ok(quote! {
                            Self::#variant_name(#f0) => {
                                encoder.encode_variant(#disc_val, &())
                            }
                        })
                    }
                    2 => {
                        let first_ty = &variant.fields.iter().next().unwrap().ty;
                        if !type_matches_discriminant(first_ty, discriminant_type) {
                            return Err(SynError::new_spanned(
                                first_ty.clone(),
                                format!(
                                    "catch_all variant first field type must be `{}`",
                                    discriminant_type_name(discriminant_type)
                                ),
                            ));
                        }
                        let f0 = format_ident!("_f0");
                        let f1 = format_ident!("_f1");
                        let disc_val = quote! {
                            ::binserde::Discriminant::#disc_variant(*#f0)
                        };
                        Ok(quote! {
                            Self::#variant_name(#f0, #f1) => {
                                encoder.encode_variant(#disc_val, &(#f1,))
                            }
                        })
                    }
                    _ => Err(SynError::new(
                        variant.ident.span(),
                        "catch_all variant must be unit, newtype (single field), or tuple with 2 fields",
                    )),
                }
            } else {
                let fields: Vec<_> = variant
                    .fields
                    .iter()
                    .enumerate()
                    .map(|(i, _)| format_ident!("_f{}", i))
                    .collect();
                Ok(quote! {
                    Self::#variant_name(#(#fields),*) => {
                        encoder.encode_variant(#disc, &(#(#fields),*))
                    }
                })
            }
        }
        syn::Fields::Named(_) => {
            if is_catch_all {
                Err(SynError::new(
                    variant.ident.span(),
                    "catch_all cannot be applied to variants with named fields",
                ))
            } else {
                let field_names: Vec<_> = variant
                    .fields
                    .iter()
                    .map(|f| f.ident.as_ref().unwrap())
                    .collect();
                let refs: Vec<_> = field_names.iter().map(|n| quote! { &#n }).collect();
                Ok(quote! {
                    Self::#variant_name { #(#field_names),* } => {
                        encoder.encode_variant(#disc, &(#(#refs),*))
                    }
                })
            }
        }
    }
}
