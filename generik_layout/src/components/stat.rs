use leptos::prelude::*;

use crate::shared::merge_class;

/// A big-number stat: a large display figure with a small mono label below.
/// The signature element of Swiss poster design — information presented as
/// typography, not as decoration.
///
/// Props: `number` (the value, rendered in the display font at display size),
/// `label` (the caption, rendered in mono as an eyebrow), `class`/`style`
/// (passthrough).
#[component]
pub fn Stat(
    number: &'static str,
    label: &'static str,
    #[prop(default = "")] class: &'static str,
    #[prop(default = "")] style: &'static str,
) -> impl IntoView {
    let class_str = merge_class("gl-stat", class);

    view! {
        <div class=class_str style=style>
            <span class="gl-stat-number">{number}</span>
            <span class="gl-eyebrow">{label}</span>
        </div>
    }
}