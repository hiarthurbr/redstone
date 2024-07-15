/// A rotation angle in steps of 1/256 of a full turn
pub struct Angle(u8);

impl From<u8> for Angle {
    fn from(value: u8) -> Self {
        Angle(value)
    }
}

impl From<Angle> for u8 {
    fn from(value: Angle) -> Self {
        value.0
    }
}
