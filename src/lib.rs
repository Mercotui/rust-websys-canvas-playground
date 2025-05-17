use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Element, HtmlCanvasElement, CanvasRenderingContext2d, MouseEvent};

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

    // Clone context for closure
    let context = std::rc::Rc::new(context);

    let closure_ctx = context.clone();
    let closure = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| {
        let rect = event.target().unwrap()
            .dyn_ref::<Element>().unwrap()
            .get_bounding_client_rect();

        let x = event.client_x() as f64 - rect.left();
        let y = event.client_y() as f64 - rect.top();

        closure_ctx.begin_path();
        closure_ctx
            .arc(x, y, 10.0, 0.0, std::f64::consts::PI * 2.0)
            .unwrap();
        closure_ctx.set_fill_style(&JsValue::from_str("red"));
        closure_ctx.fill();
    });
    canvas.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
    closure.forget(); // Prevents closure from being dropped

    Ok(())
}