use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Ident, LitBool};

use crate::utils::default_debug::default_debug;

pub(crate) fn expand_bool(
    lit: &LitBool,
    name: &Ident,
) -> Result<TokenStream2, syn::Error> {
    let value: bool = lit.value;

    let common: TokenStream2 = default_debug(name);

    let serde_impl: TokenStream2 = if crate::utils::features::SERDE {
        quote! {
            impl<'de> ::serde::Deserialize<'de> for #name {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: ::serde::Deserializer<'de>,
                {
                    deserializer
                        .deserialize_any(::literalize::serde::bool::MustBeBoolVisitor(Self::VALUE))
                        .map(|()| Self)
                }
            }

            impl ::serde::Serialize for #name {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: ::serde::Serializer,
                {
                    serializer.serialize_bool(**self)
                }
            }
        }
    } else {
        TokenStream2::new()
    };

    let utoipa_impl: TokenStream2 = if crate::utils::features::UTOIPA {
        quote! {
            impl ::utoipa::PartialSchema for #name {
                fn schema() -> ::utoipa::openapi::RefOr<::utoipa::openapi::schema::Schema> {
                    ::utoipa::openapi::schema::ObjectBuilder::new()
                        .schema_type(::utoipa::openapi::schema::Type::Boolean)
                        .enum_values(Some([Self::VALUE]))
                        .build()
                        .into()
                }
            }

            impl ::utoipa::ToSchema for #name {}
        }
    } else {
        TokenStream2::new()
    };

    Ok(quote! {
        impl #name {
            pub const VALUE: bool = #value;
        }

        impl ::std::ops::Deref for #name {
            type Target = bool;

            fn deref(&self) -> &Self::Target {
                &Self::VALUE
            }
        }

        #common

        #serde_impl

        #utoipa_impl
    })
}
