use thiserror::Error;

use crate::{data_types::var_int::VarInt, Boolean, DataResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Error)]
pub enum SlotDataError {
    #[error("Invalid boolean value")]
    InvalidBoolean,
}

pub struct Slot {
    pub data: Option<SlotData>,
}

#[allow(clippy::module_name_repetitions)]
pub struct SlotData {
    pub item_id: VarInt,
    pub item_count: u8,
    pub nbt: quartz_nbt::NbtTag,
}

impl Slot {
    pub fn encode(&self) -> DataResult<Vec<u8>> {
        match &self.data {
            Some(data) => {
                let mut bytes = Vec::new();
                bytes.extend_from_slice(data.item_id.encode()?.as_slice());
                bytes.push(data.item_count);
                // bytes.extend_from_slice(&data.nbt.into());
                Ok(bytes)
            }
            None => Ok(vec![Boolean(false).encode()]),
        }
    }

    /// Decodes a `u8` into a [`Boolean`], where:
    ///
    /// - `0x00` represents `false`.
    /// - `0x01` represents `true`.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is not `0x00` or `0x01`.
    pub fn decode(value: u8) -> DataResult<Boolean> {
        match value {
            0x00 => Ok(Boolean(false)),
            0x01 => Ok(Boolean(true)),
            _ => Err(SlotDataError::InvalidBoolean)?,
        }
    }
}
