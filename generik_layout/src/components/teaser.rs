use leptos::prelude::*;

use crate::shared::merge_class;

/// A single editorial teaser card: a media block (image/placeholder), an
/// eyebrow, a title, and optional meta. The atomic unit of a "more from this
/// issue" or "recent features" row — wrap several in a `Grid` with `Col`s for
/// a 2- or 3-up layout.
///
/// Props: `eyebrow` (the category label, e.g. "Feature"), `title` (the teaser
/// title), `media` (optional `&'static str` CSS class for the media block —
/// e.g. a gradient or bg-image class; if empty, a neutral placeholder renders),
/// `meta` (optional, e.g. "8 min" or "p. 14"), `class`/`style` (passthrough).
#[component]
pub fn Teaser(
    eyebrow: &'static str,
    title: &'static str,
    #[prop(default = "")] media: &'static str,
    #[prop(optional)] meta: Option<&'static str>,
    #[prop(default = "")] class: &'static str,
    #[prop(default = "")] style: &'static str,
) -> impl IntoView {
    let class_str = merge_class("gl-teaser", class);
    let media_class = merge_class("gl-teaser-media", media);

    let meta_view = meta.map(|m| view! { <span class="gl-teaser-meta">{m}</span> });

    view! {
        <article class=class_str style=style>
            <div class=media_class></div>
            <span class="gl-teaser-eyebrow">{eyebrow}</span>
            <span class="gl-teaser-title">{title}</span>
            {meta_view}
        </article>
    }
}