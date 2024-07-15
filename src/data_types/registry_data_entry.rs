use super::Identifier;

pub struct Entry {
    id: Identifier,
    data: Option<quartz_nbt::NbtCompound>,
}
