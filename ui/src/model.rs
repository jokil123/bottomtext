use common::frame::{FrameJson, FramesJson};
use gloo_net::http::Request;
use yew::{function_component, html, Properties};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct FrameModel {
    pub frame: FrameJson,
    pub depth: i32,
    pub inner: Option<Box<FrameModel>>,
}

impl FrameModel {
    pub async fn from_request() -> FrameModel {
        let fetched_frames: FramesJson = Request::get("/api/frames")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        fetched_frames.into()
    }

    pub fn push_front(mut self, new: FrameJson) {
        // TODO: unnecessary clone
        let mut fj: FramesJson = self.clone().into();
        fj.frames.insert(0, new);
        let new: FrameModel = fj.into();
        self = new;
        // self.inner = new.inner;
        // self.frame = new.frame;
        // self.depth = new.depth;
    }
}

impl Into<FramesJson> for FrameModel {
    fn into(self) -> FramesJson {
        let mut frames: Vec<FrameJson> = vec![];

        let mut current = Some(self);
        while let Some(c) = current {
            frames.push(c.frame);
            current = c.inner.map(|f| *f);
        }

        FramesJson { frames }
    }
}

impl From<FramesJson> for FrameModel {
    fn from(frames: FramesJson) -> Self {
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
}
