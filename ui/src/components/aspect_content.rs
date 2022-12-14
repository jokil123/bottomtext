use crate::external_js::maintain_aspect_ratio;
use clone_all::clone_all;
use rand::random;
use yew::prelude::*;

#[function_component(AspectContent)]
pub fn aspect_content(props: &AspectContentProps) -> Html {
    let id = use_ref(|| random::<u32>());

    {
        clone_all!(id);
        let ratio = props.ratio;
        use_effect_with_deps(
            move |_| {
                maintain_aspect_ratio(ratio, &id.to_string());
                log::info!("test");
                || ()
            },
            (),
        );
    }

    html! {
        <div id={(*id).to_string()} class="w-full h-full relative">
          <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2">
            {props.children.clone().unwrap_or_default()}
          </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct AspectContentProps {
    pub ratio: f32,
    pub children: Option<Children>,
}
