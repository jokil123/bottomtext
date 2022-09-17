use ui::get_html_element_by_id;
use yew::{function_component, html, use_state, Html, KeyboardEvent, UseStateHandle};

#[function_component(FrameInput)]
pub fn frame_input() -> Html {
    let text: UseStateHandle<String> = use_state(|| "".to_string());
    let subtext: UseStateHandle<String> = use_state(|| "".to_string());

    html! {
        <div>
            <input id={"text"} onkeypress={|e| {handle_keypress(e, || {get_html_element_by_id("subtext").unwrap().focus();})}} placeholder={"Text"}/>
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
