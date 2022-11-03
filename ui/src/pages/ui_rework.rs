use yew::prelude::*;

#[function_component(UiRework)]
pub fn uitest() -> Html {
    html! {
        <>
            <img class="guide page1" src="public/desktop_page_1.png" />
            <img class="guide page2" src="public/desktop_page_2.png" />
            <div class="pageContainer page1">
                <div class="googleAd page1 left paddingSide">
                </div>
                <div class="page1 center">
                </div>
                <div class="googleAd page1 right paddingSide">
                </div>
            </div>
            <div class="pageContainer page2">
                <div class="content page2">
                    <h1 id="heading1">{"WHAT, HOW?"}</h1>
                    <p>{"Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet."}</p>
                    <h1 id="heading2" class="hideSmall">{"WHAT, HOW?"}</h1>
                    <p class="hideSmall">{"Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet."}</p>

                    <div class="row2 fullWidth" id="pseudoFooter">
                        <div class="stats">
                            <div class="statName">{"total messages:"}</div>
                            <div class="statValue">{"1432"}</div>
                            <div class="statName">{"unique users:"}</div>
                            <div class="statValue">{"123"}</div>
                            <div class="statName">{"last message from:"}</div>
                            <div class="statValue">{"AT"}</div>
                        </div>
                        <div class="github">
                            <div class="supportMe">{"support me on"}</div>
                            <div class="ghTitle">{"GitHub"}</div>
                            <img src="public/github.png"/>
                        </div>
                    </div>
                </div>
            </div>
        </>
    }
}
