use crate::components::{ad::Ad, frame::Frame, input::Input};
use crate::context::AppContext;
use yew::prelude::*;

#[function_component(Landing)]
pub fn landing() -> Html {
    let context = use_context::<AppContext>().expect("Failed to get context");

    html! {
    <section class="flex h-screen snap-center flex-col justify-between bg-black md:flex-row items-center">
        // <div class="bg-slate-500 h-28 md:w-28 w-full md:h-full">
        //     <Ad slot="4973753444"/>
        // </div>
        <div class="flex flex-grow flex-col justify-around p-7 md:h-full md:w-auto w-full h-auto">
            <Frame frames={context.frames.clone()} depth=0/>
            <Input />
        </div>
        // <div class="hidden h-28 bg-slate-500 md:block md:h-full md:w-28"></div>
    </section>
    }
}
