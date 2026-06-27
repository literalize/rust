use std::fmt;

use serde::de::{Error, Unexpected, Visitor};

/// Serde visitor for a constant boolean.
pub struct MustBeBoolVisitor(pub bool);

impl<'de> Visitor<'de> for MustBeBoolVisitor {
    type Value = ();

    fn expecting(
        &self,
        formatter: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(formatter, "{:?}", self.0)
    }

    fn visit_bool<E>(
        self,
        v: bool,
    ) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if v == self.0 {
            Ok(())
        } else {
            Err(E::invalid_value(Unexpected::Bool(v), &self))
        }
    }
}
