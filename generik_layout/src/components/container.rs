use leptos::prelude::*;

use crate::shared::merge_class;

/// A max-width page wrapper that centers content horizontally. The editorial
/// equivalent of a print page: a fixed measure the eye can rely on, regardless
/// of viewport width.
///
/// Distinct from `Grid` (a 12-column grid) — `Container` is a simple
/// single-column width cap for page sections.
///
/// Props: `width` (max-width CSS length; `""` uses `--gl-max-width`),
/// `class`/`style` (passthrough).
#[component]
pub fn Container(
    children: Children,
    #[prop(default = "")] width: &'static str,
    #[prop(default = "")] class: &'static str,
    #[prop(default = "")] style: &'static str,
) -> impl IntoView {
    let class_str = merge_class("gl-container", class);

    let own_style = if width.is_empty() {
        String::new()
    } else {
        format!("max-width: {width};")
    };
    let style_str = if style.is_empty() {
        own_style
    } else if own_style.is_empty() {
        style.to_string()
    } else {
        format!("{own_style} {style}")
    };

    view! {
        <div class=class_str style=style_str>{children()}</div>
    }
}