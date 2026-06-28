//! A library to build literal-based singleton types and values.
//!
//! ## Example
//!
//! ```
//! use literalize::literal;
//!
//! #[literal("not_found")]
//! struct NotFoundErrorCode;
//! ```
//!
//! ## Features
//!
//! - `serde` - Implement [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) and [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) traits from [`serde`](https://docs.rs/serde/latest/serde/).
//! - `utoipa` - Implement [`ToSchema`](https://docs.rs/utoipa/latest/utoipa/trait.ToSchema.html) trait from [`utoipa`](https://docs.rs/utoipa/latest/utoipa/).

#[cfg(feature = "serde")]
pub mod serde;

pub use literalize_macros::literal;
