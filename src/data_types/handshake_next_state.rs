/// 1 for [Status](https://wiki.vg/Protocol#Status),
/// 2 for [Login](https://wiki.vg/Protocol#Login),
/// 3 for [Transfer](https://wiki.vg/Protocol#Login)
pub enum HandshakeNextState {
    Status,
    Login,
    Transfer,
}
