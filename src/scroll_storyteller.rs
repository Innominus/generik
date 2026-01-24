use leptos::ev::resize;
use leptos::prelude::*;
use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use web_sys::{ScrollBehavior, ScrollToOptions};

/// Normalized scroll progress (0.0-1.0)
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ScrollProgress {
    pub progress: f64,
    pub scroll_y: f64,
    pub scroll_height: f64,
    pub viewport_height: f64,
}

impl ScrollProgress {
    #[inline(always)]
    pub fn new(scroll_y: f64, scroll_height: f64, viewport_height: f64) -> Self {
        let mut new_progress = Self {
            progress: 0f64,
            scroll_y,
            scroll_height,
            viewport_height,
        };

        new_progress.calculate_progress();

        new_progress
    }

    #[inline(always)]
    pub fn calculate_progress(&mut self) {
        let max_scroll = (self.scroll_height - self.viewport_height).max(0.0);
        self.progress = if max_scroll > 0.0 {
            (self.scroll_y / max_scroll).clamp(self.viewport_height / self.scroll_height, 1.0)
        } else {
            1.0
        };
    }

    #[inline(always)]
    pub fn eased(&self, easing: EasingFunction) -> f64 {
        match easing {
            EasingFunction::Linear => self.progress,
            EasingFunction::EaseIn => self.progress * self.progress,
            EasingFunction::EaseOut => 1.0 - (1.0 - self.progress) * (1.0 - self.progress),
            EasingFunction::EaseInOut => {
                if self.progress < 0.5 {
                    2.0 * self.progress * self.progress
                } else {
                    let t = -2.0 * self.progress + 2.0;
                    1.0 - t * t / 2.0
                }
            }
            EasingFunction::EaseInCubic => self.progress * self.progress * self.progress,
            EasingFunction::EaseOutCubic => {
                let t = 1.0 - self.progress;
                1.0 - t * t * t
            }
            EasingFunction::EaseInOutCubic => {
                if self.progress < 0.5 {
                    4.0 * self.progress * self.progress * self.progress
                } else {
                    let t = -2.0 * self.progress + 2.0;
                    1.0 - t * t * t / 2.0
                }
            }
        }
    }

    #[inline(always)]
    pub fn in_range(&self, from: f64, to: f64) -> f64 {
        if self.progress <= from {
            0.0
        } else if self.progress >= to {
            1.0
        } else {
            (self.progress - from) / (to - from)
        }
    }

    #[inline(always)]
    pub fn is_in_range(&self, from: f64, to: f64) -> bool {
        self.progress >= from && self.progress <= to
    }
}

#[derive(Clone, Copy)]
pub enum EasingFunction {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
}

#[derive(Clone)]
pub struct ScrollStorytellerConfig {
    pub throttle_ms: u32,
    pub smooth_scroll: bool,
    pub offset_top: f64,
    pub offset_bottom: f64,
    pub run_straight_away: bool,
    pub resize_debounce_ms: u32,
}

impl Default for ScrollStorytellerConfig {
    fn default() -> Self {
        Self {
            throttle_ms: 8,
            smooth_scroll: true,
            offset_top: 0.0,
            offset_bottom: 0.0,
            run_straight_away: false,
            resize_debounce_ms: 250,
        }
    }
}

pub struct ScrollStoryteller {
    element: HtmlElement,
    config: ScrollStorytellerConfig,
    last_progress: RwSignal<ScrollProgress>,
    callbacks: Rc<std::cell::RefCell<Vec<Box<dyn Fn(ScrollProgress)>>>>,
    last_scroll_time: Rc<Cell<f64>>,
    last_resize_time: Rc<Cell<f64>>,
}

