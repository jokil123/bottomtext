use clone_all::clone_all;
use js_sys::Function;
use std::sync::Arc;

use ui::{get_html_element_by_id, value_from_event};
use web_sys::Event;
use yew::{function_component, html, use_state, KeyboardEvent, UseStateHandle};

#[function_component(FrameInput)]
pub fn frame_input() -> Html {
    let text: UseStateHandle<String> = use_state(|| "".to_string());
    let subtext: UseStateHandle<String> = use_state(|| "".to_string());
    let is_shown: UseStateHandle<bool> = use_state(|| false);

    let onkeypress_handler = move |e| {
        handle_keypress(e, || {
            get_html_element_by_id("subtext").map(|e| e.focus());
        })
    };

    web_sys::window()
        .unwrap()
        // .document()
        // .unwrap()
        .add_event_listener_with_callback(
            "keypress",
            ((|| log::info!("test")).as_ref().unchecked_ref()),
        );
    // .add_event_listener_with_callback("keypress", {
    //     clone_all!(is_shown);
    //     move |_| {
    //         if !*is_shown {
    //             is_shown.set(true)
    //         }
    //     }
    // });

    html! {
        <div>
            <input
                id={"text"}
                onchange={clone_all!(text); move |e: Event| {value_from_event(e).map(|e| {text.set(e.to_string()); log::info!("{}", e.to_string());});}}
                onkeydown={onkeypress_handler}
                placeholder={"Text"}
                value={(*text).clone()}
                style={if *is_shown {"visibility: visible;"} else {"visibility: hidden;"}}
            />
            <input
                id={"subtext"}
                onchange={clone_all!(subtext); move |e: Event| {value_from_event(e).map(|e| {subtext.set(e.to_string()); log::info!("{}", e.to_string());});}}
                onkeydown={|e| {handle_keypress(e, submit)}}
                placeholder={"Optional Subtext"}
                value={(*subtext).clone()}
                style={if *is_shown {"visibility: visible;"} else {"visibility: hidden;"}}
            />
        </div>
    }
}

fn handle_keypress(e: KeyboardEvent, enter_handler: fn() -> ()) {
    match e.key().as_str() {
        "Enter" => {
            enter_handler();
        }
        _ => {}
    }
}

fn submit() {}
