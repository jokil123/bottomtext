use clone_all::clone_all;
use common::frame::FrameJson;
use common::frame::FramesJson;
use yew::callback;
use yew::prelude::*;
use yew_agent::Bridged;

use crate::components::frame::Frame;
use crate::components::frame_input::FrameInput;
use crate::event_bus::EventBus;
use crate::model::FrameModel;
use crate::ws::WebsocketService;

#[function_component(App)]
pub fn app() -> Html {
    let wss = use_ref(|| WebsocketService::new());
    let _producer = use_ref(|| {
        EventBus::bridge(Callback::from(move |f: FrameJson| {
            log::info!("eventbus callback");
            frame.set((*frame).clone().push_front(f));
        }))
    });

    // let wss: WebsocketService = WebsocketService::new();

    let frame: UseStateHandle<FrameModel> = use_state(|| FrameModel::default());
    {
        clone_all!(frame);
        use_effect_with_deps(
            move |_| {
                let frame = frame.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    frame.set(FrameModel::from_request().await);
                });
                || ()
            },
            (),
        );
    }

    {
        clone_all!(frame);
        use_effect_with_deps(
            {
                move |_| {
                    EventBus::bridge(Callback::from(move |f: FrameJson| {
                        log::info!("eventbus callback");
                        frame.set((*frame).clone().push_front(f));
                    }));
                    || ()
                }
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
