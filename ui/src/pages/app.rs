use common::frame::FrameJson;
use yew::callback;
use yew::prelude::*;
use yew_agent::Bridged;

use crate::components::frame_input::FrameInput;
use crate::event_bus::EventBus;
use crate::model::Frame;
use crate::model::FrameModel;

#[function_component(App)]
pub fn app() -> Html {
    let frame: UseStateHandle<FrameModel> = use_state(|| FrameModel::default());
    {
        let frame = frame.clone();
        use_effect(move || {
            let frame = frame.clone();
            wasm_bindgen_futures::spawn_local(async move {
                frame.set(FrameModel::from_request().await);
            });
            || ()
        });

        use_effect(move || {
            let a = Callback::from(move |f: FrameJson| {
                frame.set(*(frame.clone()).merge(f));
            });

            EventBus::bridge(move |msg: Frame| {
                frame.set(FrameModel::from_frame(msg));
            });
            || ()
        });
    }

    html! {
        <>
            <Frame fm={(*frame).clone()} />
            <FrameInput />
        </>
    }
}
