mod info;
mod landing;

use yew::prelude::*;

use crate::components::page_container::PageContainer;
use crate::pages::index::info::Info;
use crate::pages::index::landing::Landing;

#[function_component(Index)]
pub fn index() -> Html {
    html! {
        <PageContainer>
            <Landing />
            <Info />
        </PageContainer>
    }
}
