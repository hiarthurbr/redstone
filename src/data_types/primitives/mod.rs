pub mod angle;
pub mod bitset;
pub mod boolean;
pub mod chat;
pub mod identifier;
pub mod json_chat;
pub mod position;
pub mod rotation;
pub mod slot;
pub mod string;
pub mod var_int;
pub mod var_long;
pub mod x;

use std::fmt::Debug;

pub use angle::*;
pub use bitset::*;
pub use boolean::*;
pub use chat::*;
pub use entity_metadata::*;
pub use identifier::*;
pub use json_chat::*;
pub use position::*;
pub use slot::*;
pub use string::*;
pub use var_int::*;
pub use var_long::*;
pub use x::*;

use thiserror::Error;

/// Data types
///
/// This enum is used to represent all data types in the Minecraft protocol.
///
/// This enum uses the same names as the [unofficial protocol documentation](https://wiki.vg/Protocol).
pub enum DataTypes {
    Boolean(Boolean),
    Byte(i8),
    UnsignedByte(u8),
    Short(i16),
    UnsignedShort(u16),
    Int(i32),
    UnsinedInt(u32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(string::String),
    Chat(Chat),
    JsonChat(JsonChat),
    Identifier(Identifier),
    VarInt(VarInt),
    VarLong(VarLong),
    EntityMetadata(EntityMetadata),
    Slot(Slot),
    NBTTag(quartz_nbt::NbtTag),
    Position(Position),
    Angle(Angle),
    UUID(uuid::Uuid),
    ByteArray(Vec<u8>),
    BitSet(BitSet),
}

/// Generic data types
///
/// This enum is used to represent generic data types.
///
/// Separated from the [`DataTypes`] enum for purely convenience reasons, so you don't have to write the generic type every time.
#[allow(clippy::module_name_repetitions)]
pub enum GenericDataTypes<X> {
    Optional(Optional<X>),
    Array(Array<X>),
    Enum(Enum<X>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Error)]
pub enum Errors {
    #[error("VarIntError: {0}")]
    VarIntError(#[from] VarIntError),
    #[error("VarFloatError: {0}")]
    VarFloatError(#[from] VarLongError),
    #[error("IdentifierError: {0}")]
    IdentifierError(#[from] IdentifierError),
    #[error("StringError: {0}")]
    StringError(#[from] StringError),
    #[error("SlotDataError: {0}")]
    SlotDataError(#[from] SlotDataError),
    #[error("BooleanError: {0}")]
    BooleanError(#[from] BooleanError),
    #[error("PositionError: {0}")]
    PositionError(#[from] PositionError),
}

impl Errors {
    /// Converts a specific error into a generic error.
    #[allow(clippy::missing_errors_doc)]
    pub fn err<T>(error: impl Into<Errors>) -> DataResult<T> {
        Err(error.into())
    }

    /// Maps to a generic error.
    #[allow(clippy::missing_errors_doc)]
    pub fn map<E>(error: impl Into<Errors>) -> impl FnOnce(E) -> Errors {
        |_| error.into()
    }
}

pub trait SerDe<'a> {
    type Input;
    type Serialized;
    type Deserialized;

    #[allow(clippy::missing_errors_doc)]
    fn encode(&self) -> Self::Serialized;
    #[allow(clippy::missing_errors_doc)]
    fn decode(data: Self::Input) -> Self::Deserialized;
}

pub type DataResult<T> = std::result::Result<T, Errors>;
