use crate::components::frame::Frame;
use common::frame::FrameJson;
use yew::prelude::*;

#[function_component(Frames)]
pub fn frames(props: &FramesProps) -> Html {
    html! {
        <div class="flex-grow">
            <div class="max-w-full max-h-full">
            </div>
            // flex-grow relative
            // <div class="max-w-full max-h-full aspect-golden-w">
            //     // { for props.frames.clone().into_iter().enumerate().map(|(depth, frame)| html!(<Frame {frame} {depth} />)) }
            // </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct FramesProps {
    pub frames: Vec<FrameJson>,
}
