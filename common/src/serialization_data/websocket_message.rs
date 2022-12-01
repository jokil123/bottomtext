use isocountry;

use super::frame::Frame;

pub struct OutgoingFrame {
    pub user_id: usize,
    pub user_country: String,
    pub frame: Frame,
}

pub struct IncomingFrame {
    pub frame: Frame,
}

pub enum WebSocketMessage {
    OutgoingFrame(OutgoingFrame),
    IncomingFrame(IncomingFrame),
}
