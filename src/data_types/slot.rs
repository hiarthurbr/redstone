use thiserror::Error;

use crate::{data_types::var_int::VarInt, Boolean, DataResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Error)]
pub enum SlotDataError {}

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
    /// Encodes a [`Slot`] into a [`Vec<u8>`].
    ///
    /// # Errors
    ///
    /// Returns [`VarIntError::EncodeOverflow`] if the [`VarInt`] is too big.
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
}
