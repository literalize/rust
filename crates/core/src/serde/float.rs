use std::fmt;

use serde::de::{Error, Unexpected, Visitor};

/// Serde visitor for a constant float.
pub struct MustBeFloatVisitor<T> {
    /// Expected constant value.
    pub expected: T,
}

impl<'de, T> Visitor<'de> for MustBeFloatVisitor<T>
where
    T: Copy + PartialEq + fmt::Display + Into<f64>,
{
    type Value = ();

    fn expecting(
        &self,
        formatter: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(formatter, "{}", self.expected)
    }

    fn visit_f64<E>(
        self,
        v: f64,
    ) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let expected: f64 = self.expected.into();
        if v == expected {
            Ok(())
        } else {
            Err(E::invalid_value(Unexpected::Float(v), &self))
        }
    }
}
