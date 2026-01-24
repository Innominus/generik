use std::future::Future;
use std::{cell::RefCell, rc::Rc};

use wasm_bindgen_futures::spawn_local;
use web_sys::{wasm_bindgen::JsCast, Event, HtmlElement};

pub fn setup_infinite_scroll<E, F, Fut>(on_scroll_end: F) -> impl Fn(E)
where
    E: AsRef<Event> + Clone + 'static,
    F: Fn(E) -> Fut + Clone + 'static,
    Fut: Future<Output = ()> + 'static,
{
    // Create state that persists across scroll events
    let is_running = Rc::new(RefCell::new(false));

    move |event: E| {
        let is_running = Rc::clone(&is_running);

        // Early return if already loading
        if *is_running.borrow() {
            return;
        }

        let element = event
            .as_ref()
            .target()
            .unwrap()
            .unchecked_into::<HtmlElement>();

        let element_height = element.scroll_height() - element.offset_height();
        let threshold_height = element_height - (element_height as f32 * 0.1) as i32;

        if element.scroll_top() >= threshold_height {
            // Set loading flag
            *is_running.borrow_mut() = true;

            // Clone the closure for async context
            let on_scroll_end = on_scroll_end.clone();
            let event = event.clone();

            spawn_local(async move {
                // Execute the async loading
                on_scroll_end(event).await;

                // Reset flag when done
                *is_running.borrow_mut() = false;
            });
        }
    }
}
