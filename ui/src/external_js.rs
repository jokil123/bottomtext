use wasm_bindgen::prelude::*;

#[wasm_bindgen(
    inline_js = "export function push_ad() { (window.adsbygoogle = window.adsbygoogle || []).push({}); }"
)]
extern "C" {
    pub fn push_ad();
}

#[wasm_bindgen(module = "/js/maintain_aspect_ratio.js")]
extern "C" {
    pub fn maintain_aspect_ratio(aspect_ratio: f32, id: &str);
}
