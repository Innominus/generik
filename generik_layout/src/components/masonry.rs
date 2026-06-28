use leptos::prelude::*;

use crate::shared::{merge_class, Gutter};

/// A CSS Grid auto-flow dense mosaic. Children set their own
/// `grid-column: span X` / `grid-row: span Y` via their own classes or passthrough
/// styles — `Masonry` itself imposes no spans, only the track count and auto rows.
///
/// Props: `cols` (track count, default 12), `gap` (default `Md`), `class`/`style` (passthrough).
#[component]
pub fn Masonry(
    children: Children,
    #[prop(default = 12)] cols: usize,
    #[prop(optional)] gap: Gutter,
    #[prop(default = "")] class: &'static str,
    #[prop(default = "")] style: &'static str,
) -> impl IntoView {
    let cols = cols.max(1);
    let base = merge_class("gl-masonry", &format!("gl-gap{}", gap.class_suffix()));
    let class_str = merge_class(&base, class);

    let own_style = format!("grid-template-columns: repeat({cols}, 1fr);");
    let style_str = if style.is_empty() {
        own_style
    } else {
        format!("{own_style} {style}")
    };

    view! {
        <div class=class_str style=style_str>{children()}</div>
    }
}