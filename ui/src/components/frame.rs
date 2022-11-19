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
                style={format!("padding: {}%; border: 1px solid rgba(255, 255, 255, {})", s * 4., sf(0.9))}
            >
                    {
                        match props.depth > props.frames.len() {
                            true => html!(),
                            false => html!(<Frame frames={props.frames.clone()} depth={props.depth + 1}/>),
                        }
                    }
                <div class="flex flex-col font-serif text-center">
                    <div style={format!("font-size: {}vw", sf(0.8)*3.)}>{current.text}</div>
                    <div style={format!("font-size: {}vw", sf(0.8)*1.5)}><p></p></div>
                </div>
            </div>
        </AspectContent>
        // <div
        //     style={format!("height: {}%; width: {}%", s*100., s*100.)}
        //     class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 flex flex-col justify-between items-center"
        // >
        //     <div class="border border-green-400 aspect-golden-w flex-grow max-h-full max-w-full"></div>
        //     <div style={format!("font-size: {}rem", s*2.)} class="flex flex-col font-serif">
        //         <div>{"What?"}</div>
        //         <div>{"How?"}</div>
        //     </div>
        // </div>
        // <div class="frameContainer" style={format!("height: {}%; width: {}%", inner_scale*100., inner_scale*100.)} depth={props.depth.to_string()}>
        //     <div class="frameBorder" style={format!("border: 1px solid rgba(255, 255, 255, {})", sf(0.85))}>
        //         {match props.frames.get(props.depth as usize + 1) {
        //             Some(_) => html! {
        //                 <Frame frames={props.frames.clone()} depth={props.depth + 1} />
        //             },
        //             None => html! {},
        //         }}
        //     </div>


        //     <p class="text" style={format!("font-size: {}vw", 4.0 * scaling_factor)}>{&current.text}</p>

        //     {match &current.subtext
        //         {
        //         Some(subtext) => html! {
        //             // <h2 class="text">{subtext}</h2>
        //             <p class="text" style={format!("font-size: {}vw", 1.0 * scaling_factor)}>{subtext}</p>
        //         },
        //         None => html! {},
        //     }}
        // </div>
    )
}

#[derive(Properties, PartialEq)]
pub struct FrameProps {
    pub frames: Vec<FrameJson>,
    pub depth: usize,
}
