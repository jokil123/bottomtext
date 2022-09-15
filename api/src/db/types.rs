use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use super::{FRAME_DELIMITER, SUBTEXT_DELIMITER};

#[derive(Serialize, Deserialize)]
pub struct FramesJson {
    pub frames: Vec<FrameJson>,
}

#[derive(Serialize, Deserialize)]
pub struct FrameJson {
    pub text: String,
    pub subtext: Option<String>,
}
