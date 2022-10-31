use clone_all::clone_all;
use yew::prelude::*;
use yew_agent::Bridged;
use yew_hooks::{use_effect_once, use_web_socket};

use crate::components::frame::Frame;
use crate::components::frame_input::FrameInput;
use crate::model::FrameModel;

#[function_component(App)]
pub fn app() -> Html {
    let frame: UseStateHandle<FrameModel> = use_state(|| FrameModel::default());

    let ws = use_web_socket("ws://localhost:8080/api/ws".to_string());

    {
        clone_all!(frame, ws);
        use_effect_with_deps(
            move |message| {
                if let Some(message) = &**message {
                    if let Ok(f) = serde_json::from_str(&message) {
                        frame.set(frame.push_front(f));
                    }
                }
                || ()
            },
            ws.message,
        );
    }

    {
        clone_all!(frame);
        use_effect_once(move || {
            // let frame = frame.clone();
            wasm_bindgen_futures::spawn_local(async move {
                frame.set(FrameModel::from_request().await);
            });
            || ()
        })
    };

    html! {
        <>
            <Frame fm={(*frame).clone()} />
            <FrameInput />
        </>
    }
}
