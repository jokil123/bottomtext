use model::FrameModel;
use wasm_bindgen::prelude::wasm_bindgen;
use yew::html;
use yew::prelude::*;

mod frame_input;
mod model;
mod util;
mod ws;

use crate::frame_input::FrameInput;
use crate::model::Frame;

#[function_component(App)]
fn app() -> Html {
    let frame: UseStateHandle<FrameModel> = use_state(|| FrameModel::default());
    {
        let frame = frame.clone();
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

    html! {
        <>
            <Frame fm={(*frame).clone()} />
            <FrameInput />
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
