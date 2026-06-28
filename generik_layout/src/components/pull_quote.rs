use leptos::prelude::*;

use crate::shared::merge_class;

/// Slot for the quote text of a `PullQuote`.
#[slot]
pub struct Quote {
    children: Children,
}

/// Slot for the attribution of a `PullQuote`. Optional.
#[slot]
pub struct Attribution {
    children: Children,
}

/// A pull-quote: a short extracted quote with optional attribution.
/// Pure typography wrapper, no layout mechanics.
///
/// Props: `class`/`style` (passthrough on the outer `<blockquote>`).
#[component]
pub fn PullQuote(
    quote_slot: Quote,
    #[prop(optional)] attribution_slot: Option<Attribution>,
    #[prop(default = "")] class: &'static str,
    #[prop(default = "")] style: &'static str,
) -> impl IntoView {
    let class_str = merge_class("gl-pullquote", class);
    // The slot children are `FnOnce`; call them once up front and wrap the
    // resulting view in the `<footer>`.
    let attribution_view = attribution_slot.map(|a| {
        view! { <footer class="gl-pullquote-attribution">{(a.children)()}</footer> }
    });
    view! {
        <blockquote class=class_str style=style>
            <div class="gl-pullquote-text">{(quote_slot.children)()}</div>
            {attribution_view}
        </blockquote>
    }
}