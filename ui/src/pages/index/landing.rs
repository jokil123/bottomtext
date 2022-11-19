use crate::components::{aspect_content::AspectContent, frame::Frame, input::Input};
use crate::context::AppContext;
use yew::prelude::*;

#[function_component(Landing)]
pub fn landing() -> Html {
    let context = use_context::<AppContext>().expect("Failed to get context");

    html! {
    <div class="flex h-screen snap-center flex-col justify-between bg-black text-white md:flex-row items-center">
        // <div class="absolute top-0 left-0 min-w-1/2 bg-slate-600">
        //     <Ad slot="4973753444"/>
        // </div>
        <div class="bg-white h-28 md:w-28 w-full md:h-full"></div>
        <div class="flex flex-grow flex-col justify-around p-7 md:h-full md:w-auto w-full h-auto">
            <Frame frames={context.frames.clone()} depth=0/>
            <Input />
        </div>
        <div class="hidden h-28 bg-white md:block md:h-full md:w-28"></div>
    </div>
    }
}
