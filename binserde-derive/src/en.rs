use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Attribute, Data, DeriveInput, parse_macro_input};

pub(crate) fn derive_encode_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let name = &derive_input.ident;

    let expanded = match derive_input.data {
        Data::Struct(d_struct) => encode_impl_for_struct(d_struct, name),
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

fn encode_impl_for_struct(d_struct: syn::DataStruct, name: &syn::Ident) -> TokenStream {
    let field_count = d_struct.fields.iter().count();
    let struct_name = name.to_string();

    let encode_fields = d_struct.fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_name_str = field_name.to_string();
        quote! {
            binserde::StructEncoder::encode_field(&mut s, #field_name_str, &self.#field_name)?;
        }
    });

    quote! {
        let mut s = encoder.encode_struct(#struct_name, #field_count)?;
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
            let variant_name_str = variant_name.to_string();
            let disc = quote! { binserde::Discriminant::#disc_variant(#disc_expr) };

            match &variant.fields {
                syn::Fields::Unit => {
                    quote! {
                        Self::#variant_name => {
                            let mut v = encoder.encode_variant()?;
                            binserde::EnumEncoder::encode_variant(
                                &mut v, #disc, #variant_name_str, &(),
                            )?;
                            binserde::EnumEncoder::end(&mut v)
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
                            let mut v = encoder.encode_variant()?;
                            binserde::EnumEncoder::encode_variant(
                                &mut v, #disc, #variant_name_str, &(#(#fields),*),
                            )?;
                            binserde::EnumEncoder::end(&mut v)
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
                            let mut v = encoder.encode_variant()?;
                            binserde::EnumEncoder::encode_variant(
                                &mut v, #disc, #variant_name_str, &(#(#refs),*),
                            )?;
                            binserde::EnumEncoder::end(&mut v)
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

fn parse_repr(attrs: &[Attribute]) -> TokenStream {
    for attr in attrs {
        if !attr.path().is_ident("repr") {
            continue;
        }
        if let Ok(ty) = attr.parse_args::<syn::Type>() {
            if let syn::Type::Path(type_path) = &ty {
                if let Some(ident) = type_path.path.get_ident() {
                    return match ident.to_string().as_str() {
                        "u8" => quote! { U8 },
                        "u16" => quote! { U16 },
                        "u32" => quote! { U32 },
                        "u64" => quote! { U64 },
                        "u128" => quote! { U128 },
                        "usize" => quote! { USize },
                        "i8" => quote! { I8 },
                        "i16" => quote! { I16 },
                        "i32" => quote! { I32 },
                        "i64" => quote! { I64 },
                        "i128" => quote! { I128 },
                        "isize" => quote! { ISize },
                        _ => continue,
                    };
                }
            }
        }
    }
    quote! { USize }
}

fn discriminants_expr(discriminants: impl Iterator<Item = Option<syn::Expr>>) -> Vec<TokenStream> {
    enum State {
        Value(usize),
        Expr(TokenStream),
    }

    let mut current = State::Value(0);
    let mut result = Vec::new();

    for disc in discriminants {
        match disc {
            Some(expr) => {
                result.push(quote! { #expr });
                current = State::Expr(quote! { #expr + 1 });
            }
            None => match &current {
                State::Value(v) => {
                    let lit = syn::LitInt::new(&v.to_string(), proc_macro2::Span::call_site());
                    result.push(quote! { #lit });
                    current = State::Value(v + 1);
                }
                State::Expr(e) => {
                    result.push(quote! { #e });
                    current = State::Expr(quote! { #e + 1 });
                }
            },
        }
    }

    result
}
