use clone_all::clone_all;
use common::frame::FrameJson;
use gloo::events::EventListener;

// use ui::{get_html_element_by_id, value_from_event};
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};
use yew::{
    function_component, html, use_effect, use_state, Callback, KeyboardEvent, Properties,
    UseStateHandle,
};
use yew_hooks::UseWebSocketHandle;

use crate::util::{get_html_element_by_id, value_from_event};

#[function_component(Input)]
pub fn frame_input(props: &InputProps) -> Html {
    let text: UseStateHandle<String> = use_state(|| "".to_string());
    let subtext: UseStateHandle<String> = use_state(|| "".to_string());
    let is_shown: UseStateHandle<bool> = use_state(|| false);

    let reset = {
        clone_all!(text, subtext, is_shown);
        move || {
            text.set("".to_string());
            subtext.set("".to_string());
            is_shown.set(false);
        }
    };

    use_effect({
        clone_all!(is_shown, text);
        move || {
            let document = gloo::utils::document();
            let listener = EventListener::new(&document, "keydown", move |event| {
                // let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();

                if !*is_shown {
                    is_shown.set(true);
                    text.set("".to_string());
                    get_html_element_by_id("text").map(|e| e.focus());
                }
            });
            || drop(listener)
        }
    });

    html! {
        <div class={"inputContainer"}>
            <input
                class={"input"}
                id={"text"}
                onchange={clone_all!(text); move |e: Event| {value_from_event(e).map(|e| {text.set(e.to_string()); log::info!("{}", e.to_string());});}}
                onkeydown={{
                    clone_all!(is_shown);
                    move |e| {
                        if *is_shown && is_enter(&e) {
                            get_html_element_by_id("subtext").map(|e| e.focus());
                        }
                    }
                }}
                placeholder={"Text"}
                value={(*text).clone()}
                style={if (*is_shown).clone() {"visibility: visible;"} else {"visibility: hidden;"}}
            />
            <input
                class={"input"}
                id={"subtext"}
                onchange={clone_all!(subtext); move |e: Event| {value_from_event(e).map(|e| {subtext.set(e.to_string()); log::info!("{}", e.to_string());});}}

                onkeydown={{
                    clone_all!(is_shown, text, subtext);
                    let submit_cb = props.submit.clone();

                    move |e| {
                        if *is_shown && is_enter(&e) {
                            let st = Some((*subtext).clone()).filter(|x| !x.is_empty());
                            submit_cb.emit(FrameJson { text: (*text).clone(), subtext: st});
                            reset();
                        }
                    }
                }}

                placeholder={"Optional Subtext"}
                value={(*subtext).clone()}
                style={if *is_shown {"visibility: visible;"} else {"visibility: hidden;"}}
            />
        </div>
    }
}

fn is_enter(e: &KeyboardEvent) -> bool {
    e.key() == "Enter"
}

#[derive(Properties, PartialEq)]
pub struct InputProps {
    pub submit: Callback<FrameJson>,
}
