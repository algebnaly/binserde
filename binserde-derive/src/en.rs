use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Expr, parse_macro_input};

pub(crate) fn derive_encode_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let name = &derive_input.ident;

    let expanded = match derive_input.data {
        Data::Struct(d_struct) => encode_impl_for_struct(d_struct),
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

fn encode_impl_for_enum(d_enum: syn::DataEnum) -> TokenStream {
    let discriminants = d_enum
        .variants
        .iter()
        .map(|variant| variant.discriminant.clone().map(|(_, expr)| expr));
    let discriminants_expr = discriminants_expr(discriminants);

    let variants =
        d_enum
            .variants
            .iter()
            .zip(discriminants_expr.iter())
            .map(|(variant, discriminant)| {
                let variant_name = &variant.ident;
                let fields = variant.fields.iter().map(|field| {
                    let field_name = &field.ident;
                    let field_ty = &field.ty;
                    quote! {
                        <#field_ty as binserde::Encode>::encode(&self.#field_name, encoder)?;
                    }
                });
                quote! {
                    #variant_name => {
                        encoder.write_u8(0)?;
                        Ok(())
                    }
                }
            });
    let _variants = &d_enum.variants;
    let _expanded = quote! {};
    TokenStream::new()
}

fn discriminants_expr<I: Iterator<Item = Option<Expr>>>(discriminants: I) -> Vec<TokenStream> {
    enum Discriminant {
        Value(usize),
        Expr(TokenStream),
    }
    let mut current = Discriminant::Value(0);
    let mut v_discriminants = Vec::new();
    for discriminant in discriminants {
        match discriminant {
            Some(expr) => {
                let next_expr = quote! { #expr + 1 };
                v_discriminants.push(quote! { #expr });
                current = Discriminant::Expr(next_expr);
            }
            None => {
                current = match current {
                    Discriminant::Value(value) => {
                        let next_value = value + 1;
                        v_discriminants.push(quote! { #next_value });
                        Discriminant::Value(next_value)
                    }
                    Discriminant::Expr(ref expr) => {
                        let next_expr = quote! { (#expr) + 1 };
                        v_discriminants.push(quote! { #next_expr });
                        Discriminant::Expr(next_expr)
                    }
                };
            }
        }
    }
    v_discriminants
}
