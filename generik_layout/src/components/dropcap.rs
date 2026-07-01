use leptos::prelude::*;

use crate::shared::merge_class;

/// An editorial drop cap: enlarges the first letter of the child content via
/// `::first-letter`. Pure CSS, no JS, no layout shift.
///
/// Wrap a single `<p>` (or any block with text content) to give its first
/// letter a 3-line display-font capital in the accent color. The
/// `.gl-dropcap::first-letter` rule is provided by the crate's global CSS.
///
/// Props: `class`/`style` (passthrough).
#[component]
pub fn Dropcap(
    children: Children,
    #[prop(default = "")] class: &'static str,
    #[prop(default = "")] style: &'static str,
) -> impl IntoView {
    let class_str = merge_class("gl-dropcap", class);

    view! {
        <div class=class_str style=style>{children()}</div>
    }
}