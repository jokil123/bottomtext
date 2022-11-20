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
    <section class="flex h-screen snap-center flex-col items-center justify-around bg-black py-5 px-7 font-serif transition-all sm:py-10 sm:px-16 md:px-24 lg:px-44 xl:px-60">
        <section class="pb-7">
            <h1 class="text-3xl text-center">{"What, Why?"}</h1>
            <p class="text-sm text-justify">{"You have probably all seen the iconic memes, so I went ahead and turned it into an interactive “chat” app. All users see messages of all other users, its like a giant chatroom. I had this idea a long time ago and finally decided to build this app for real. Additionally, I was also searching for a new project at the time and wanted to learn more about rust and full stack development. The project seemed like a good idea and an interesting challenge, so I started work. The process was more or less fun aside from some hiccups, partly due to my lack of skill and partly due to poor documentation."}</p>
        </section>

        <section class="hidden sm:block">
            <h1 class="text-3xl text-center">{"What, How?"}</h1>
            <p class="text-sm text-justify">{"I created the entire app in Rust, a blazingly fast systems programming language. The backend was build using Warp, which is a very simple yet powerful webserver and Tokio as the async runtime. Persisting the data is achieved with a custom database implementation designed perfectly for the apps needs. The frontend is built with Yew, a react like, component-based web framework, utilizing a syntax like react JSX by using rusts macros. Frontend and backend communicate using web sockets. The entire application is packaged as a docker container and hosted on a digital ocean droplet."}</p>
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
    </section>
    }
}
