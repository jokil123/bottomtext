use yew::prelude::*;

#[function_component(Landing)]
pub fn landing() -> Html {
    html! {
        <div class="landing">
            <h1>{"Welcome to the Frame App!"}</h1>
            <p>{"This is a simple app that allows you to create and share frames."}</p>
            <p>{"To get started, click the 'Create Frame' button in the top right corner."}</p>
        </div>
    }
}
