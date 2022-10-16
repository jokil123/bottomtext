use common::frame::{FrameJson, FramesJson};
use gloo_net::http::Request;
use serde::Deserialize;
use yew::{function_component, html, Properties};

#[derive(Debug, Clone, PartialEq, Properties, Default)]
pub struct FrameModel {
    frame: FrameJson,
    depth: i32,
    inner: Option<Box<FrameModel>>,
}

impl FrameModel {
    fn new(frame_data: Vec<(&str, Option<&str>)>, depth: Option<i32>) -> Option<FrameModel> {
        let depth = match depth {
            Some(d) => d,
            None => 0,
        };

        match frame_data.get(0) {
            Some(d) => Some(FrameModel {
                frame: FrameJson {
                    text: d.0.to_string(),
                    subtext: d.1.map(str::to_string),
                },
                depth: depth,
                inner: match FrameModel::new(frame_data[1..].to_owned(), Some(depth + 1)) {
                    Some(f) => Some(Box::new(f)),
                    None => None,
                },
            }),
            None => None,
        }
    }

    fn from_json(frames: FramesJson) -> FrameModel {
        let mut depth: i32 = frames.frames.len() as i32;

        frames.frames.iter().fold(FrameModel::default(), |fm, fj| {
            depth -= 1;
            FrameModel {
                frame: FrameJson {
                    text: fj.text.clone(),
                    subtext: fj.subtext.clone(),
                },
                depth: depth,
                inner: Some(Box::new(fm)),
            }
        })
    }

    pub async fn from_request() -> FrameModel {
        let fetched_frames: FramesJson = Request::get("/api/frames")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        FrameModel::from_json(fetched_frames)
    }
}

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
