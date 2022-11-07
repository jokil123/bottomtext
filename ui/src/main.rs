use ui::pages::{app::App, ui_rework::UiRework};

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
    // yew::start_app::<App>();
}
