use leptos::prelude::*;

use crate::shared::merge_class;

/// Flex-centers its children both axes with `min-height: 100%`. Use for Swiss hero
/// sections. For full-viewport height, pass `class="min-h-screen"` or set the parent's height.
///
/// Props: `class`/`style` (passthrough).
#[component]
pub fn Center(
    children: Children,
    #[prop(default = "")] class: &'static str,
    #[prop(default = "")] style: &'static str,
) -> impl IntoView {
    let class_str = merge_class("gl-center", class);

    view! {
        <div class=class_str style=style>{children()}</div>
    }
}