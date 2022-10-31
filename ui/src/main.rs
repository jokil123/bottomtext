use ui::pages::app::App;
// use ui::pages::ws_test::chat::Chat;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    // yew::start_app::<App>();
    yew::start_app::<App>();
}
