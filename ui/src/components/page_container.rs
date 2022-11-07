use yew::prelude::*;

#[function_component(PageContainer)]
pub fn page_container(props: &PageContainerProps) -> Html {
    html! {
      <div class="page-container">
        {props.children.clone()}
      </div>
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct PageContainerProps {
    pub children: Children,
}
