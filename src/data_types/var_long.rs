use thiserror::Error;

use crate::DataResult;

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

    /// Decodes a [`VarLong`] from a given buffer.
    ///
    /// ## Errors
    ///
    /// Returns an error if the [`VarLong`] is too big.
    pub fn decode(buf: &[u8]) -> DataResult<Self> {
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
    pub fn write(&mut self) -> DataResult<Vec<u8>> {
        let mut bytes: Vec<u8> = Vec::new();

        loop {
            if (self.0 & !SEGMENT_BITS) == 0 {
                bytes.push(
                    self.0
                        .try_into()
                        .map_err(|_| VarLongError::EncodeOverflow)?,
                );

                return Ok(bytes);
            }

            bytes.push(
                ((self.0 & SEGMENT_BITS) | CONTINUE_BIT)
                    .try_into()
                    .map_err(|_| VarLongError::EncodeOverflow)?,
            );

            // Perform logical right shift by 7 bits (equivalent to >>>= 7 in other languages)
            self.0 >>= 7; // Perform arithmetic right shift
            self.0 &= !(!0 << (64 - 7)); // Masking to ensure zero-fill behavior
        }
    }

    #[must_use]
    pub fn value(&self) -> i64 {
        self.0
    }
}

#[cfg(test)]
mod test {
    use super::VarLong;

    #[test]
    fn read_0() {
        assert_eq!(VarLong::decode(&[0x00]).unwrap().value(), 0);
    }

    #[test]
    fn write_0() {
        assert_eq!(VarLong(0).write().unwrap(), vec![0x00]);
    }

    #[test]
    fn read_1() {
        assert_eq!(VarLong::decode(&[0x01]).unwrap().value(), 1);
    }

    #[test]
    fn write_1() {
        assert_eq!(VarLong(1).write().unwrap(), vec![0x01]);
    }

    #[test]
    fn read_2() {
        assert_eq!(VarLong::decode(&[0x02]).unwrap().value(), 2);
    }

    #[test]
    fn write_2() {
        assert_eq!(VarLong(2).write().unwrap(), vec![0x02]);
    }

    #[test]
    fn read_127() {
        assert_eq!(VarLong::decode(&[0x7f]).unwrap().value(), 127);
    }

    #[test]
    fn write_127() {
        assert_eq!(VarLong(127).write().unwrap(), vec![0x7f]);
    }

    #[test]
    fn read_128() {
        assert_eq!(VarLong::decode(&[0x80, 0x01]).unwrap().value(), 128);
    }

    #[test]
    fn write_128() {
        assert_eq!(VarLong(128).write().unwrap(), vec![0x80, 0x01]);
    }

    #[test]
    fn read_255() {
        assert_eq!(VarLong::decode(&[0xff, 0x01]).unwrap().value(), 255);
    }

    #[test]
    fn write_255() {
        assert_eq!(VarLong(255).write().unwrap(), vec![0xff, 0x01]);
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
            VarLong(2_147_483_647).write().unwrap(),
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
            VarLong(9_223_372_036_854_775_807).write().unwrap(),
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
            VarLong(-1).write().unwrap(),
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
            VarLong(-2_147_483_648).write().unwrap(),
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
            VarLong(-9_223_372_036_854_775_808).write().unwrap(),
            vec![0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01]
        );
    }
}
