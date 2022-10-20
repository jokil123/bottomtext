use clone_all::clone_all;
use gloo::events::EventListener;

// use ui::{get_html_element_by_id, value_from_event};
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};
use yew::{function_component, html, use_effect, use_state, KeyboardEvent, UseStateHandle};

use crate::util::{get_html_element_by_id, value_from_event};

#[function_component(FrameInput)]
pub fn frame_input() -> Html {
    let text: UseStateHandle<String> = use_state(|| "".to_string());
    let subtext: UseStateHandle<String> = use_state(|| "".to_string());
    let is_shown: UseStateHandle<bool> = use_state(|| false);

    let onkeypress_handler = move |e| {
        handle_keypress(&e, |e| {
            let value = e
                .target()
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap()
                .value();

            get_html_element_by_id("subtext").map(|e| e.focus());
        })
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
                onkeydown={onkeypress_handler}
                placeholder={"Text"}
                value={(*text).clone()}
                style={if (*is_shown).clone() {"visibility: visible;"} else {"visibility: hidden;"}}
            />
            <input
                class={"input"}
                id={"subtext"}
                onchange={clone_all!(subtext); move |e: Event| {value_from_event(e).map(|e| {subtext.set(e.to_string()); log::info!("{}", e.to_string());});}}
                onkeydown={|e| {handle_keypress(&e, submit)}}
                placeholder={"Optional Subtext"}
                value={(*subtext).clone()}
                style={if *is_shown {"visibility: visible;"} else {"visibility: hidden;"}}
            />
        </div>
    }
}

fn handle_keypress(e: &KeyboardEvent, enter_handler: fn(e: &KeyboardEvent) -> ()) {
    match e.key().as_str() {
        "Enter" => {
            enter_handler(e);
        }
        _ => {}
    }
}

fn submit(e: &KeyboardEvent) {
    log::info!("submitted")
}
