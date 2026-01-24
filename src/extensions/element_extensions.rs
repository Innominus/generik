use web_sys::{window, Element, ScrollBehavior, ScrollToOptions};

pub trait ElementExtensions {
    fn scroll_to_with_offset(&self, offset: f64);
    fn scroll_element_to_with_offset(&self, element: Element, offset: f64);
}

impl ElementExtensions for web_sys::Element {
    fn scroll_to_with_offset(&self, offset: f64) {
        let window = window().unwrap();
        let rect = self.get_bounding_client_rect();
        let target_position = rect.top() + window.scroll_y().unwrap() - offset;
        let scroll_options = ScrollToOptions::new();

        scroll_options.set_top(target_position);
        scroll_options.set_behavior(ScrollBehavior::Smooth);
        window.scroll_to_with_scroll_to_options(&scroll_options);
    }

    fn scroll_element_to_with_offset(&self, element: Element, offset: f64) {
        let self_rect = self.get_bounding_client_rect();
        let container_rect = element.get_bounding_client_rect();

        let element_top = self_rect.top() - container_rect.top() + element.scroll_top() as f64;
        let target_position = element_top - offset;
        let scroll_options = ScrollToOptions::new();

        scroll_options.set_top(target_position);
        scroll_options.set_behavior(ScrollBehavior::Smooth);
        element.scroll_to_with_scroll_to_options(&scroll_options);
    }
}
