#[derive(Serialize, Deserialize)]
pub struct FramesJson {
    pub frames: Vec<FrameJson>,
}

#[derive(Serialize, Deserialize)]
pub struct FrameJson {
    pub text: String,
    pub subtext: Option<String>,
}
