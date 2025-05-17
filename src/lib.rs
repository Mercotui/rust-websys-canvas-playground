use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, HtmlCanvasElement, CanvasRenderingContext2d};

mod click_view;
use click_view::ClickView;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // Set panic hook for better error messages.
    console_error_panic_hook::set_once();

    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()?;
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    let view = ClickView::new(canvas, context, window);
    view.attach_click_handler()?;

    Ok(())
}
