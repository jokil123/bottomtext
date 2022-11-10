use gloo::utils::window;
use js_sys::{Array, Reflect};
use wasm_bindgen::JsValue;
use yew::prelude::*;

use crate::js_external::push_ad;

#[function_component(Ad)]
pub fn ad(props: &AdProps) -> Html {
    use_effect_with_deps(
        // move |message| {
        //     log::info!("Ad: {:?}", message);
        //     let ads_by_google =
        //         JsValue::from(window().get("adsbygoogle").expect("adsbygoogle not found"));

        //     match Reflect::get(&ads_by_google, &JsValue::from_str("push")) {
        //         Ok(push) => {
        //             let args = Array::new();
        //             args.push(&JsValue::default());

        //             match Reflect::apply(&push.into(), &ads_by_google, &args) {
        //                 Ok(_) => log::info!("ad pushed"),
        //                 Err(e) => log::error!("adsbygoogle.push failed: {:?}", e),
        //             }
        //         }
        //         Err(err) => log::error!("adsbygoogle.push not found: {:?}", err),
        //     };
        //     return || ()
        // },
        move |_| {
            push_ad();
            return || ();
        },
        (),
    );

    html! {
      <>
        <ins class="adsbygoogle"
            style="display:block"
            data-ad-client="ca-pub-7998438068195946"
            data-ad-slot={props.slot.clone()}
            data-ad-format="auto"
            data-full-width-responsive="true"
            >
        </ins>
        // <script>{"(adsbygoogle = window.adsbygoogle || []).push({});"}</script>
      </>
    }
}

#[derive(Properties, PartialEq)]
pub struct AdProps {
    pub slot: String,
}
