use gloo::utils::window;
use js_sys::{Array, Reflect};
use wasm_bindgen::JsValue;
use yew::prelude::*;

use crate::external_js::push_ad;

#[function_component(Ad)]
pub fn ad(props: &AdProps) -> Html {
    use_effect_with_deps(
        move |_| {
            push_ad();
            return || ();
        },
        (),
    );

    html! {
        <ins class="adsbygoogle"
            style="display:block"
            data-ad-client="ca-pub-7998438068195946"
            data-ad-slot={props.slot.clone()}
            data-ad-format="auto"
            data-full-width-responsive="true"
            >
        </ins>
    }
}

#[derive(Properties, PartialEq)]
pub struct AdProps {
    pub slot: String,
}
