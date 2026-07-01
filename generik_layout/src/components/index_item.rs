use leptos::prelude::*;

use crate::shared::merge_class;

/// A single entry in an editorial index (contents page, "in this issue" list,
/// related-reading rail). Renders a number, a category eyebrow, a title, and
/// optional meta in a tight grid row, with a thin rule beneath.
///
/// The atomic unit of an editorial navigation list. Stack several `IndexItem`s
/// inside a `Stack` to build a full contents page.
///
/// Props: `number` (the index number, e.g. "01"), `category` (the eyebrow label,
/// e.g. "Feature"), `title` (the entry title), `meta` (optional, e.g. "p. 14"
/// or "8 min"), `class`/`style` (passthrough).
#[component]
pub fn IndexItem(
    number: &'static str,
    category: &'static str,
    title: &'static str,
    #[prop(optional)] meta: Option<&'static str>,
    #[prop(default = "")] class: &'static str,
    #[prop(default = "")] style: &'static str,
) -> impl IntoView {
    let class_str = merge_class("gl-index-item", class);

    let meta_view = meta.map(|m| view! { <span class="gl-index-item-meta">{m}</span> });

    view! {
        <div class=class_str style=style>
            <span class="gl-index-item-number">{number}</span>
            <div class="gl-index-item-body">
                <span class="gl-index-item-category">{category}</span>
                <span class="gl-index-item-title">{title}</span>
            </div>
            {meta_view}
        </div>
    }
}