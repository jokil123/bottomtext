use serde::{Deserialize, Serialize};
use warp::ws::Message;

use super::{CastError, MessageContent};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PacketC2S {
    SendMessage(MessageC2S),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MessageC2S {
    pub content: MessageContent,
}

impl TryFrom<Message> for PacketC2S {
    type Error = CastError;

    fn try_from(msg: Message) -> Result<Self, Self::Error> {
        let s = msg.to_str().map_err(|_| CastError::ToStringError)?;
        serde_json::from_str(s).map_err(|e| CastError::SerializationError(e))
    }
}
