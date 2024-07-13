use crate::data_types::VarInt;

pub struct Packet {
    /// Length of Packet ID + Data
    length: VarInt,
    packet_id: VarInt,
    data: Vec<u8>,
}

impl Packet {
    #[must_use]
    pub fn is_compressed(&self) -> bool {
        todo!()
    }
}

pub enum State {
    Handshaking,
    Status,
    Login,
    Configuration,
    Play,
}

pub enum BoundTo {
    Client,
    Server,
}
