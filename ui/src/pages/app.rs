use clone_all::clone_all;
use common::frame;
use common::frame::FrameJson;
use common::frame::FramesJson;
use futures::SinkExt;
use futures::StreamExt;
use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::Message;
use wasm_bindgen_futures::spawn_local;
use yew::callback;
use yew::prelude::*;
use yew_agent::Bridged;

use crate::components::frame::Frame;
use crate::components::frame_input::FrameInput;
use crate::event_bus::EventBus;
use crate::model::FrameModel;
// use crate::ws::WebsocketService;

#[function_component(App)]
pub fn app() -> Html {
    let frame: UseStateHandle<FrameModel> = use_state(|| FrameModel::default());

    {
        clone_all!(frame);
        use_effect_with_deps(
            move |_| {
                // let frame = frame.clone();
                wasm_bindgen_futures::spawn_local({
                    clone_all!(frame);
                    async move {
                        frame.set(FrameModel::from_request().await);
                    }
                });
                || ()
            },
            (),
        );
    }

    {
        clone_all!(frame);
        use_effect_with_deps(
            move |_| {
                let ws = WebSocket::open("ws://127.0.0.1:8080/api/ws").unwrap();
                let (write, mut read) = ws.split();

                spawn_local(async move {
                    while let Some(r) = read.next().await {
                        if let Ok(Message::Text(s)) = r {
                            if let Ok(f) = serde_json::from_str::<FrameJson>(&s) {
                                // log::debug!("got event from websocket! {:#?}", &f);
                                // log::info!("current frame: {:#?}", *frame);

                                // let new_frame = frame.push_front(f);
                                // log::info!("new frame: {:#?}", new_frame.clone());

                                frame.set(*frame);

                                // log::info!("new frame state: {:#?}", *frame);
                            }
                        }
                    }
                });

                || ()
            },
            (),
        );
    }

    html! {
        <>
            <Frame fm={(*frame).clone()} />
            <FrameInput />
        </>
    }
}
