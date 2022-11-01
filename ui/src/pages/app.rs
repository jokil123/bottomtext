use clone_all::clone_all;
use common::frame::FrameJson;
use gloo::utils::window;
use yew::prelude::*;
use yew_hooks::{use_effect_once, use_list, use_web_socket, UseListHandle};

use crate::components::frame::Frame;
use crate::components::input::Input;
use crate::util::request_frames;

#[function_component(App)]
pub fn app() -> Html {
    let frames: UseListHandle<FrameJson> = use_list(vec![]);
    let ws_url = format!(
        "wss://{}/api/ws",
        window().location().host().expect("error getting host")
    );
    let ws = use_web_socket(ws_url);

    let submit_cb = {
        clone_all!(ws, frames);
        Callback::from(move |frame: FrameJson| {
            ws.send(serde_json::to_string(&frame).unwrap());
            frames.insert(0, frame);
        })
    };

    {
        clone_all!(frames, ws);
        use_effect_with_deps(
            move |message| {
                if let Some(message) = &**message {
                    if let Ok(f) = serde_json::from_str(message) {
                        frames.insert(0, f)
                    }
                }
                || ()
            },
            ws.message,
        );
    }

    {
        clone_all!(frames);
        use_effect_once(move || {
            // let frame = frame.clone();
            wasm_bindgen_futures::spawn_local(async move {
                frames.set(request_frames().await);
            });
            || ()
        })
    };

    html! {
        <>
            <Frame frames={frames.current().clone()} depth={0} />
            <Input submit={submit_cb}/>
        </>
    }
}
