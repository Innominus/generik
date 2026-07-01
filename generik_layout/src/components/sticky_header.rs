use leptos::prelude::*;

use crate::shared::merge_class;

/// A fixed-position running header that sticks to the top of the viewport on
/// scroll. Contains a wordmark (publication name), optional section navigation,
/// and an optional issue/date line — the running header of a magazine.
///
/// Renders a `<header>` with `position: sticky; top: 0; z-index: 50` and a
/// paper background, bounded by a thin rule beneath. The wordmark uses the
/// display font at a small size; the nav is a mono inline list.
///
/// Props: `wordmark` (the publication name, rendered small in display font),
/// `nav` (optional slice of section labels — rendered as inline mono links
/// separated by middots), `issue` (optional, e.g. "Issue 14 — Jun 2026"),
/// `class`/`style` (passthrough).
#[component]
pub fn StickyHeader(
    wordmark: &'static str,
    #[prop(optional)] nav: Option<&'static [&'static str]>,
    #[prop(optional)] issue: Option<&'static str>,
    #[prop(default = "")] class: &'static str,
    #[prop(default = "")] style: &'static str,
) -> impl IntoView {
    let class_str = merge_class("gl-sticky-header", class);

    let nav_view = nav.map(|items| {
        view! {
            <nav class="gl-sticky-header-nav">
                {items.iter().copied().enumerate().map(|(i, label)| view! {
                    <span>{if i > 0 { " · " } else { "" }}{label}</span>
                }).collect::<Vec<_>>()}
            </nav>
        }
    });

    let issue_view = issue.map(|i| view! { <span class="gl-sticky-header-issue">{i}</span> });

    view! {
        <header class=class_str style=style>
            <span class="gl-sticky-header-wordmark">{wordmark}</span>
            {nav_view}
            {issue_view}
        </header>
    }
}