use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Ident;

/// Emits the `Default` and `Debug` impls shared by every literal kind.
///
/// These bodies are byte-identical across `str`, `int`, `float`, and `bool`;
/// only `Deref` (in `expand::<kind>`) varies because the `Target` type and the
/// dereferencing expression differ per kind.
pub(crate) fn default_debug(name: &Ident) -> TokenStream2 {
    quote! {
        impl ::std::default::Default for #name {
            fn default() -> Self {
                Self
            }
        }

        impl ::std::fmt::Debug for #name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                ::std::fmt::Debug::fmt(&**self, f)
            }
        }
    }
}
