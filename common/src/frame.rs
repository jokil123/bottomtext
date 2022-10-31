use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FramesJson {
    pub frames: Vec<FrameJson>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct FrameJson {
    pub text: String,
    pub subtext: Option<String>,
}
