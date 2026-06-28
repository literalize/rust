//! A library to build literal-based singleton types and values.
//!
//! ## Usage
//!
//! Apply the `literal` attribute to a struct with a string, integer, float, or boolean literal.
//!
//! ```rust
//! use literalize::literal;
//!
//! #[literal("not_found")]
//! struct NotFound;
//!
//! #[literal(404)]
//! struct HttpNotFound;
//!
//! #[literal(3.14)]
//! struct Pi;
//!
//! #[literal(true)]
//! struct LiteralTrue;
//! ```
//!
//! ## Features
//!
//! - `serde` - Implement `Deserialize` and `Serialize` from [`serde`](https://crates.io/crates/serde).
//! - `utoipa` - Implement `ToSchema` from [`utoipa`](https://crates.io/crates/utoipa).
//! - `all` - Enable all features.

#[cfg(feature = "serde")]
pub mod serde;

pub use literalize_macros::literal;
