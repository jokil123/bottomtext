use common::frame::FrameJson;
use yew::prelude::*;

#[function_component(Frame)]
pub fn frame(props: &FrameProps) -> Html {
    let current = match props.frames.get(props.depth as usize) {
        Some(f) => f,
        None => return html! {},
    };

    html!(
        <div class="frameContainer" depth={props.depth.to_string()}>
            <div class="frameBorder">
                {match props.frames.get(props.depth as usize + 1) {
                    Some(inner) => html! {
                        <Frame frames={props.frames.clone()} depth={props.depth + 1} />
                    },
                    None => html! {},
                }}
            </div>


            <h1 class="text">{current.text.clone()}</h1>

            {match current.subtext.clone() {
                Some(subtext) => html! {
                    <h2 class="text">{subtext}</h2>
                },
                None => html! {},
            }}
        </div>
    )
}

#[derive(Properties, PartialEq)]
pub struct FrameProps {
    pub frames: Vec<FrameJson>,
    pub depth: i32,
}
