use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Ident, LitFloat};

use crate::utils::default_debug::default_debug;

pub(crate) fn expand_float(
    lit: &LitFloat,
    name: &Ident,
) -> Result<TokenStream2, syn::Error> {
    let suffix: &str = lit.suffix();

    let (ty, serialize_method, schema_type): (syn::Type, &str, syn::Ident) =
        match suffix {
            | "" => (
                syn::parse_quote!(f64),
                "serialize_f64",
                syn::Ident::new("Number", proc_macro2::Span::call_site()),
            ),
            | "f32" => (
                syn::parse_quote!(f32),
                "serialize_f32",
                syn::Ident::new("Number", proc_macro2::Span::call_site()),
            ),
            | "f64" => (
                syn::parse_quote!(f64),
                "serialize_f64",
                syn::Ident::new("Number", proc_macro2::Span::call_site()),
            ),
            | other => {
                return Err(syn::Error::new_spanned(
                    lit,
                    format!("unsupported float suffix `{other}`"),
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
                    .deserialize_any(::literalize::serde::float::MustBeFloatVisitor::<#ty> {
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
