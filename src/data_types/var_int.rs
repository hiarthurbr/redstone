use thiserror::Error;

static SEGMENT_BITS: i32 = 0x7F;
static CONTINUE_BIT: i32 = 0x80;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Error)]
pub enum VarIntError {
    #[error("{0}")]
    DecodeOverflow(&'static str),
    #[error("{0}")]
    EncodeOverflow(&'static str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VarInt(i32);

impl VarInt {
    #[must_use]
    pub fn new(value: i32) -> Self {
        VarInt(value)
    }

    /// Decodes a [`VarInt`] from a given buffer.
    ///
    /// ## Errors
    ///
    /// Returns an error if the [`VarInt`] is too big.
    pub fn decode(buf: &[u8]) -> Result<Self, VarIntError> {
        let mut value = 0;
        let mut position = 0;

        for current_byte in buf {
            value |= (i32::from(*current_byte) & SEGMENT_BITS) << position;

            if (i32::from(*current_byte) & CONTINUE_BIT) == 0 {
                break;
            }

            position += 7;

            if position >= 32 {
                return Err(VarIntError::DecodeOverflow("VarInt is too big"));
            }
        }

        Ok(VarInt(value))
    }

    /// Encodes a [`VarInt`] to a buffer.
    ///
    /// ## Errors
    ///
    /// Returns an error on overflow.
    pub fn encode(&self) -> Result<Vec<u8>, VarIntError> {
        let mut bytes: Vec<u8> = Vec::new();

        let mut int = self.0;

        loop {
            if (int & !SEGMENT_BITS) == 0 {
                bytes.push(
                    int.try_into()
                        .map_err(|_| VarIntError::EncodeOverflow("u8 overflow"))?,
                );

                return Ok(bytes);
            }

            bytes.push(
                ((int & SEGMENT_BITS) | CONTINUE_BIT)
                    .try_into()
                    .map_err(|_| VarIntError::EncodeOverflow("u8 overflow"))?,
            );

            // Perform logical right shift by 7 bits (equivalent to >>>= 7 in other languages)
            int >>= 7; // Perform arithmetic right shift
            int &= !(!0 << (32 - 7)); // Masking to ensure zero-fill behavior
        }
    }

    #[must_use]
    pub fn value(&self) -> i32 {
        self.0
    }
}

#[cfg(test)]
mod test {
    use super::VarInt;

    #[test]
    fn read_0() {
        assert_eq!(VarInt::decode(&[0x00]).unwrap().value(), 0);
    }

    #[test]
    fn write_0() {
        assert_eq!(VarInt(0).encode().unwrap(), vec![0x00]);
    }

    #[test]
    fn read_1() {
        assert_eq!(VarInt::decode(&[0x01]).unwrap().value(), 1);
    }

    #[test]
    fn write_1() {
        assert_eq!(VarInt(1).encode().unwrap(), vec![0x01]);
    }

    #[test]
    fn read_2() {
        assert_eq!(VarInt::decode(&[0x02]).unwrap().value(), 2);
    }

    #[test]
    fn write_2() {
        assert_eq!(VarInt(2).encode().unwrap(), vec![0x02]);
    }

    #[test]
    fn read_127() {
        assert_eq!(VarInt::decode(&[0x7f]).unwrap().value(), 127);
    }

    #[test]
    fn write_127() {
        assert_eq!(VarInt(127).encode().unwrap(), vec![0x7f]);
    }

    #[test]
    fn read_128() {
        assert_eq!(VarInt::decode(&[0x80, 0x01]).unwrap().value(), 128);
    }

    #[test]
    fn write_128() {
        assert_eq!(VarInt(128).encode().unwrap(), vec![0x80, 0x01]);
    }

    #[test]
    fn read_255() {
        assert_eq!(VarInt::decode(&[0xff, 0x01]).unwrap().value(), 255);
    }

    #[test]
    fn write_255() {
        assert_eq!(VarInt(255).encode().unwrap(), vec![0xff, 0x01]);
    }

    #[test]
    fn read_25565() {
        assert_eq!(VarInt::decode(&[0xdd, 0xc7, 0x01]).unwrap().value(), 25565);
    }

    #[test]
    fn write_25565() {
        assert_eq!(VarInt(25565).encode().unwrap(), vec![0xdd, 0xc7, 0x01]);
    }

    #[test]
    fn read_2097151() {
        assert_eq!(
            VarInt::decode(&[0xff, 0xff, 0x7f]).unwrap().value(),
            2_097_151
        );
    }

    #[test]
    fn write_2097151() {
        assert_eq!(VarInt(2_097_151).encode().unwrap(), vec![0xff, 0xff, 0x7f]);
    }

    #[test]
    fn read_2147483647() {
        assert_eq!(
            VarInt::decode(&[0xff, 0xff, 0xff, 0xff, 0x07])
                .unwrap()
                .value(),
            2_147_483_647
        );
    }

    #[test]
    fn write_2147483647() {
        assert_eq!(
            VarInt(2_147_483_647).encode().unwrap(),
            vec![0xff, 0xff, 0xff, 0xff, 0x07]
        );
    }

    #[test]
    fn read_minus_1() {
        assert_eq!(
            VarInt::decode(&[0xff, 0xff, 0xff, 0xff, 0x0f])
                .unwrap()
                .value(),
            -1
        );
    }

    #[test]
    fn write_minus_1() {
        assert_eq!(
            VarInt(-1).encode().unwrap(),
            vec![0xff, 0xff, 0xff, 0xff, 0x0f]
        );
    }

    #[test]
    fn read_minus_2147483648() {
        assert_eq!(
            VarInt::decode(&[0x80, 0x80, 0x80, 0x80, 0x08])
                .unwrap()
                .value(),
            -2_147_483_648
        );
    }

    #[test]
    fn write_minus_2147483648() {
        assert_eq!(
            VarInt(-2_147_483_648).encode().unwrap(),
            vec![0x80, 0x80, 0x80, 0x80, 0x08]
        );
    }
}
