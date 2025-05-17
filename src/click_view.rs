use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{closure::Closure, JsCast, JsValue, prelude::*};
use web_sys::{CanvasRenderingContext2d, Element, HtmlCanvasElement, MouseEvent, Window};
use wasm_bindgen_futures::JsFuture;

// Helper to await a single animation frame, now takes a flag to prevent parallel requests
async fn wait_animation_frame(window: &Window, frame_pending: Rc<RefCell<bool>>) -> Result<(), JsValue> {
    {
        let mut pending = frame_pending.borrow_mut();
        if *pending {
            // Already waiting for an animation frame, just return early
            return Ok(());
        }
        *pending = true;
    }

    let (sender, receiver) = futures_channel::oneshot::channel();

    // We need to clone the sender so it can be used in the closure below
    let sender = RefCell::new(Some(sender));
    let frame_pending_clone = frame_pending.clone();
    let cb = Closure::once(Box::new(move || {
        if let Some(sender) = sender.borrow_mut().take() {
            // Ignore errors (receiver may have been dropped)
            let _ = sender.send(());
        }
        // Reset the pending flag when the frame is done
        *frame_pending_clone.borrow_mut() = false;
    }) as Box<dyn FnOnce()>);

    window
        .request_animation_frame(cb.as_ref().unchecked_ref())?;
    cb.forget(); // don't drop until callback is called

    // Wait for signal from the callback
    let _ = receiver.await;
    Ok(())
}

pub struct ClickView {
    canvas: HtmlCanvasElement,
    context: Rc<CanvasRenderingContext2d>,
    window: Window,
    frame_pending: Rc<RefCell<bool>>,
}

impl ClickView {
    pub fn new(canvas: HtmlCanvasElement, context: CanvasRenderingContext2d, window: Window) -> Self {
        ClickView {
            canvas,
            context: Rc::new(context),
            window,
            frame_pending: Rc::new(RefCell::new(false)),
        }
    }

    pub fn attach_click_handler(&self) -> Result<(), JsValue> {
        let closure_ctx = self.context.clone();
        let window = self.window.clone();
        let frame_pending = self.frame_pending.clone();

        let closure = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| {
            let rect = event
                .target()
                .unwrap()
                .dyn_ref::<Element>()
                .unwrap()
                .get_bounding_client_rect();

            let x = event.client_x() as f64 - rect.left();
            let y = event.client_y() as f64 - rect.top();

            let closure_ctx = closure_ctx.clone();
            let window = window.clone();
            let frame_pending = frame_pending.clone();

            wasm_bindgen_futures::spawn_local(async move {
                // Await until next animation frame, with safeguard
                let _ = wait_animation_frame(&window, frame_pending).await;

                // Now do the drawing
                closure_ctx.begin_path();
                closure_ctx
                    .arc(x, y, 10.0, 0.0, std::f64::consts::PI * 2.0)
                    .unwrap();
                closure_ctx.set_fill_style_str("blue");
                closure_ctx.fill();
            });
        });

        self.canvas
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
        Ok(())
    }
}
