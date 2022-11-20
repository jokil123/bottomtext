use yew::prelude::*;

#[function_component(PageContainer)]
pub fn page_container(props: &PageContainerProps) -> Html {
    html! {
    <div class="page-container no-scrollbar h-screen snap-y snap-mandatory overflow-y-scroll">
        {props.children.clone()}
    </div>
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct PageContainerProps {
    pub children: Children,
}
