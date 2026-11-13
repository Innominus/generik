//! One shared [`ResizeObserver`](web_sys::ResizeObserver) per thread.
//!
//! [`observe_resize`] subscribes an element with a callback and returns a
//! [`ResizeObserverHandle`]; dropping the handle (or calling
//! [`ResizeObserverHandle::disconnect`]) unsubscribes. Every subscription shares
//! one browser observer, and an element is unobserved once its last subscriber
//! leaves.
//!
//! Off `wasm32` nothing is observed, so native builds and tests still compile.
//!
//! ```ignore
//! let handle = generik_resize::observe_resize(&element, || log("resized"));
//! // ...
//! drop(handle);
//! ```

use web_sys::Element;

#[cfg(target_arch = "wasm32")]
mod wasm {
    use std::cell::RefCell;
    use std::rc::Rc;

    use js_sys::Array;
    use web_sys::wasm_bindgen::JsCast;
    use web_sys::wasm_bindgen::closure::Closure;
    use web_sys::{Element, ResizeObserver, ResizeObserverEntry};

    struct ResizeSubscription {
        id: u32,
        element: Element,
        callback: Rc<dyn Fn()>,
    }

    #[derive(Default)]
    struct SharedResizeObserverState {
        next_id: u32,
        subscriptions: Vec<ResizeSubscription>,
    }

    struct SharedResizeObserver {
        observer: ResizeObserver,
        state: Rc<RefCell<SharedResizeObserverState>>,
        _callback: Closure<dyn FnMut(Array, ResizeObserver)>,
    }

    thread_local! {
        static SHARED_RESIZE_OBSERVER: RefCell<Option<SharedResizeObserver>> = const { RefCell::new(None) };
    }

    impl SharedResizeObserver {
        fn new() -> Option<Self> {
            let state = Rc::new(RefCell::new(SharedResizeObserverState::default()));
            let callback_state = state.clone();
            let callback =
                Closure::wrap(Box::new(move |entries: Array, _observer: ResizeObserver| {
                    for entry in entries.iter() {
                        let entry: ResizeObserverEntry = entry.unchecked_into();
                        let target = entry.target();
                        // Collect first so callbacks may subscribe or unsubscribe.
                        let callbacks = {
                            let state = callback_state.borrow();
                            state
                                .subscriptions
                                .iter()
                                .filter(|subscription| subscription.element == target)
                                .map(|subscription| subscription.callback.clone())
                                .collect::<Vec<_>>()
                        };

                        for callback in callbacks {
                            callback();
                        }
                    }
                }) as Box<dyn FnMut(Array, ResizeObserver)>);

            let observer = ResizeObserver::new(callback.as_ref().unchecked_ref()).ok()?;
            Some(Self {
                observer,
                state,
                _callback: callback,
            })
        }
    }

    fn with_shared_resize_observer<R>(f: impl FnOnce(&SharedResizeObserver) -> R) -> Option<R> {
        SHARED_RESIZE_OBSERVER.with(|slot| {
            if slot.borrow().is_none() {
                *slot.borrow_mut() = SharedResizeObserver::new();
            }

            let borrow = slot.borrow();
            let observer = borrow.as_ref()?;
            Some(f(observer))
        })
    }

    pub(super) fn remove_resize_subscription(subscription_id: u32) {
        SHARED_RESIZE_OBSERVER.with(|slot| {
            let borrow = slot.borrow();
            let Some(observer) = borrow.as_ref() else {
                return;
            };

            let mut state = observer.state.borrow_mut();
            let Some(index) = state
                .subscriptions
                .iter()
                .position(|subscription| subscription.id == subscription_id)
            else {
                return;
            };
            let removed = state.subscriptions.remove(index);
            let still_observed = state
                .subscriptions
                .iter()
                .any(|subscription| subscription.element == removed.element);
            // Release the state before the callback drops, in case it owns a handle.
            drop(state);

            if !still_observed {
                observer.observer.unobserve(&removed.element);
            }
        });
    }

    /// `None` when the browser has no `ResizeObserver`.
    pub(super) fn observe_resize(element: &Element, callback: impl Fn() + 'static) -> Option<u32> {
        let callback = Rc::new(callback);
        with_shared_resize_observer(|observer| {
            let mut state = observer.state.borrow_mut();
            let subscription_id = state.next_id;
            state.next_id = state.next_id.wrapping_add(1);
            let should_observe = !state
                .subscriptions
                .iter()
                .any(|subscription| subscription.element == *element);
            state.subscriptions.push(ResizeSubscription {
                id: subscription_id,
                element: element.clone(),
                callback,
            });
            drop(state);

            if should_observe {
                observer.observer.observe(element);
            }

            subscription_id
        })
    }
}

/// A subscription from [`observe_resize`]. Dropping it unsubscribes.
#[must_use = "dropping the handle stops observing"]
pub struct ResizeObserverHandle {
    subscription_id: Option<u32>,
}

impl ResizeObserverHandle {
    /// Stop calling this subscription's callback. Calling it again does nothing.
    pub fn disconnect(&mut self) {
        let Some(subscription_id) = self.subscription_id.take() else {
            return;
        };

        #[cfg(target_arch = "wasm32")]
        wasm::remove_resize_subscription(subscription_id);

        #[cfg(not(target_arch = "wasm32"))]
        let _ = subscription_id;
    }
}

impl Drop for ResizeObserverHandle {
    fn drop(&mut self) {
        self.disconnect();
    }
}

/// Call `callback` once `element` is first observed and again whenever its
/// content box resizes, until the returned handle is dropped or disconnected.
///
/// Where `ResizeObserver` is unavailable (or off `wasm32`) the callback never
/// runs.
pub fn observe_resize<F>(element: &Element, callback: F) -> ResizeObserverHandle
where
    F: Fn() + 'static,
{
    #[cfg(target_arch = "wasm32")]
    let subscription_id = wasm::observe_resize(element, callback);

    #[cfg(not(target_arch = "wasm32"))]
    let subscription_id = {
        let _ = (element, callback);
        None
    };

    ResizeObserverHandle { subscription_id }
}
