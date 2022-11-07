use yew::prelude::*;

#[function_component(UiRework)]
pub fn uitest() -> Html {
    html! {
    <div class="no-scrollbar h-screen snap-y snap-mandatory overflow-y-scroll">
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
                <form class="flex flex-col items-center font-serif">
                    <input class="input bg-transparent text-center text-2xl" placeholder="Type Something!" type="text" />
                    <input class="input bg-transparent text-center text-sm" placeholder="(Optional Subtext)" type="text" />
                    <input type="submit" />
                </form>
            </div>
            <div class="hidden h-28 bg-white md:block md:h-full md:w-28"></div>
        </div>
        <div class="flex h-screen snap-center flex-col items-center justify-around bg-black py-5 px-7 font-serif text-white transition-all sm:py-10 sm:px-16 md:px-24 lg:px-44 xl:px-60">
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
                    <div class="justify-self-end">{"1432"}</div>
                    <div class="font-bold">{"unique users:"}</div>
                    <div class="justify-self-end">{"123"}</div>
                    <div class="font-bold">{"last message from:"}</div>
                    <div class="justify-self-end">{"AT"}</div>
                </div>
                <div class="mt-10 flex flex-col items-center">
                    <div class="text-xs opacity-75">{"support me on"}</div>
                    <div class="text-4xl font-bold">{"GitHub"}</div>
                    <img class="w-24 py-2 invert" src="https://upload.wikimedia.org/wikipedia/commons/thumb/9/91/Octicons-mark-github.svg/900px-Octicons-mark-github.svg.png" />
                </div>
            </div>
        </div>
    </div>
    }
}
