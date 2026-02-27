use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

use js_sys::Array;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Element, IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};

const OBSERVER_ID_ATTRIBUTE: &'static str = "data-observer-id";

type HashIncrement = usize;

pub struct ObserverCallback {
    /// Only trigger once when element becomes visible (doesn't re-trigger on scroll out/in)
    pub once: Option<bool>,
    pub callback: Box<dyn Fn() + 'static>,
}

/// Simple configuration for the intersection observer
#[derive(Debug, Clone)]
pub struct IntersectionConfig {
    /// The threshold at which the observer's callback should be executed
    /// 0.0 = as soon as any part is visible, 1.0 = when fully visible
    pub threshold: f64,
}

impl Default for IntersectionConfig {
    fn default() -> Self {
        Self { threshold: 0.1 }
    }
}

#[derive(Clone)]
pub struct Intersectioner {
    observer: IntersectionObserver,
    observer_callbacks:
        Rc<std::cell::RefCell<(HashIncrement, HashMap<HashIncrement, ObserverCallback>)>>,
    _observer_event_callback: Rc<Closure<dyn FnMut(Array)>>,
}

impl Intersectioner {
    pub fn new(config: IntersectionConfig) -> Self {
        let observer_callbacks = Rc::new(RefCell::new((0, HashMap::new())));

        let inner_callbacks = observer_callbacks.clone();

        let callback = Closure::wrap(Box::new(move |entries: js_sys::Array| {
            for i in 0..entries.length() {
                let entry = entries.get(i).unchecked_into::<IntersectionObserverEntry>();

                if entry.is_intersecting() {
                    // We love to unwrap unwrap unwrap (if this became serious, we'd need to handle this)
                    let mut map = inner_callbacks.borrow_mut();

                    let observee: &mut ObserverCallback = map
                        .1
                        .get_mut(
                            &entry
                                .target()
                                .get_attribute(OBSERVER_ID_ATTRIBUTE)
                                .unwrap()
                                .parse::<usize>()
                                .unwrap(),
                        )
                        .unwrap();

                    // Check if we should only trigger once
                    if let Some(triggered) = observee.once {
                        if triggered {
                            return;
                        }
                        observee.once = Some(true);
                    }

                    (observee.callback)();
                }
            }
        }) as Box<dyn FnMut(js_sys::Array)>);

        let options = IntersectionObserverInit::new();
        options.set_threshold(&JsValue::from_f64(config.threshold.clamp(0.0, 1.0)));

        let observer =
            IntersectionObserver::new_with_options(callback.as_ref().unchecked_ref(), &options)
                .unwrap();

        Intersectioner {
            observer,
            observer_callbacks,
            _observer_event_callback: Rc::new(callback),
        }
    }

    pub fn observe<F>(&self, element: &Element, once: bool, callback: F)
    where
        F: Fn() + 'static,
    {
        let mut callbacks = self.observer_callbacks.borrow_mut();

        let observer_callback = ObserverCallback {
            once: if once { Some(true) } else { None },
            callback: Box::new(callback),
        };

        let index = callbacks.0;
        callbacks.1.insert(index, observer_callback);

        _ = element.set_attribute(OBSERVER_ID_ATTRIBUTE, &callbacks.0.to_string());

        callbacks.0 += callbacks.0.wrapping_add(1);
        self.observer.observe(element.unchecked_ref());
    }

    pub fn unobserve(&self, element: &Element) {
        let id = element
            .get_attribute(OBSERVER_ID_ATTRIBUTE)
            .unwrap()
            .parse::<usize>()
            .unwrap();

        self.observer_callbacks.borrow_mut().1.remove(&id);
        self.observer.unobserve(element.unchecked_ref());
    }
}

impl Drop for Intersectioner {
    fn drop(&mut self) {
        self.observer.disconnect();
    }
}
