use leptos::prelude::*;

use crate::shared::merge_class;

/// Slot for the header of a `Card`. Optional.
#[slot]
pub struct CardHeader {
    children: Children,
}

/// Slot for the body of a `Card`. Optional.
#[slot]
pub struct CardBody {
    children: Children,
}

/// Slot for the footer of a `Card`. Optional.
#[slot]
pub struct CardFooter {
    children: Children,
}

/// A general-purpose card with optional header, body, and footer slots.
/// Renders whichever slots are present, in source order header → body → footer.
///
/// Props: `class`/`style` (passthrough on the outer `<div>`).
#[component]
pub fn Card(
    #[prop(optional)] header_slot: Option<CardHeader>,
    #[prop(optional)] body_slot: Option<CardBody>,
    #[prop(optional)] footer_slot: Option<CardFooter>,
    #[prop(default = "")] class: &'static str,
    #[prop(default = "")] style: &'static str,
) -> impl IntoView {
    let class_str = merge_class("gl-card", class);
    // The slot children are `FnOnce`; call them once up front and wrap each in
    // its region element, so the optional slots render their wrapped view.
    let header_view = header_slot.map(|h| {
        view! { <div class="gl-card-header">{(h.children)()}</div> }
    });
    let body_view = body_slot.map(|b| {
        view! { <div class="gl-card-body">{(b.children)()}</div> }
    });
    let footer_view = footer_slot.map(|f| {
        view! { <div class="gl-card-footer">{(f.children)()}</div> }
    });
    view! {
        <div class=class_str style=style>
            {header_view}
            {body_view}
            {footer_view}
        </div>
    }
}