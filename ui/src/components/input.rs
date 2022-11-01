use anyhow;
use clone_all::clone_all;
use common::frame::FrameJson;
use gloo::events::EventListener;

use web_sys::Event;
use yew::{
    function_component, html, use_effect, use_state, Callback, KeyboardEvent, Properties,
    UseStateHandle,
};

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
            let listener = EventListener::new(&document, "keydown", move |_| {
                if !*is_shown {
                    is_shown.set(true);
                    text.set("".to_string());

                    match get_html_element_by_id("text").map(|el| el.focus()) {
                        Ok(Ok(_)) => (),
                        Ok(Err(e)) => log::error!("error focusing on text: {:#?}", e),
                        Err(e) => log::error!("error getting text element: {:#?}", e),
                    };
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
                onchange={
                    clone_all!(text);
                    move |e: Event| {
                        if let Ok(event) = value_from_event(e) {
                            text.set(event.to_string());
                        }
                    }
                }
                onkeydown={
                    clone_all!(is_shown);
                    move |e| {
                        if *is_shown && is_enter(&e) {
                            if let Err(e) = || -> anyhow::Result<()> {
                                get_html_element_by_id("text")?.focus().map_err(|_| anyhow::anyhow!("could not focus"))?;
                                Ok(())
                            }() {
                                log::error!("error focusing on text: {:#?}", e);
                            }
                        }
                    }
                }
                placeholder={"Text"}
                value={(*text).clone()}
                style={if *is_shown {"visibility: visible;"} else {"visibility: hidden;"}}
            />
            <input
                class={"input"}
                id={"subtext"}
                onchange={
                    clone_all!(subtext);
                    move |e: Event| {
                        if let Ok(event) = value_from_event(e) {
                            subtext.set(event.to_string());
                            // log::info!("{}", e.to_string());
                        }
                    }
                }

                onkeydown={
                    clone_all!(is_shown, text, subtext);
                    let submit_cb = props.submit.clone();

                    move |e| {
                        if *is_shown && is_enter(&e) {
                            let st = Some((*subtext).clone()).filter(|x| !x.is_empty());
                            submit_cb.emit(FrameJson { text: (*text).clone(), subtext: st});
                            reset();
                        }
                    }
                }

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
