use common::frame::FrameJson;
use yew::Callback;

#[derive(Clone, Debug, PartialEq)]
pub struct AppContext {
    pub submit_cb: Callback<FrameJson>,
    pub frames: Vec<FrameJson>,
}
