use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct Frame {
    pub text: String,
    pub subtext: Option<String>,
}
