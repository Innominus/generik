use leptos::prelude::*;

use crate::shared::{merge_class, Gutter, Side};

/// Slot for the media portion of a `MediaObject`.
#[slot]
pub struct Media {
    children: Children,
}

/// Slot for the body (text) portion of a `MediaObject`.
#[slot]
pub struct Body {
    children: Children,
}

/// Image-on-one-side, text-on-the-other. Collapses to stacked on mobile,
/// side-by-side from md (768px) up.
///
/// Props: `side` (default `Left`), `gap` (default `Md`), `media_width`
/// (CSS length for the media column; empty uses `--gl-media-width`), `class`/`style` (passthrough).
#[component]
pub fn MediaObject(
    media_slot: Media,
    body_slot: Body,
    #[prop(optional)] side: Side,
    #[prop(optional)] gap: Gutter,
    #[prop(default = "")] media_width: &'static str,
    #[prop(default = "")] class: &'static str,
    #[prop(default = "")] style: &'static str,
) -> AnyView {
    let mut base = "gl-media-object".to_string();
    if side == Side::Right {
        base = merge_class(&base, "gl-media-object-right");
    }
    base = merge_class(&base, &format!("gl-gap{}", gap.class_suffix()));
    let class_str = merge_class(&base, class);

    let style_str = if media_width.is_empty() {
        style.to_string()
    } else if style.is_empty() {
        format!("--gl-media-width: {media_width};")
    } else {
        format!("--gl-media-width: {media_width}; {style}")
    };

    if side == Side::Right {
        view! {
            <div class=class_str style=style_str>
                <div class="gl-media-body">{(body_slot.children)()}</div>
                <div class="gl-media-media">{(media_slot.children)()}</div>
            </div>
        }
        .into_any()
    } else {
        view! {
            <div class=class_str style=style_str>
                <div class="gl-media-media">{(media_slot.children)()}</div>
                <div class="gl-media-body">{(body_slot.children)()}</div>
            </div>
        }
        .into_any()
    }
}