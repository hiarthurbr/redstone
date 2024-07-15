use thiserror::Error;

use crate::data_types::{DataResult, Errors, SerDe};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Error)]
pub enum BooleanError {
    #[error("Invalid boolean value: {0}")]
    InvalidValue(&'static str),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Boolean(bool);

impl SerDe<'_> for Boolean {
    type Input = u8;
    type Serialized = u8;
    type Deserialized = DataResult<Self>;

    /// Encodes a [`Boolean`] into a [`u8`], where:
    /// - `0x00` represents `false`.
    /// - `0x01` represents `true`.
    fn encode(&self) -> Self::Serialized {
        u8::from(self.0)
    }

    /// Decodes a [`u8`] into a [`Boolean`], where:
    ///
    /// - `0x00` represents `false`.
    /// - `0x01` represents `true`.
    ///
    /// # Errors
    ///
    /// Returns [`BooleanError::InvalidValue`] if the value is not `0x00` or `0x01`.
    fn decode(data: Self::Input) -> Self::Deserialized {
        match data {
            0x00 => Ok(Boolean(false)),
            0x01 => Ok(Boolean(true)),
            _ => Errors::err(BooleanError::InvalidValue("Invalid byte value")),
        }
    }
}

impl From<bool> for Boolean {
    fn from(value: bool) -> Self {
        Boolean(value)
    }
}

impl From<Boolean> for bool {
    fn from(value: Boolean) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode() {
        let boolean = Boolean(true);
        assert_eq!(boolean.encode(), 0x01);
        let boolean = Boolean(false);
        assert_eq!(boolean.encode(), 0x00);
    }

    #[test]
    fn decode() {
        let boolean = Boolean::decode(0x01).unwrap();
        assert_eq!(boolean, Boolean(true));
        let boolean = Boolean::decode(0x00).unwrap();
        assert_eq!(boolean, Boolean(false));
    }
}
