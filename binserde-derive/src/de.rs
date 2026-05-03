use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Data, DeriveInput, Error as SynError, Fields, parse_macro_input};

use crate::enum_helper::{DiscriminantType, discriminants_expr, parse_repr_type};

#[allow(unused)]
const CATCH_ALL_ATTR_NAME: &str = "catch_all";

pub(crate) fn derive_decode_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let name = &derive_input.ident;
    let attrs = &derive_input.attrs;

    let expanded = match derive_input.data {
        Data::Struct(d_struct) => decode_impl_for_struct(d_struct),
        Data::Enum(d_enum) => decode_impl_for_enum(d_enum, attrs),
        Data::Union(_d_union) => {
            unimplemented!("Union types are not supported yet")
        }
    };
    let impl_block = quote! {
      impl ::binserde::Decode for #name {
          fn decode<D: ::binserde::Decoder>(decoder: D) -> Result<Self, D::Error> {
              #expanded
          }
      }
    };
    proc_macro::TokenStream::from(impl_block)
}

fn decode_impl_for_struct(d_struct: syn::DataStruct) -> TokenStream {
    let field_count = d_struct.fields.iter().count();

    let field_names: Vec<_> = d_struct
        .fields
        .iter()
        .map(|field| field.ident.as_ref().unwrap().clone())
        .collect();
    let field_tys: Vec<_> = d_struct.fields.iter().map(|field| &field.ty).collect();

    let field_idents = field_names.clone();

    let decode_fields = field_names.iter().zip(field_tys.iter()).map(|(name, ty)| {
        quote! {
            let #name = ::binserde::StructDecoder::decode_field::<#ty>(&mut s)?;
        }
    });

    quote! {
        let mut s = decoder.decode_struct(#field_count)?;
        #( #decode_fields )*
        ::binserde::StructDecoder::end(&mut s)?;
        Ok(Self { #( #field_idents ),* })
    }
}

fn decode_impl_for_enum(d_enum: syn::DataEnum, attrs: &[Attribute]) -> TokenStream {
    let discriminant_type = parse_repr_type(attrs).unwrap_or(DiscriminantType::USize);

    let mut catch_all_seen = false;
    // #[binserde(catch_all)] attribute check
    for variant in &d_enum.variants {
        if variant.attrs.iter().any(|a| is_catch_all_attr(a)) {
            if catch_all_seen {
                return SynError::new(
                    variant.ident.span(),
                    "only one #[binserde(catch_all)] variant is allowed",
                )
                .into_compile_error();
            }
            catch_all_seen = true;
            if let Err(e) = validate_catch_all_variant(variant, &discriminant_type) {
                return e.into_compile_error();
            }
        }
    }
    let discriminants = d_enum
        .variants
        .iter()
        .map(|variant| variant.discriminant.clone().map(|(_, expr)| expr));

    let disc_exprs = discriminants_expr(discriminants);

    let create_enum_decoder_expr = quote! {
        let mut enum_decoder = ::binserde::Decoder::decode_variant(decoder)?;
    };

    let decode_discriminant_expr = match discriminant_type {
        DiscriminantType::U8 => {
            quote! {
                let disc_val = ::binserde::EnumDecoder::decode_discriminant_u8(enum_decoder)?;
            }
        }
        DiscriminantType::U16 => {
            quote! {
                let disc_val = ::binserde::EnumDecoder::decode_discriminant_u16(enum_decoder)?;
            }
        }
        DiscriminantType::U32 => {
            quote! {
                let disc_val = ::binserde::EnumDecoder::decode_discriminant_u32(enum_decoder)?;
            }
        }
        DiscriminantType::U64 => {
            quote! {
                let disc_val = ::binserde::EnumDecoder::decode_discriminant_u64(enum_decoder)?;
            }
        }
        DiscriminantType::U128 => {
            quote! {
                let disc_val = ::binserde::EnumDecoder::decode_discriminant_u128(enum_decoder)?;
            }
        }
        DiscriminantType::USize => {
            quote! {
                let disc_val = ::binserde::EnumDecoder::decode_discriminant_usize(enum_decoder)?;
            }
        }
        DiscriminantType::I8 => {
            quote! {
                let disc_val = ::binserde::EnumDecoder::decode_discriminant_i8(enum_decoder)?;
            }
        }
        DiscriminantType::I16 => {
            quote! {
                let disc_val = ::binserde::EnumDecoder::decode_discriminant_i16(enum_decoder)?;
            }
        }
        DiscriminantType::I32 => {
            quote! {
                let disc_val = ::binserde::EnumDecoder::decode_discriminant_i32(enum_decoder)?;
            }
        }
        DiscriminantType::I64 => {
            quote! {
                let disc_val = ::binserde::EnumDecoder::decode_discriminant_i64(enum_decoder)?;
            }
        }
        DiscriminantType::I128 => {
            quote! {
                let disc_val = ::binserde::EnumDecoder::decode_discriminant_i128(enum_decoder)?;
            }
        }
        DiscriminantType::ISize => {
            quote! {
                let disc_val = ::binserde::EnumDecoder::decode_discriminant_isize(enum_decoder)?;
            }
        }
    };
    let decode_exprs = d_enum
        .variants
        .iter()
        .zip(disc_exprs)
        .map(|(v, disc_expr)| {
            match &v.fields {
                Fields::Unit => {}
                Fields::Unnamed(fields) => {}
                Fields::Named(fields) => {}
            }
            quote! {
                _ if #disc_expr == disc_val => {
                    ::binserde::EnumDecoder::decode_field(&mut enum_decoder);
                }
            }
        });
    quote! {
        #create_enum_decoder_expr
        #decode_discriminant_expr
        match disc_val {
            #( #decode_exprs)*
        }
    }
}

fn is_catch_all_attr(attr: &Attribute) -> bool {
    attr.path().is_ident("binserde")
        && attr
            .parse_args::<syn::Ident>()
            .ok()
            .map(|i| i == "catch_all")
            .unwrap_or(false)
}

fn validate_catch_all_variant(
    variant: &syn::Variant,
    disc_type: &DiscriminantType,
) -> Result<(), SynError> {
    match &variant.fields {
        syn::Fields::Unit => Ok(()),
        syn::Fields::Unnamed(fields) => match fields.unnamed.len() {
            1 => {
                let ty = &fields.unnamed[0].ty;
                if type_matches_discriminant(ty, disc_type) {
                    Ok(())
                } else {
                    Err(SynError::new_spanned(
                        ty.clone(),
                        format!(
                            "catch_all newtype variant payload type must be `{}`",
                            discriminant_type_name(disc_type)
                        ),
                    ))
                }
            }
            2 => {
                let ty = &fields.unnamed[0].ty;
                if type_matches_discriminant(ty, disc_type) {
                    Ok(())
                } else {
                    Err(SynError::new_spanned(
                        ty.clone(),
                        format!(
                            "catch_all tuple variant first field type must be `{}`",
                            discriminant_type_name(disc_type)
                        ),
                    ))
                }
            }
            _ => Err(SynError::new(
                variant.ident.span(),
                "catch_all variant must be unit, newtype (single field), or tuple with 2 fields",
            )),
        },
        syn::Fields::Named(_) => Err(SynError::new(
            variant.ident.span(),
            "catch_all cannot be applied to variants with named fields",
        )),
    }
}

fn type_matches_discriminant(ty: &syn::Type, disc_type: &DiscriminantType) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(ident) = type_path.path.get_ident() {
            return ident == discriminant_type_name(disc_type);
        }
    }
    false
}

fn discriminant_type_name(disc_type: &DiscriminantType) -> &str {
    match disc_type {
        DiscriminantType::U8 => "u8",
        DiscriminantType::U16 => "u16",
        DiscriminantType::U32 => "u32",
        DiscriminantType::U64 => "u64",
        DiscriminantType::U128 => "u128",
        DiscriminantType::USize => "usize",
        DiscriminantType::I8 => "i8",
        DiscriminantType::I16 => "i16",
        DiscriminantType::I32 => "i32",
        DiscriminantType::I64 => "i64",
        DiscriminantType::I128 => "i128",
        DiscriminantType::ISize => "isize",
    }
}