impl ScrollStoryteller {
    pub fn new(
        element: HtmlElement,
        config: Option<ScrollStorytellerConfig>,
    ) -> Result<Self, JsValue> {
        let config = config.unwrap_or_default();
        let last_progress = RwSignal::new(ScrollProgress::new(0.0, 0.0, 0.0));
        let callbacks: Rc<std::cell::RefCell<Vec<Box<dyn Fn(ScrollProgress)>>>> =
            Rc::new(std::cell::RefCell::new(Vec::with_capacity(8)));
        let last_scroll_time = Rc::new(Cell::new(0.0));
        let last_resize_time = Rc::new(Cell::new(0.0));

        // Pre-calculate values for hot path
        let throttle_ms = config.throttle_ms as f64;
        let resize_debounce_ms = config.resize_debounce_ms as f64;
        let offset_top = config.offset_top;
        let offset_bottom = config.offset_bottom;

        // Create scroll closure with optimized hot path
        let element_clone = element.clone();
        let callbacks_clone = callbacks.clone();
        let last_scroll_time_clone = last_scroll_time.clone();

        let window = web_sys::window().unwrap();
        let body = window.document().unwrap().body().unwrap();
        let performance = window.performance().unwrap();

        // Initial values for full page calculation
        let current_window_height = window.inner_height().unwrap().unchecked_into_f64();
        let current_body_height = body.client_height();

        let scroll_closure = Closure::wrap(Box::new(move || {
            let now = performance.now();

            // Inline throttling check
            let last_time = last_scroll_time_clone.get();
            if now - last_time < throttle_ms {
                return;
            }
            last_scroll_time_clone.set(now);

            // Inline progress calculation
            let mut new_progress = last_progress.get_untracked();

            new_progress.scroll_y = element_clone.scroll_top() as f64 + offset_top;

            new_progress.calculate_progress();

            last_progress.set(new_progress);

            // Execute callbacks with minimal overhead
            for callback in callbacks_clone.borrow().iter() {
                callback(new_progress);
            }
        }) as Box<dyn FnMut()>);

        // Create resize closure
        let resize_element = element.clone();
        let resize_callbacks = callbacks.clone();
        let last_resize_time_clone = last_resize_time.clone();

        let window = web_sys::window().unwrap();
        let performance = window.performance().unwrap();

        // Attach event listeners
        element
            .add_event_listener_with_callback("scroll", scroll_closure.as_ref().unchecked_ref())?;

        let handle = window_event_listener(resize, move |_| {
            let now = performance.now();

            let last_time = last_resize_time_clone.get();
            if now - last_time < resize_debounce_ms {
                return;
            }
            last_resize_time_clone.set(now);

            let scroll_y = resize_element.scroll_top() as f64 + offset_top;
            let scroll_height = resize_element.scroll_height() as f64;
            let mut viewport_height =
                resize_element.client_height() as f64 - offset_top - offset_bottom;

            if viewport_height as i32 == body.client_height() {
                viewport_height -=
                    viewport_height - window.inner_height().unwrap().unchecked_into_f64();
            }

            let progress = ScrollProgress::new(scroll_y, scroll_height, viewport_height);
            last_progress.set(progress);

            for callback in resize_callbacks.borrow().iter() {
                callback(progress);
            }
        });

        // store the values in the reactive system which stops them from being dropped immediately
        // attaches their lifetime to the lifetime of the reactive context they're in
        let _ = StoredValue::new_local(scroll_closure);

        on_cleanup(move || {
            handle.remove();
        });

        // Calculate initial progress
        let scroll_y = element.scroll_top() as f64 + config.offset_top;
        let scroll_height = element.scroll_height() as f64;
        let mut viewport_height =
            element.client_height() as f64 - config.offset_top - config.offset_bottom;

        if viewport_height as i32 == current_body_height {
            viewport_height -= viewport_height - current_window_height;
        }

        let initial_progress = ScrollProgress::new(scroll_y, scroll_height, viewport_height);

        last_progress.set(initial_progress);

        // Handle run_straight_away for non-scrollable content
        let is_not_scrollable = scroll_height <= viewport_height;
        if config.run_straight_away || is_not_scrollable {
            let callbacks_clone = callbacks.clone();
            request_animation_frame(move || {
                for callback in callbacks_clone.borrow().iter() {
                    callback(initial_progress);
                }
            });
        }

        Ok(Self {
            element,
            config,
            last_progress,
            callbacks,
            last_scroll_time,
            last_resize_time,
        })
    }

    pub fn for_window(config: Option<ScrollStorytellerConfig>) -> Result<Self, JsValue> {
        let window = web_sys::window().ok_or("No window")?;
        let document = window.document().ok_or("No document")?;
        let element = document
            .document_element()
            .ok_or("No document element")?
            // This may just break. Does the document element allow casting to html element?
            .unchecked_into::<HtmlElement>();
        Self::new(element, config)
    }

