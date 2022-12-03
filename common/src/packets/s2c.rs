// use gloo_net::websocket::Message;
use serde::{Deserialize, Serialize};
use warp::ws::Message;

use super::MessageContent;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PacketS2C {
    SendMessage(MessageS2C),
    Rejected(RejectReason),
    Accepted(MessageS2C),
    InitializeUser(InitializationData),
}

impl Into<Message> for PacketS2C {
    fn into(self) -> Message {
        Message::text(serde_json::to_string(&self).unwrap())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MessageS2C {
    pub content: MessageContent,
    pub user_id: u128,
    pub user_country: isocountry::CountryCode,
    pub message_id: u128,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RejectReason {
    DisallowedContent(String),
    CooldownRemaining(u64),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InitializationData {
    pub user_id: u128,
    pub user_country: isocountry::CountryCode,
}
