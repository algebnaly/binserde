use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Attribute, Data, DeriveInput, Error as SynError, Fields, parse_macro_input};

use crate::enum_helper::{
    DiscriminantType, discriminant_type_name, discriminants_expr, is_catch_all_variant,
    parse_repr_type, type_matches_discriminant,
};

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
    let is_named = matches!(&d_struct.fields, Fields::Named(_));

    let decode_fields: Vec<TokenStream> = d_struct
        .fields
        .iter()
        .enumerate()
        .map(|(i, field)| {
            let ty = &field.ty;
            if let Some(name) = &field.ident {
                quote! {
                    let #name = ::binserde::StructDecoder::decode_field::<#ty>(&mut s)?;
                }
            } else {
                let var = format_ident!("_f{}", i);
                quote! {
                    let #var = ::binserde::StructDecoder::decode_field::<#ty>(&mut s)?;
                }
            }
        })
        .collect();

    let init = if is_named {
        let idents: Vec<_> = d_struct
            .fields
            .iter()
            .map(|f| {
                let name = f.ident.as_ref().unwrap();
                quote! { #name }
            })
            .collect();
        quote! { Self { #( #idents ),* } }
    } else {
        let idents: Vec<_> = (0..field_count)
            .map(|i| {
                let var = format_ident!("_f{}", i);
                quote! { #var }
            })
            .collect();
        quote! { Self(#( #idents ),*) }
    };

    quote! {
        let mut s = decoder.decode_struct(#field_count)?;
        #( #decode_fields )*
        ::binserde::StructDecoder::end(&mut s)?;
        Ok(#init)
    }
}

fn decode_impl_for_enum(d_enum: syn::DataEnum, attrs: &[Attribute]) -> TokenStream {
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
        return SynError::new(
            second.ident.span(),
            "only one #[binserde(catch_all)] variant is allowed",
        )
        .into_compile_error();
    }

    let discriminants = d_enum
        .variants
        .iter()
        .map(|variant| variant.discriminant.clone().map(|(_, expr)| expr));

    let disc_exprs = discriminants_expr(discriminants);

    let create_enum_decoder_expr = quote! {
        let mut enum_decoder = ::binserde::Decoder::decode_variant(decoder)?;
    };

    let decode_discriminant_expr = decode_discriminant(&discriminant_type);

    let catch_all_position = d_enum.variants.iter().position(|v| is_catch_all_variant(v));

    let variants: Vec<&syn::Variant> = d_enum.variants.iter().collect();
    let mut decode_exprs = gen_variant_decode_arms(
        &variants,
        &disc_exprs,
        catch_all_position,
        &discriminant_type,
    );

    if let Some(idx) = catch_all_position {
        let catch_all_arm = decode_exprs.remove(idx);
        decode_exprs.push(catch_all_arm);
    } else {
        decode_exprs.push(quote! {
            _ => Err(::binserde::EnumDecoder::on_unknown_discriminant(&mut enum_decoder, disc_val)),
        });
    }

    quote! {
        #create_enum_decoder_expr
        #decode_discriminant_expr
        match disc_val {
            #( #decode_exprs ),*
        }
    }
}

fn gen_variant_decode_arms(
    variants: &[&syn::Variant],
    disc_exprs: &[TokenStream],
    catch_all_position: Option<usize>,
    discriminant_type: &DiscriminantType,
) -> Vec<TokenStream> {
    let mut disc_iter = disc_exprs.iter();
    variants
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let variant_name = &v.ident;
            let disc_expr = disc_iter.next().unwrap();
            let is_catch_all = catch_all_position == Some(i);
            let pattern = if is_catch_all {
                quote! { _ }
            } else {
                quote! { #disc_expr }
            };

            match &v.fields {
                Fields::Unit => {
                    quote! {
                        #pattern => {
                            Ok(Self::#variant_name)
                        }
                    }
                }
                Fields::Unnamed(fields) => {
                    if is_catch_all {
                        let field_count = fields.unnamed.len();
                        match field_count {
                            1 => {
                                let ty = &fields.unnamed[0].ty;
                                if !type_matches_discriminant(ty, discriminant_type) {
                                    return SynError::new_spanned(
                                        ty.clone(),
                                        format!(
                                            "catch_all variant payload type must be `{}`",
                                            discriminant_type_name(discriminant_type)
                                        ),
                                    )
                                    .into_compile_error();
                                }
                            }
                            2 => {
                                let ty = &fields.unnamed[0].ty;
                                if !type_matches_discriminant(ty, discriminant_type) {
                                    return SynError::new_spanned(
                                        ty.clone(),
                                        format!(
                                            "catch_all variant first field type must be `{}`",
                                            discriminant_type_name(discriminant_type)
                                        ),
                                    )
                                    .into_compile_error();
                                }
                            }
                            _ => {
                                return SynError::new(
                                    v.ident.span(),
                                    "catch_all variant must be unit, newtype (single field), or tuple with 2 fields",
                                )
                                .into_compile_error();
                            }
                        }

                        if field_count == 1 {
                            quote! {
                                #pattern => {
                                    Ok(Self::#variant_name(disc_val))
                                }
                            }
                        } else {
                            let ty = &fields.unnamed[1].ty;
                            let var = format_ident!("_f1");
                            quote! {
                                #pattern => {
                                    let #var = ::binserde::EnumDecoder::decode_field::<#ty>(&mut enum_decoder)?;
                                    Ok(Self::#variant_name(disc_val, #var))
                                }
                            }
                        }
                    } else {
                        let field_vars: Vec<_> = fields
                            .unnamed
                            .iter()
                            .enumerate()
                            .map(|(i, f)| {
                                let var = format_ident!("_f{}", i);
                                let ty = &f.ty;
                                quote! {
                                    let #var = ::binserde::EnumDecoder::decode_field::<#ty>(&mut enum_decoder)?;
                                }
                            })
                            .collect();

                        let field_idents: Vec<_> = fields
                            .unnamed
                            .iter()
                            .enumerate()
                            .map(|(i, _)| format_ident!("_f{}", i))
                            .collect();

                        quote! {
                            #pattern => {
                                #( #field_vars )*
                                Ok(Self::#variant_name(#( #field_idents ),*))
                            }
                        }
                    }
                }
                Fields::Named(fields) => {
                    if is_catch_all {
                        SynError::new(
                            v.ident.span(),
                            "catch_all cannot be applied to variants with named fields",
                        )
                        .into_compile_error()
                    } else {
                        let field_vars: Vec<_> = fields
                            .named
                            .iter()
                            .map(|f| {
                                let name = f.ident.as_ref().unwrap();
                                let ty = &f.ty;
                                quote! {
                                    let #name = ::binserde::EnumDecoder::decode_field::<#ty>(&mut enum_decoder)?;
                                }
                            })
                            .collect();

                        let field_idents: Vec<_> = fields
                            .named
                            .iter()
                            .map(|f| f.ident.as_ref().unwrap())
                            .collect();

                        quote! {
                            #pattern => {
                                #( #field_vars )*
                                Ok(Self::#variant_name { #( #field_idents ),* })
                            }
                        }
                    }
                }
            }
        })
        .collect()
}

fn decode_discriminant(discriminant_type: &DiscriminantType) -> TokenStream {
    match discriminant_type {
        DiscriminantType::U8 => {
            quote! {
                let disc_val = ::binserde::EnumDecoder::decode_discriminant_u8(&mut enum_decoder)?;
            }
        }
        DiscriminantType::U16 => {
            quote! {
                let disc_val = ::binserde::EnumDecoder::decode_discriminant_u16(&mut enum_decoder)?;
            }
        }
        DiscriminantType::U32 => {
            quote! {
                let disc_val = ::binserde::EnumDecoder::decode_discriminant_u32(&mut enum_decoder)?;
            }
        }
        DiscriminantType::U64 => {
            quote! {
                let disc_val = ::binserde::EnumDecoder::decode_discriminant_u64(&mut enum_decoder)?;
            }
        }
        DiscriminantType::U128 => {
            quote! {
                let disc_val = ::binserde::EnumDecoder::decode_discriminant_u128(&mut enum_decoder)?;
            }
        }
        DiscriminantType::USize => {
            quote! {
                let disc_val = ::binserde::EnumDecoder::decode_discriminant_usize(&mut enum_decoder)?;
            }
        }
        DiscriminantType::I8 => {
            quote! {
                let disc_val = ::binserde::EnumDecoder::decode_discriminant_i8(&mut enum_decoder)?;
            }
        }
        DiscriminantType::I16 => {
            quote! {
                let disc_val = ::binserde::EnumDecoder::decode_discriminant_i16(&mut enum_decoder)?;
            }
        }
        DiscriminantType::I32 => {
            quote! {
                let disc_val = ::binserde::EnumDecoder::decode_discriminant_i32(&mut enum_decoder)?;
            }
        }
        DiscriminantType::I64 => {
            quote! {
                let disc_val = ::binserde::EnumDecoder::decode_discriminant_i64(&mut enum_decoder)?;
            }
        }
        DiscriminantType::I128 => {
            quote! {
                let disc_val = ::binserde::EnumDecoder::decode_discriminant_i128(&mut enum_decoder)?;
            }
        }
        DiscriminantType::ISize => {
            quote! {
                let disc_val = ::binserde::EnumDecoder::decode_discriminant_isize(&mut enum_decoder)?;
            }
        }
    }
}
