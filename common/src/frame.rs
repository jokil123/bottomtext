use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use yew::{function_component, html, Properties};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FramesJson {
    pub frames: Vec<FrameJson>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct FrameJson {
    pub text: String,
    pub subtext: Option<String>,
}
