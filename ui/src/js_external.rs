use yew_set_state_onchange::wasm_bindgen;

#[wasm_bindgen(
    inline_js = "export function push_ad() { (window.adsbygoogle = window.adsbygoogle || []).push({}); }"
)]
extern "C" {
    pub fn push_ad();
}
