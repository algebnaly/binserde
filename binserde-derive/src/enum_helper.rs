use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Ident};

pub(crate) enum DiscriminantType {
    U8,
    U16,
    U32,
    U64,
    U128,
    USize,
    I8,
    I16,
    I32,
    I64,
    I128,
    ISize,
}

pub(crate) fn is_catch_all_attr(attr: &Attribute) -> bool {
    attr.path().is_ident("binserde")
        && attr
            .parse_args::<Ident>()
            .ok()
            .map(|i| i == "catch_all")
            .unwrap_or(false)
}

pub(crate) fn is_catch_all_variant(variant: &syn::Variant) -> bool {
    variant.attrs.iter().any(|a| is_catch_all_attr(a))
}

pub(crate) fn type_matches_discriminant(ty: &syn::Type, disc_type: &DiscriminantType) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(ident) = type_path.path.get_ident() {
            return ident == discriminant_type_name(disc_type);
        }
    }
    false
}

pub(crate) fn discriminant_type_name(disc_type: &DiscriminantType) -> &str {
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

pub(crate) fn parse_repr_type(attrs: &[Attribute]) -> Option<DiscriminantType> {
    for attr in attrs {
        if !attr.path().is_ident("repr") {
            continue;
        }
        if let Ok(ty) = attr.parse_args::<syn::Type>() {
            if let syn::Type::Path(type_path) = &ty {
                if let Some(ident) = type_path.path.get_ident() {
                    let dt = match ident.to_string().as_str() {
                        "u8" => DiscriminantType::U8,
                        "u16" => DiscriminantType::U16,
                        "u32" => DiscriminantType::U32,
                        "u64" => DiscriminantType::U64,
                        "u128" => DiscriminantType::U128,
                        "usize" => DiscriminantType::USize,
                        "i8" => DiscriminantType::I8,
                        "i16" => DiscriminantType::I16,
                        "i32" => DiscriminantType::I32,
                        "i64" => DiscriminantType::I64,
                        "i128" => DiscriminantType::I128,
                        "isize" => DiscriminantType::ISize,
                        _ => return None,
                    };
                    return Some(dt);
                }
            }
        }
    }
    None
}

pub(crate) fn parse_repr(attrs: &[Attribute]) -> TokenStream {
    if let Some(disc_type) = parse_repr_type(attrs) {
        match disc_type {
            DiscriminantType::U8 => quote! { U8 },
            DiscriminantType::U16 => quote! { U16 },
            DiscriminantType::U32 => quote! { U32 },
            DiscriminantType::U64 => quote! { U64 },
            DiscriminantType::U128 => quote! { U128 },
            DiscriminantType::USize => quote! { USize },
            DiscriminantType::I8 => quote! { I8 },
            DiscriminantType::I16 => quote! { I16 },
            DiscriminantType::I32 => quote! { I32 },
            DiscriminantType::I64 => quote! { I64 },
            DiscriminantType::I128 => quote! { I128 },
            DiscriminantType::ISize => quote! { ISize },
        }
    } else {
        quote! { USize }
    }
}

pub(crate) fn discriminants_expr(
    discriminants: impl Iterator<Item = Option<syn::Expr>>,
) -> Vec<TokenStream> {
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
