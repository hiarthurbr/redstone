use std::num::TryFromIntError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// A position in the world.
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Position {
    /// Creates a new [`Position`] from the given coordinates.
    ///
    /// ## Errors
    ///
    /// Returns an error if any of the coordinates are out of range.
    pub fn new(x: i32, y: i32, z: i32) -> Result<Position, &'static str> {
        if x > 0b11_1111_1111_1111_1111_1111_1111 {
            return Err("X is out of range");
        }

        if y > 0b1111_1111_1111 {
            return Err("Y is out of range");
        }

        if z > 0b11_1111_1111_1111_1111_1111_1111 {
            return Err("Z is out of range");
        }

        Ok(Position { x, y, z })
    }

    /// Encodes a [`Position`] into a 64-bit integer
    #[must_use]
    pub fn encode(&self) -> i64 {
        let x = (i64::from(self.x) & 0x03FF_FFFF) << 38;
        let y = i64::from(self.y) & 0xFFF;
        let z = (i64::from(self.z) & 0x03FF_FFFF) << 12;

        x | z | y
    }

    /// Decodes a 64-bit integer into a [`Position`], where:
    ///
    /// - The first 26 bits represent the X coordinate.
    /// - The next 26 bits represent the Z coordinate.
    /// - The last 12 bits represent the Y
    ///
    /// ## Errors
    ///
    /// Returns an error if any of the coordinates overflow an `i32`.
    pub fn decode(value: i64) -> Result<Position, TryFromIntError> {
        let x = value >> 38;
        let y = value << 52 >> 52;
        let z = value << 26 >> 38;

        Ok(Position {
            x: x.try_into()?,
            y: y.try_into()?,
            z: z.try_into()?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::Position;

    #[allow(clippy::unusual_byte_groupings)]
    static EXAMPLE_1_BYTES: i64 =
        0b01000110000001110110001100_10110000010101101101001000_001100111111;

    #[test]
    fn encode() {
        assert_eq!(
            Position::new(18_357_644, 831, -20_882_616)
                .unwrap()
                .encode(),
            EXAMPLE_1_BYTES
        );
    }

    #[test]
    fn decode() {
        assert_eq!(
            Position::decode(EXAMPLE_1_BYTES).unwrap(),
            Position::new(18_357_644, 831, -20_882_616).unwrap()
        );
    }

    #[test]
    fn encode_decode() {
        let position = Position::new(18_357_644, 831, -20_882_616).unwrap();
        assert_eq!(Position::decode(position.encode()).unwrap(), position);
    }
}
