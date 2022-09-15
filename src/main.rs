use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
struct FrameModel {
    text: String,
    subtext: Option<String>,
    depth: i32,
    inner: Option<Box<FrameModel>>,
}

impl FrameModel {
    fn new(frame_data: Vec<(&str, Option<&str>)>, depth: Option<i32>) -> Option<FrameModel> {
        let depth = match depth {
            Some(d) => d,
            None => 0,
        };

        match frame_data.get(0) {
            Some(d) => Some(FrameModel {
                text: d.0.to_string(),
                subtext: d.1.map(str::to_string),
                depth: depth,
                inner: match FrameModel::new(frame_data[1..].to_owned(), Some(depth + 1)) {
                    Some(f) => Some(Box::new(f)),
                    None => None,
                },
            }),
            None => None,
        }
    }
}

#[function_component(App)]
fn app() -> Html {
    // let frame = FrameModel {
    //     text: "WHAT".to_string(),
    //     subtext: Some("HOW".to_string()),
    //     inner: Some(Box::new(FrameModel {
    //         text: "WHAT".to_string(),
    //         subtext: Some("HOW".to_string()),
    //         inner: None,
    //     })),
    // };

    let frame = FrameModel::new(
        vec![("WHAT", Some("HOW")), ("WHAT", Some("HOW")), ("WHAT", None)],
        None,
    )
    .unwrap();

    html! {
        <Frame frame={frame} />
    }
}

#[function_component(Frame)]
fn frame(props: &FrameProps) -> Html {
    html!(
        <div class="frameContainer" depth={props.frame.depth.to_string()}>
            <div class="frameBorder">
                {match &props.frame.inner {
                    Some(inner) => html! {
                        <Frame frame={inner.as_ref().clone()} />
                    },
                    None => html! {},
                }}
            </div>


            <h1 class="text">{&props.frame.text}</h1>

            {match &props.frame.subtext {
                Some(subtext) => html! {
                    <h2 class="text">{subtext}</h2>
                },
                None => html! {},
            }}
        </div>
    )
}

#[derive(Properties, PartialEq)]
struct FrameProps {
    frame: FrameModel,
}

// #[function_component(FrameMock)]
// fn frame_mock() -> Html {
//     // html! {
//     //     <div class="frameMock">
//     //         <div class="frameMock">
//     //             <div class="frameMock">
//     //             </div>
//     //         </div>
//     //     </div>
//     // }
//     html! {
//         <div class="frameContainer">
//             <div class="frameBorder">
//                 //Inner
//                     <div class="frameContainer">
//                         <div class="frameBorder">
//                         </div>
//                         <h1>{"WHAT"}</h1>
//                         <h2>{"HOW"}</h2>
//                     </div>
//                 //Inner
//             </div>
//             <h1>{"WHAT"}</h1>
//             <h2>{"HOW"}</h2>
//         </div>
//     }
// }

fn main() {
    // yew::start_app::<App>();
    yew::start_app::<App>();
}