    #[inline(always)]
    pub fn on_scroll<F>(&self, callback: F)
    where
        F: Fn(ScrollProgress) + 'static,
    {
        self.callbacks.borrow_mut().push(Box::new(callback));
    }

    pub fn on_progress_range<F>(&self, from: f64, to: f64, callback: F)
    where
        F: Fn(ScrollProgress, f64) + 'static,
    {
        self.on_scroll(move |progress| {
            if progress.is_in_range(from, to) {
                callback(progress, progress.in_range(from, to));
            }
        });
    }

    pub fn on_enter_range<F>(&self, from: f64, to: f64, callback: F)
    where
        F: Fn(ScrollProgress) + 'static,
    {
        let was_in_range = Rc::new(Cell::new(None::<bool>));

        // Handle run_straight_away
        if self.config.run_straight_away {
            let current_progress = self.last_progress.get_untracked();
            let is_currently_in_range = current_progress.is_in_range(from, to);
            if is_currently_in_range {
                callback(current_progress);
                was_in_range.set(Some(true));
            } else {
                was_in_range.set(Some(false));
            }
        }

        self.on_scroll(move |progress| {
            let is_in_range = progress.is_in_range(from, to);
            match was_in_range.get() {
                None => {
                    if is_in_range {
                        callback(progress);
                    }
                    was_in_range.set(Some(is_in_range));
                }
                Some(was_in_range_val) => {
                    if is_in_range && !was_in_range_val {
                        callback(progress);
                    }
                    was_in_range.set(Some(is_in_range));
                }
            }
        });
    }

    pub fn on_exit_range<F>(&self, from: f64, to: f64, callback: F)
    where
        F: Fn(ScrollProgress) + 'static,
    {
        let was_in_range = Rc::new(Cell::new(None::<bool>));

        if self.config.run_straight_away {
            let current_progress = self.last_progress.get_untracked();
            let is_currently_in_range = current_progress.is_in_range(from, to);
            was_in_range.set(Some(is_currently_in_range));
        }

        self.on_scroll(move |progress| {
            let is_in_range = progress.is_in_range(from, to);
            match was_in_range.get() {
                None => was_in_range.set(Some(is_in_range)),
                Some(was_in_range_val) => {
                    if !is_in_range && was_in_range_val {
                        callback(progress);
                    }
                    was_in_range.set(Some(is_in_range));
                }
            }
        });
    }

    #[inline(always)]
    pub fn progress(&self) -> ReadSignal<ScrollProgress> {
        self.last_progress.read_only()
    }

    pub fn scroll_to_progress(&self, progress: f64) -> Result<(), JsValue> {
        let clamped_progress = progress.clamp(0.0, 1.0);
        let current = self.last_progress.get_untracked();
        let max_scroll = (current.scroll_height - current.viewport_height).max(0.0);
        let target_scroll = (clamped_progress * max_scroll) - self.config.offset_top;

        let options = ScrollToOptions::new();
        options.set_top(target_scroll);
        if self.config.smooth_scroll {
            options.set_behavior(ScrollBehavior::Smooth);
        }

        self.element.scroll_with_scroll_to_options(&options);
        Ok(())
    }

    pub fn scroll_to_pixels(&self, pixels: f64) -> Result<(), JsValue> {
        let options = ScrollToOptions::new();
        options.set_top(pixels);
        if self.config.smooth_scroll {
            options.set_behavior(ScrollBehavior::Smooth);
        }

        self.element.scroll_with_scroll_to_options(&options);
        Ok(())
    }
}

#[inline(always)]
pub fn create_window_storyteller() -> Result<ScrollStoryteller, JsValue> {
    ScrollStoryteller::for_window(None)
}

#[inline(always)]
pub fn create_window_storyteller_with_config(
    config: ScrollStorytellerConfig,
) -> Result<ScrollStoryteller, JsValue> {
    ScrollStoryteller::for_window(Some(config))
}

#[inline(always)]
pub fn create_element_storyteller(element: HtmlElement) -> Result<ScrollStoryteller, JsValue> {
    ScrollStoryteller::new(element, None)
}

#[inline(always)]
pub fn create_element_storyteller_with_config(
    element: HtmlElement,
    config: ScrollStorytellerConfig,
) -> Result<ScrollStoryteller, JsValue> {
    ScrollStoryteller::new(element, Some(config))
}
