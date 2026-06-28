mod expand;
mod utils;

use proc_macro::TokenStream;
use syn::{ItemStruct, Lit, parse_macro_input};

/// Build a literal-based singleton type and value.
///
/// ## Example
///
/// ```rust,ignore
/// use literalize::literal;
///
/// #[literal("not_found")]
/// struct NotFound;
/// ```
#[proc_macro_attribute]
pub fn literal(
    attr: TokenStream,
    item: TokenStream,
) -> TokenStream {
    let attr_lit: Lit = match syn::parse2(attr.into()) {
        | Ok(lit) => lit,
        | Err(err) => return err.to_compile_error().into(),
    };

    let item_struct: ItemStruct = parse_macro_input!(item as ItemStruct);

    match expand::expand(&attr_lit, &item_struct) {
        | Ok(tokens) => tokens.into(),
        | Err(err) => err.to_compile_error().into(),
    }
}
