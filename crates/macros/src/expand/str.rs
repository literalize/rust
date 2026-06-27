use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Ident, LitStr};

use crate::utils::default_debug::default_debug;

pub(crate) fn expand_str(
    lit: &LitStr,
    name: &Ident,
) -> Result<TokenStream2, syn::Error> {
    let value: String = lit.value();

    let common: TokenStream2 = default_debug(name);

    Ok(quote! {
        impl #name {
            pub const VALUE: &'static str = #value;
        }

        impl ::std::ops::Deref for #name {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                Self::VALUE
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
                    .deserialize_any(::literalize::serde::str::MustBeStrVisitor(Self::VALUE))
                    .map(|()| Self)
            }
        }

        #[cfg(feature = "serde")]
        impl ::serde::Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                serializer.serialize_str(Self::VALUE)
            }
        }

        #[cfg(feature = "utoipa")]
        impl ::utoipa::PartialSchema for #name {
            fn schema() -> ::utoipa::openapi::RefOr<::utoipa::openapi::schema::Schema> {
                ::utoipa::openapi::schema::ObjectBuilder::new()
                    .schema_type(::utoipa::openapi::schema::Type::String)
                    .enum_values(Some([Self::VALUE]))
                    .build()
                    .into()
            }
        }

        #[cfg(feature = "utoipa")]
        impl ::utoipa::ToSchema for #name {}
    })
}
