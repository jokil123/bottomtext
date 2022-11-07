use clone_all::clone_all;
use common::frame::FrameJson;

use web_sys::FocusEvent;
use yew::{function_component, html, use_context, use_state, Callback, UseStateHandle};

use yew::events::*;

use crate::context::AppContext;
use crate::util::value_from_input_event;

#[function_component(Input)]
pub fn frame_input() -> Html {
    let context = use_context::<AppContext>().expect("Failed to get context");
    let text: UseStateHandle<String> = use_state(|| "".to_string());
    let subtext: UseStateHandle<String> = use_state(|| "".to_string());

    let reset = {
        clone_all!(text, subtext);
        move || {
            text.set("".to_string());
            subtext.set("".to_string());
        }
    };

    let onsubmit = {
        clone_all!(text, subtext);
        let submit_cb = context.submit_cb.clone();
        move |e: FocusEvent| {
            e.prevent_default();

            let text = Some((*text).clone()).filter(|x| !x.is_empty());
            let st = Some((*subtext).clone()).filter(|x| !x.is_empty());

            if text.is_none() && st.is_none() {
                return;
            } else if text.is_none() && st.is_some() {
                submit_cb.emit(FrameJson {
                    text: st.unwrap(),
                    subtext: None,
                });
            } else {
                submit_cb.emit(FrameJson {
                    text: text.unwrap(),
                    subtext: st,
                });
            }

            reset();
        }
    };

    html! {
    <form class="flex flex-col items-center font-serif" onsubmit={onsubmit}>
        <input class="input bg-transparent text-center text-2xl" placeholder="Type Something!" type="text"
        value={(*text).clone()}
        oninput={Callback::from(move |e: InputEvent| text.set(value_from_input_event(e).unwrap()))}/>

        <input class="input bg-transparent text-center text-sm" placeholder="(Optional Subtext)" type="text"
        value={(*subtext).clone()}
        oninput={Callback::from(move |e: InputEvent| subtext.set(value_from_input_event(e).unwrap()))}/>

        <input type="submit" />
    </form>
    }
}
