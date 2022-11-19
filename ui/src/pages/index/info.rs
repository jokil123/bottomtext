use gloo::utils::window;
use rand::prelude::*;
use yew::prelude::*;

use crate::context::AppContext;

#[function_component(Info)]
pub fn info() -> Html {
    let context = use_context::<AppContext>().expect("error getting context");

    let open_github = |_| {
        window()
            .location()
            .set_href("https://github.com/jokil123")
            .expect("error opening github")
    };

    html! {
    <div class="flex h-screen snap-center flex-col items-center justify-around bg-black py-5 px-7 font-serif transition-all sm:py-10 sm:px-16 md:px-24 lg:px-44 xl:px-60">
        <section class="pb-7 text-center">
            <h1 class="text-3xl">{"What, How?"}</h1>
            <p class="text-sm">{"Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet."}</p>
        </section>

        <section class="hidden text-center sm:block">
            <h1 class="text-3xl">{"What, How?"}</h1>
            <p class="text-sm">{"Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet."}</p>
        </section>

        <div class="contents w-full items-center justify-around sm:flex">
            <div class="items-middle grid h-28 max-w-sm grid-cols-[2fr,1fr] grid-rows-3 items-center text-xl">
                <div class="font-bold">{"total messages:"}</div>
                <div class="justify-self-end">
                  {context.frames.len()}
                </div>
                <div class="font-bold">{"unique users:"}</div>
                <div class="justify-self-end">
                  {random::<u16>()}
                </div>
                <div class="font-bold">{"last message from:"}</div>
                <div class="justify-self-end">
                  {"AT"}
                </div>
            </div>
            <div class="mt-10 flex flex-col items-center" onclick={open_github}>
                <div class="text-xs opacity-75">{"support me on"}</div>
                <div class="text-4xl font-bold">{"GitHub"}</div>
                <img class="w-24 py-2 invert" src="https://upload.wikimedia.org/wikipedia/commons/thumb/9/91/Octicons-mark-github.svg/900px-Octicons-mark-github.svg.png" />
            </div>
        </div>
    </div>
    }
}
