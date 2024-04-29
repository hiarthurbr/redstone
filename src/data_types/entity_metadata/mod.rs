pub struct EntityMetadata;
use crate::{VarInt, VarLong, String, Chat, Slot, Position};

#[allow(clippy::module_name_repetitions)]
pub enum EntityMetadataType {
  Byte(i8),
  VarInt(VarInt),
  VarLong(VarLong),
  Float(f32),
  String(String),
  Chat(Chat),
  OptChat(Option<Chat>),
  Slot(Slot),
  Boolean(bool),
  Rotation,
  Position(Position),
  OptPosition(Option<Position>),
  Direction,
  OptUuid,
  OptBlockId,
  NbtTag,
  Particle,
  VillagerData,
  OptVarInt,
  Pose,
  OptAngle,
  BlockI
}