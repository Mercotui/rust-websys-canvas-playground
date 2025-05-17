use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, Element, HtmlCanvasElement, MouseEvent};

pub struct ClickView {
    canvas: HtmlCanvasElement,
    context: Rc<CanvasRenderingContext2d>,
}

impl ClickView {
    pub fn new(canvas: HtmlCanvasElement, context: CanvasRenderingContext2d) -> Self {
        ClickView {
            canvas,
            context: Rc::new(context),
        }
    }

    pub fn attach_click_handler(&self) -> Result<(), JsValue> {
        let closure_ctx = self.context.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| {
            let rect = event
                .target()
                .unwrap()
                .dyn_ref::<Element>()
                .unwrap()
                .get_bounding_client_rect();

            let x = event.client_x() as f64 - rect.left();
            let y = event.client_y() as f64 - rect.top();

            closure_ctx.begin_path();
            closure_ctx
                .arc(x, y, 10.0, 0.0, std::f64::consts::PI * 2.0)
                .unwrap();
            closure_ctx.set_fill_style_str("red");
            closure_ctx.fill();
        });
        self.canvas
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget(); // Prevents closure from being dropped
        Ok(())
    }
}
