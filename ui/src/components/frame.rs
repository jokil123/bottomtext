use yew::prelude::*;

use crate::model::FrameModel;

#[function_component(Frame)]
pub fn frame(props: &FrameProps) -> Html {
    html!(
        <div class="frameContainer" depth={props.fm.depth.to_string()}>
            <div class="frameBorder">
                {match &props.fm.inner {
                    Some(inner) => html! {
                        <Frame fm={inner.as_ref().clone()} />
                    },
                    None => html! {},
                }}
            </div>


            <h1 class="text">{&props.fm.frame.text}</h1>

            {match &props.fm.frame.subtext {
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
    pub fm: FrameModel,
}
