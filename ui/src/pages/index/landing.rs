use crate::components::input::Input;
use yew::prelude::*;

#[function_component(Landing)]
pub fn landing() -> Html {
    html! {
    <div class="flex h-screen snap-center flex-col justify-between bg-black md:flex-row">
        <div class="h-28 bg-white md:h-full md:w-28"></div>
        <div class="flex flex-grow flex-col justify-around p-7">
            // <div class="aspect-golden-w border">
            //     <div class="flex flex-grow flex-col justify-around p-7">
            //         <div class="aspect-golden-w border"></div>
            //         <div class="flex flex-col items-center py-2 font-serif">
            //             <div class="input bg-transparent text-center text-2xl">{"What?"}</div>
            //             <div class="input bg-transparent text-center text-sm">{"How?"}</div>
            //         </div>
            //     </div>
            // </div>
            <Input />
        </div>
        <div class="hidden h-28 bg-white md:block md:h-full md:w-28"></div>
    </div>
    }
}
