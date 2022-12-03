use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod c2s;
pub mod s2c;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MessageContent {
    pub text: String,
    pub subtext: Option<String>,
}

#[derive(Error, Debug)]
pub enum CastError {
    #[error("websoche message was not a text message")]
    ToStringError,
    #[error("could not deserialize message: {0}")]
    SerializationError(serde_json::Error),
}
