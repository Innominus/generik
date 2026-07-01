use leptos::prelude::*;

use crate::shared::merge_class;

/// A prose line-length wrapper. Constrains content to a readable measure
/// (default 65ch) so long-form text stays legible regardless of container width.
///
/// Props: `measure` (CSS length or a ch-based value; `""` uses `--gl-measure`),
/// `class`/`style` (passthrough).
#[component]
pub fn Measure(
    children: Children,
    #[prop(default = "")] measure: &'static str,
    #[prop(default = "")] class: &'static str,
    #[prop(default = "")] style: &'static str,
) -> impl IntoView {
    let class_str = merge_class("gl-measure", class);

    let own_style = if measure.is_empty() {
        String::new()
    } else {
        format!("max-width: {measure};")
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