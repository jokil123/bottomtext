use common::frame::FrameJson;
use yew::prelude::*;

#[function_component(Frame)]
pub fn frame(props: &FrameProps) -> Html {
    let current = match props.frames.get(props.depth as usize) {
        Some(f) => f,
        None => return html! {},
    };

    let sf = |p: f64| -> f64 { p.powi(props.depth) };
    let inner_scale = 0.7;
    let scaling_factor = sf(inner_scale);

    html!(
        <div class="frameContainer" style={format!("height: {}%; width: {}%", inner_scale*100., inner_scale*100.)} depth={props.depth.to_string()}>
            <div class="frameBorder" style={format!("border: 1px solid rgba(255, 255, 255, {})", sf(0.85))}>
                {match props.frames.get(props.depth as usize + 1) {
                    Some(_) => html! {
                        <Frame frames={props.frames.clone()} depth={props.depth + 1} />
                    },
                    None => html! {},
                }}
            </div>


            <p class="text" style={format!("font-size: {}vw", 4.0 * scaling_factor)}>{&current.text}</p>

            {match &current.subtext
                {
                Some(subtext) => html! {
                    // <h2 class="text">{subtext}</h2>
                    <p class="text" style={format!("font-size: {}vw", 1.0 * scaling_factor)}>{subtext}</p>
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
