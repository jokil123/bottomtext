use common::frame::FrameJson;
use yew::prelude::*;

use crate::components::aspect_content::AspectContent;

#[function_component(Frame)]
pub fn frame(props: &FrameProps) -> Html {
    let current = match props.frames.get(props.depth as usize) {
        Some(f) => f.clone(),
        // None => return html! {},
        None => FrameJson::default(),
    };

    let sf = |p: f64| -> f64 { p.powi(props.depth as i32) };
    let inner_scale = 0.7;
    let s = sf(inner_scale);

    html!(
        <AspectContent ratio={1.61803398875}>
            <div
                class={"w-full h-full border flex flex-col items-center justify-around"}
                style={format!("padding: {}%; border: 1px solid rgba(48, 52, 54, {})", s * 4., sf(0.85))}
            >
                    {
                        match props.depth > props.frames.len() {
                            true => html!(),
                            false => html!(<Frame frames={props.frames.clone()} depth={props.depth + 1}/>),
                        }
                    }
                <div class="flex flex-col font-serif text-center">
                    <div style={format!("font-size: {}vw", sf(0.80)*3.)}>{current.text}</div>
                    <div style={format!("font-size: {}vw", sf(0.80)*1.5)}><p></p></div>
                </div>
            </div>
        </AspectContent>
    )
}

#[derive(Properties, PartialEq)]
pub struct FrameProps {
    pub frames: Vec<FrameJson>,
    pub depth: usize,
}
