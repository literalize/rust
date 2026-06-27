mod bool;
mod float;
mod int;
mod str;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Error, Fields, ItemStruct, Lit};

use crate::expand::bool::expand_bool;
use crate::expand::float::expand_float;
use crate::expand::int::expand_int;
use crate::expand::str::expand_str;

/// Expand the `#[literal(..)]` attribute on `item`.
///
/// Caller (`lib::literal`) has already parsed `attr` into a [`Lit`] and `item`
/// into an [`ItemStruct`]; this fn validates shape, dispatches to the
/// kind-specific expander, and wraps the generated impls around the original
/// struct definition (preserving `vis`, `attrs`, and ident).
pub(crate) fn expand(
    lit: &Lit,
    item: &ItemStruct,
) -> Result<TokenStream2, Error> {
    let vis: &syn::Visibility = &item.vis;
    let name: &syn::Ident = &item.ident;
    let attrs: &[syn::Attribute] = &item.attrs;

    if !matches!(item.fields, Fields::Unit) {
        return Err(Error::new_spanned(
            &item.fields,
            "`literal` can only be applied to unit structs",
        ));
    }

    let value_impl: TokenStream2 = match lit {
        | Lit::Str(l) => expand_str(l, name)?,
        | Lit::Int(l) => expand_int(l, name)?,
        | Lit::Float(l) => expand_float(l, name)?,
        | Lit::Bool(l) => expand_bool(l, name)?,
        | other => {
            return Err(Error::new_spanned(
                other,
                "`literal` accepts only string, integer, float, or boolean literals",
            ));
        },
    };

    Ok(quote! {
        #[derive(::core::cmp::Eq, ::core::cmp::PartialEq)]
        #(#attrs)*
        #vis struct #name;

        #value_impl
    })
}
