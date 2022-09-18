use ui::get_html_element_by_id;
use web_sys::Event;
use yew::{function_component, html, use_state, Html, KeyboardEvent, UseStateHandle};

#[function_component(FrameInput)]
pub fn frame_input() -> Html {
    let text: UseStateHandle<String> = use_state(|| "".to_string());
    let subtext: UseStateHandle<String> = use_state(|| "".to_string());

    let onkeypress_handler = |e| {
        handle_keypress(e, || {
            get_html_element_by_id("subtext").unwrap().focus();
        })
    };

    html! {
        <div>
            <input id={"text"} onchange={|e: Event| {e.target().unwrap();}} onkeypress={onkeypress_handler} placeholder={"Text"} value={(*text).clone()}/>
            <input id={"subtext"} onkeypress={|e| {handle_keypress(e, submit)}} placeholder={"Optional Subtext"}/>
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
