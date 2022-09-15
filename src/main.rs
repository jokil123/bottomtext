use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
struct FrameModel {
    text: String,
    subtext: Option<String>,
    inner: Option<Box<FrameModel>>,
}

#[function_component(App)]
fn app() -> Html {
    let frame = FrameModel {
        text: "WHAT".to_string(),
        subtext: Some("HOW".to_string()),
        inner: Some(Box::new(FrameModel {
            text: "WHAT".to_string(),
            subtext: Some("HOW".to_string()),
            inner: None,
        })),
    };

    html! {
        <Frame frame={frame} />
    }
}

#[function_component(Frame)]
fn frame(props: &FrameProps) -> Html {
    html!(
        <div class="frameContainer">
            <div class="frameBorder">
                {match &props.frame.inner {
                    Some(inner) => html! {
                        <Frame frame={inner.as_ref().clone()} />
                    },
                    None => html! {},
                }}
            </div>


            <h1 class="text">{&props.frame.text}</h1>

            {match &props.frame.subtext {
                Some(subtext) => html! {
                    <h2 class="text">{subtext}</h2>
                },
                None => html! {},
            }}
        </div>
    )
}

#[derive(Properties, PartialEq)]
struct FrameProps {
    frame: FrameModel,
}

fn main() {
    yew::start_app::<App>();
}
