use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Ident, LitInt};

use crate::utils::default_debug::default_debug;

pub(crate) fn expand_int(
    lit: &LitInt,
    name: &Ident,
) -> Result<TokenStream2, syn::Error> {
    let suffix: &str = lit.suffix();
    let value_str: &str = lit.base10_digits();

    let (ty, serialize_method, schema_type): (syn::Type, &str, syn::Ident) =
        match suffix {
            | "" => {
                let value: i128 = value_str.parse().map_err(|_| {
                    syn::Error::new_spanned(lit, "integer literal out of range")
                })?;
                if value >= i32::MIN as i128 && value <= i32::MAX as i128 {
                    (
                        syn::parse_quote!(i32),
                        "serialize_i32",
                        syn::Ident::new(
                            "Integer",
                            proc_macro2::Span::call_site(),
                        ),
                    )
                } else if value >= i64::MIN as i128 && value <= i64::MAX as i128
                {
                    (
                        syn::parse_quote!(i64),
                        "serialize_i64",
                        syn::Ident::new(
                            "Integer",
                            proc_macro2::Span::call_site(),
                        ),
                    )
                } else {
                    return Err(syn::Error::new_spanned(
                        lit,
                        "integer literal too large for i64; add a suffix like `u128`",
                    ));
                }
            },
            | "u8" => (
                syn::parse_quote!(u8),
                "serialize_u8",
                syn::Ident::new("Integer", proc_macro2::Span::call_site()),
            ),
            | "u16" => (
                syn::parse_quote!(u16),
                "serialize_u16",
                syn::Ident::new("Integer", proc_macro2::Span::call_site()),
            ),
            | "u32" => (
                syn::parse_quote!(u32),
                "serialize_u32",
                syn::Ident::new("Integer", proc_macro2::Span::call_site()),
            ),
            | "u64" => (
                syn::parse_quote!(u64),
                "serialize_u64",
                syn::Ident::new("Integer", proc_macro2::Span::call_site()),
            ),
            | "u128" => (
                syn::parse_quote!(u128),
                "serialize_u128",
                syn::Ident::new("Integer", proc_macro2::Span::call_site()),
            ),
            | "usize" => (
                syn::parse_quote!(usize),
                "serialize_u64",
                syn::Ident::new("Integer", proc_macro2::Span::call_site()),
            ),
            | "i8" => (
                syn::parse_quote!(i8),
                "serialize_i8",
                syn::Ident::new("Integer", proc_macro2::Span::call_site()),
            ),
            | "i16" => (
                syn::parse_quote!(i16),
                "serialize_i16",
                syn::Ident::new("Integer", proc_macro2::Span::call_site()),
            ),
            | "i32" => (
                syn::parse_quote!(i32),
                "serialize_i32",
                syn::Ident::new("Integer", proc_macro2::Span::call_site()),
            ),
            | "i64" => (
                syn::parse_quote!(i64),
                "serialize_i64",
                syn::Ident::new("Integer", proc_macro2::Span::call_site()),
            ),
            | "i128" => (
                syn::parse_quote!(i128),
                "serialize_i128",
                syn::Ident::new("Integer", proc_macro2::Span::call_site()),
            ),
            | "isize" => (
                syn::parse_quote!(isize),
                "serialize_i64",
                syn::Ident::new("Integer", proc_macro2::Span::call_site()),
            ),
            | other => {
                return Err(syn::Error::new_spanned(
                    lit,
                    format!("unsupported integer suffix `{other}`"),
                ));
            },
        };

    let serialize_ident: syn::Ident =
        syn::Ident::new(serialize_method, proc_macro2::Span::call_site());

    let common: TokenStream2 = default_debug(name);

    Ok(quote! {
        impl #name {
            pub const VALUE: #ty = #lit;
        }

        impl ::std::ops::Deref for #name {
            type Target = #ty;

            fn deref(&self) -> &Self::Target {
                &Self::VALUE
            }
        }

        #common

        #[cfg(feature = "serde")]
        impl<'de> ::serde::Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                deserializer
                    .deserialize_any(::literalize::serde::int::MustBeIntVisitor::<#ty> {
                        expected: Self::VALUE,
                    })
                    .map(|()| Self)
            }
        }

        #[cfg(feature = "serde")]
        impl ::serde::Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                serializer.#serialize_ident(**self)
            }
        }

        #[cfg(feature = "utoipa")]
        impl ::utoipa::PartialSchema for #name {
            fn schema() -> ::utoipa::openapi::RefOr<::utoipa::openapi::schema::Schema> {
                ::utoipa::openapi::schema::ObjectBuilder::new()
                    .schema_type(::utoipa::openapi::schema::Type::#schema_type)
                    .enum_values(Some([Self::VALUE]))
                    .build()
                    .into()
            }
        }

        #[cfg(feature = "utoipa")]
        impl ::utoipa::ToSchema for #name {}
    })
}
