use leptos::prelude::*;

use crate::components::rule::Rule;
use crate::shared::RuleVariant;
use crate::shared::merge_class;

/// A publication masthead: the title block at the top of an editorial page.
/// Renders the publication name in the display font, with an issue/date line
/// below in mono, and an optional tagline. A thick `Rule` sits beneath.
///
/// This is the "nameplate" of the page — the first thing the eye lands on.
///
/// Props: `name` (publication title), `issue` (the issue/date line, e.g.
/// "Issue 14 — June 2026"), `tagline` (optional), `class`/`style` (passthrough).
#[component]
pub fn Masthead(
    name: &'static str,
    issue: &'static str,
    #[prop(optional)] tagline: Option<&'static str>,
    #[prop(default = "")] class: &'static str,
    #[prop(default = "")] style: &'static str,
) -> impl IntoView {
    let class_str = merge_class("gl-masthead", class);

    let tagline_view = tagline.map(|t| view! { <span class="gl-meta">{t}</span> });

    view! {
        <div class=class_str style=style>
            <h1 class="gl-masthead-title">{name}</h1>
            <div class="gl-masthead-meta">
                <span class="gl-meta">{issue}</span>
                {tagline_view}
            </div>
            <Rule variant=RuleVariant::Thick/>
        </div>
    }
}