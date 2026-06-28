use leptos::prelude::*;

use crate::shared::{merge_class, Gutter};

/// A vertical flex stack with a configurable gap (`gl-stack` + `gl-gap-*`).
///
/// Props: `gap` (default `Md`), `class`/`style` (passthrough).
#[component]
pub fn Stack(
    children: Children,
    #[prop(optional)] gap: Gutter,
    #[prop(default = "")] class: &'static str,
    #[prop(default = "")] style: &'static str,
) -> impl IntoView {
    let base = merge_class("gl-stack", &format!("gl-gap{}", gap.class_suffix()));
    let class_str = merge_class(&base, class);

    view! {
        <div class=class_str style=style>{children()}</div>
    }
}