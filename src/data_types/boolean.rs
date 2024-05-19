#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Boolean(pub bool);

impl Boolean {
    #[must_use]
    pub fn encode(&self) -> u8 {
        u8::from(self.0)
    }

    /// Decodes a `u8` into a [`Boolean`], where:
    ///
    /// - `0x00` represents `false`.
    /// - `0x01` represents `true`.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is not `0x00` or `0x01`.
    pub fn decode(value: u8) -> Result<Boolean, &'static str> {
        match value {
            0x00 => Ok(Boolean(false)),
            0x01 => Ok(Boolean(true)),
            _ => Err("Invalid boolean value"),
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
