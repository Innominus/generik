use leptos::prelude::*;

use crate::shared::{merge_class, AlignItems, Gutter, JustifyContent};

/// A horizontal flex that wraps, with vertical alignment and justification.
/// Use for groups of buttons, tags, chips, inline form controls.
///
/// Props: `gap` (default `Md`), `align` (`AlignItems`, default `Center`),
/// `justify` (`JustifyContent`, default `Start`), `class`/`style` (passthrough).
///
/// `align`/`justify` are emitted as inline `align-items`/`justify-content` styles,
/// merged with any passthrough `style`. This keeps the CSS small (no per-value
/// classes) while still being overridable per instance.
#[component]
pub fn Cluster(
    children: Children,
    #[prop(optional)] gap: Gutter,
    #[prop(optional)] align: AlignItems,
    #[prop(optional)] justify: JustifyContent,
    #[prop(default = "")] class: &'static str,
    #[prop(default = "")] style: &'static str,
) -> impl IntoView {
    let base = merge_class("gl-cluster", &format!("gl-gap{}", gap.class_suffix()));
    let class_str = merge_class(&base, class);

    let inline = format!(
        "align-items: {}; justify-content: {};",
        align.css_value(),
        justify.css_value()
    );
    let style_str = if style.is_empty() {
        inline
    } else {
        format!("{inline} {style}")
    };

    view! {
        <div class=class_str style=style_str>{children()}</div>
    }
}