use leptos::prelude::*;

use crate::shared::merge_class;

/// A data-poster metric: a big display figure, a small mono label, and an
/// optional delta (trend indicator). A richer `Stat` for editorial data
/// posters where the change matters as much as the value.
///
/// Props: `value` (the big number, e.g. "14" or "1.2k"), `label` (the caption,
/// rendered as an eyebrow), `delta` (optional, e.g. "+12%" or "-3%" — rendered
/// in mono, accent-colored when it begins with ASCII `+`, ink-muted otherwise),
/// `class`/`style` (passthrough).
#[component]
pub fn Metric(
    value: &'static str,
    label: &'static str,
    #[prop(optional)] delta: Option<&'static str>,
    #[prop(default = "")] class: &'static str,
    #[prop(default = "")] style: &'static str,
) -> impl IntoView {
    let class_str = merge_class("gl-metric", class);

    let delta_view = delta.map(|d| {
        let color: &'static str = if d.starts_with('+') {
            "color: var(--gl-accent)"
        } else {
            "color: var(--gl-ink-muted)"
        };
        view! { <span class="gl-metric-delta" style=color>{d}</span> }
    });

    view! {
        <div class=class_str style=style>
            <span class="gl-metric-value">{value}</span>
            {delta_view}
            <span class="gl-eyebrow">{label}</span>
        </div>
    }
}