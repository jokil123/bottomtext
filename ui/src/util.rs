use common::frame::{FrameJson, FramesJson};
use gloo_net::http::Request;
use thiserror::Error;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlElement, HtmlInputElement};

#[derive(Error, Debug)]
pub enum ConversionError {
    #[error("Element not found")]
    NoneValue,
    #[error("Element is not an HtmlElement")]
    NonHtmlElement,
}

pub fn get_html_element_by_id(id: &str) -> Result<HtmlElement, ConversionError> {
    let element: HtmlElement = match web_sys::window() {
        Some(w) => match w.document() {
            Some(d) => match d.get_element_by_id(id) {
                Some(e) => match e.dyn_into() {
                    Ok(e) => e,
                    Err(_) => return Err(ConversionError::NonHtmlElement),
                },
                None => return Err(ConversionError::NoneValue),
            },
            None => return Err(ConversionError::NoneValue),
        },
        None => return Err(ConversionError::NoneValue),
    };

    Ok(element)
}

pub fn htmlelement_from_event(e: Event) -> Result<HtmlElement, ConversionError> {
    let element: HtmlElement = match e.target() {
        Some(e) => match e.dyn_into() {
            Ok(e) => e,
            Err(_) => return Err(ConversionError::NonHtmlElement),
        },
        None => return Err(ConversionError::NoneValue),
    };

    Ok(element)
}

pub fn value_from_event(e: Event) -> Result<String, ConversionError> {
    let element: HtmlInputElement = match e.target() {
        // TODO: remove unnecessary cast
        Some(e) => match e.dyn_into() {
            Ok(e) => e,
            Err(_) => return Err(ConversionError::NonHtmlElement),
        },
        None => return Err(ConversionError::NoneValue),
    };

    Ok(element.value())
}

pub async fn request_frames() -> Vec<FrameJson> {
    let mut fetched_frames: FramesJson = Request::get("/api/frames")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    fetched_frames.frames.reverse();

    fetched_frames.frames
}
