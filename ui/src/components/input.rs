use clone_all::clone_all;
use common::frame::FrameJson;

use web_sys::{Event, FocusEvent};
use yew::{
    function_component, html, use_state, Callback, KeyboardEvent, Properties, UseStateHandle,
};

use yew_set_state_onchange::set_state;

#[function_component(Input)]
pub fn frame_input(props: &InputProps) -> Html {
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
        let submit_cb = props.submit.clone();
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
        <div class={"inputContainer"}>
            <form onsubmit={onsubmit}>
                <input class={"input"} id={"text"} type="text" placeholder={"Text"} value={(*text).clone()} onchange={set_state!(text)}/>
                <input class={"input"} id={"subtext"} type="text" placeholder={"(Optional Subtext)"} value={(*subtext).clone()} onchange={set_state!(subtext)}/>
                <input class={"submit"} type="submit" />
            </form>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct InputProps {
    pub submit: Callback<FrameJson>,
}
