use crate::{data_types::var_int::VarInt, Boolean};

pub struct Slot {
  pub data: Option<SlotData>
}

#[allow(clippy::module_name_repetitions)]
pub struct SlotData {
  pub item_id: VarInt,
  pub item_count: u8,
  pub nbt: quartz_nbt::NbtTag,
}

impl Slot {
  #[must_use] pub fn encode(&self) -> Vec<u8> {
    match &self.data {
      Some(data) => {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(data.item_id.clone().encode().unwrap().as_slice());
        bytes.push(data.item_count);
        // bytes.extend_from_slice(&data.nbt.into());
        bytes
      },
      None => {
        vec![Boolean(false).encode()]
      }
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
  pub fn decode(value: u8) -> Result<Boolean, &'static str> {
    match value {
      0x00 => Ok(Boolean(false)),
      0x01 => Ok(Boolean(true)),
      _ => Err("Invalid boolean value"),
    }
  }
}