use thiserror::Error;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::Html;

#[derive(Error, Debug)]
pub enum ByIdError {
    #[error("Element not found")]
    NoneValue,
    #[error("Element is not an HtmlElement")]
    NotHtmlElement,
}

pub fn get_html_element_by_id(id: &str) -> Result<HtmlElement, ByIdError> {
    let element: HtmlElement = match web_sys::window() {
        Some(w) => match w.document() {
            Some(d) => match d.get_element_by_id(id) {
                Some(e) => match e.dyn_into() {
                    Ok(e) => e,
                    Err(_) => return Err(ByIdError::NotHtmlElement),
                },
                None => return Err(ByIdError::NoneValue),
            },
            None => return Err(ByIdError::NoneValue),
        },
        None => return Err(ByIdError::NoneValue),
    };

    Ok(element)
}
