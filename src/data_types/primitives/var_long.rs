use thiserror::Error;

use crate::data_types::{DataResult, SerDe};

static SEGMENT_BITS: i64 = 0x7F;
static CONTINUE_BIT: i64 = 0x80;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Error)]
pub enum VarLongError {
    #[error("Could not fit buffer into VarLong")]
    DecodeOverflow,
    #[error("Overflow while encoding VarLong")]
    EncodeOverflow,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct VarLong(i64);

impl VarLong {
    #[must_use]
    pub fn new(value: i64) -> Self {
        VarLong(value)
    }

    #[must_use]
    pub fn value(&self) -> i64 {
        self.0
    }
}

impl<'a> SerDe<'a> for VarLong {
    type Input = &'a [u8];
    type Serialized = DataResult<Vec<u8>>;
    type Deserialized = DataResult<Self>;

    /// Decodes a [`VarLong`] from a given buffer.
    ///
    /// ## Errors
    ///
    /// Returns an error if the [`VarLong`] is too big.
    fn decode(buf: Self::Input) -> Self::Deserialized {
        let mut int = 0;
        let mut position: i32 = 0;

        for current_byte in buf {
            int |= (i64::from(*current_byte) & SEGMENT_BITS) << position;

            if (i64::from(*current_byte) & CONTINUE_BIT) == 0 {
                break;
            }

            position += 7;

            if position >= 64 {
                return Err(VarLongError::DecodeOverflow)?;
            }
        }

        Ok(Self(int))
    }

    /// Encodes a [`VarLong`] to a buffer.
    ///
    /// ## Errors
    ///
    /// Returns an error on overflow.
    fn encode(&self) -> Self::Serialized {
        let mut bytes: Vec<u8> = Vec::new();

        let mut int = self.0;

        loop {
            if (int & !SEGMENT_BITS) == 0 {
                bytes.push(int.try_into().map_err(|_| VarLongError::EncodeOverflow)?);

                return Ok(bytes);
            }

            bytes.push(
                ((int & SEGMENT_BITS) | CONTINUE_BIT)
                    .try_into()
                    .map_err(|_| VarLongError::EncodeOverflow)?,
            );

            // Perform logical right shift by 7 bits (equivalent to >>>= 7 in other languages)
            int >>= 7; // Perform arithmetic right shift
            int &= !(!0 << (64 - 7)); // Masking to ensure zero-fill behavior
        }
    }
}

#[cfg(test)]
mod test {
    use crate::data_types::SerDe;

    use super::VarLong;

    #[test]
    fn read_0() {
        assert_eq!(VarLong::decode(&[0x00]).unwrap().value(), 0);
    }

    #[test]
    fn write_0() {
        assert_eq!(VarLong(0).encode().unwrap(), vec![0x00]);
    }

    #[test]
    fn read_1() {
        assert_eq!(VarLong::decode(&[0x01]).unwrap().value(), 1);
    }

    #[test]
    fn write_1() {
        assert_eq!(VarLong(1).encode().unwrap(), vec![0x01]);
    }

    #[test]
    fn read_2() {
        assert_eq!(VarLong::decode(&[0x02]).unwrap().value(), 2);
    }

    #[test]
    fn write_2() {
        assert_eq!(VarLong(2).encode().unwrap(), vec![0x02]);
    }

    #[test]
    fn read_127() {
        assert_eq!(VarLong::decode(&[0x7f]).unwrap().value(), 127);
    }

    #[test]
    fn write_127() {
        assert_eq!(VarLong(127).encode().unwrap(), vec![0x7f]);
    }

    #[test]
    fn read_128() {
        assert_eq!(VarLong::decode(&[0x80, 0x01]).unwrap().value(), 128);
    }

    #[test]
    fn write_128() {
        assert_eq!(VarLong(128).encode().unwrap(), vec![0x80, 0x01]);
    }

    #[test]
    fn read_255() {
        assert_eq!(VarLong::decode(&[0xff, 0x01]).unwrap().value(), 255);
    }

    #[test]
    fn write_255() {
        assert_eq!(VarLong(255).encode().unwrap(), vec![0xff, 0x01]);
    }

    #[test]
    fn read_2147483647() {
        assert_eq!(
            VarLong::decode(&[0xff, 0xff, 0xff, 0xff, 0x07])
                .unwrap()
                .value(),
            2_147_483_647
        );
    }

    #[test]
    fn write_2147483647() {
        assert_eq!(
            VarLong(2_147_483_647).encode().unwrap(),
            vec![0xff, 0xff, 0xff, 0xff, 0x07]
        );
    }

    #[test]
    fn read_9223372036854775807() {
        assert_eq!(
            VarLong::decode(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f])
                .unwrap()
                .value(),
            9_223_372_036_854_775_807
        );
    }

    #[test]
    fn write_9223372036854775807() {
        assert_eq!(
            VarLong(9_223_372_036_854_775_807).encode().unwrap(),
            vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f]
        );
    }

    #[test]
    fn read_minus_1() {
        assert_eq!(
            VarLong::decode(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01])
                .unwrap()
                .value(),
            -1
        );
    }

    #[test]
    fn write_minus_1() {
        assert_eq!(
            VarLong(-1).encode().unwrap(),
            vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01]
        );
    }

    #[test]
    fn read_minus_2147483648() {
        assert_eq!(
            VarLong::decode(&[0x80, 0x80, 0x80, 0x80, 0xf8, 0xff, 0xff, 0xff, 0xff, 0x01])
                .unwrap()
                .value(),
            -2_147_483_648
        );
    }

    #[test]
    fn write_minus_2147483648() {
        assert_eq!(
            VarLong(-2_147_483_648).encode().unwrap(),
            vec![0x80, 0x80, 0x80, 0x80, 0xf8, 0xff, 0xff, 0xff, 0xff, 0x01]
        );
    }

    #[test]
    fn read_minus_9223372036854775808() {
        assert_eq!(
            VarLong::decode(&[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01])
                .unwrap()
                .value(),
            -9_223_372_036_854_775_808
        );
    }

    #[test]
    fn write_minus_9223372036854775808() {
        assert_eq!(
            VarLong(-9_223_372_036_854_775_808).encode().unwrap(),
            vec![0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01]
        );
    }
}
