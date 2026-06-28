#[cfg(feature = "serde")]
pub(crate) const SERDE: bool = true;

#[cfg(not(feature = "serde"))]
pub(crate) const SERDE: bool = false;

#[cfg(feature = "utoipa")]
pub(crate) const UTOIPA: bool = true;

#[cfg(not(feature = "utoipa"))]
pub(crate) const UTOIPA: bool = false;
