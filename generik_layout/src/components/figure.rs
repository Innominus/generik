use leptos::prelude::*;

use crate::shared::merge_class;

/// Slot for the media content of a `Figure` (image, video, etc.).
#[slot]
pub struct FigureMedia {
    children: Children,
}

/// Slot for the caption of a `Figure`. Optional.
#[slot]
pub struct FigureCaption {
    children: Children,
}

/// An editorial figure: media with an optional caption below.
/// Renders a semantic `<figure>` with `<figcaption>` when the caption slot is present.
///
/// Props: `class`/`style` (passthrough on the `<figure>`).
#[component]
pub fn Figure(
    media_slot: FigureMedia,
    #[prop(optional)] caption_slot: Option<FigureCaption>,
    #[prop(default = "")] class: &'static str,
    #[prop(default = "")] style: &'static str,
) -> impl IntoView {
    let class_str = merge_class("gl-figure", class);
    // The slot children are `FnOnce`; call them once up front and wrap the
    // resulting view in the `<figcaption>`. (Re-running a `FnOnce` is
    // impossible by definition, so there is no per-update work to do here.)
    let caption_view = caption_slot.map(|c| {
        view! { <figcaption class="gl-figure-caption">{(c.children)()}</figcaption> }
    });
    view! {
        <figure class=class_str style=style>
            <div class="gl-figure-media">{(media_slot.children)()}</div>
            {caption_view}
        </figure>
    }
}