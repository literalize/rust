use std::fmt;

use serde::de::{Error, Unexpected, Visitor};

/// Serde visitor for a constant integer.
pub struct MustBeIntVisitor<T> {
    /// Expected constant value.
    pub expected: T,
}

impl<'de, T> Visitor<'de> for MustBeIntVisitor<T>
where
    T: Copy + PartialEq + fmt::Debug + TryFrom<i64> + TryFrom<u64>,
{
    type Value = ();

    fn expecting(
        &self,
        formatter: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(formatter, "{:?}", self.expected)
    }

    fn visit_i64<E>(
        self,
        v: i64,
    ) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match T::try_from(v) {
            | Ok(parsed) if parsed == self.expected => Ok(()),
            | _ => Err(E::invalid_value(Unexpected::Signed(v), &self)),
        }
    }

    fn visit_u64<E>(
        self,
        v: u64,
    ) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match T::try_from(v) {
            | Ok(parsed) if parsed == self.expected => Ok(()),
            | _ => Err(E::invalid_value(Unexpected::Unsigned(v), &self)),
        }
    }
}
