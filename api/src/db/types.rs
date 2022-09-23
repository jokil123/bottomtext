use std::clone;

#[derive(Serialize, Deserialize, Clone)]
pub struct FramesJson {
    pub frames: Vec<FrameJson>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FrameJson {
    pub text: String,
    pub subtext: Option<String>,
}
