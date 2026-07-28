//! Silkenweb adapter for the `MediaObject` layout component.
//!
//! [`media_object`] mirrors the Leptos `MediaObject` component: image-on-one-
//! side, text-on-the-other, collapsing to stacked on mobile and side-by-side
//! from md (768px) up. The media and body are wrapped in `gl-media-media` /
//! `gl-media-body` `<div>`s; [`Side::Left`] renders media then body, [`Side::Right`]
//! renders body then media and adds the `gl-media-object-right` class. The gap
//! class is always included and, when `media_width` is non-empty,
//! `--gl-media-width: …` is prepended to the passthrough `style`.
//!
//! [`Side::Left`]: crate::shared::Side::Left
//! [`Side::Right`]: crate::shared::Side::Right

use silkenweb::dom::Dom;
use silkenweb::elements::html::Div;
use silkenweb::node::ChildNode;

use crate::shared::{merge_class, Gutter, Side};

use super::helpers::{layout_div, merge_style};

/// Image-on-one-side, text-on-the-other. Collapses to stacked on mobile,
/// side-by-side from md (768px) up.
///
/// `side` (default [`Side::Left`]) controls the visual side and source order:
/// `Left` renders `gl-media-media` then `gl-media-body`, `Right` renders body
/// then media and adds the `gl-media-object-right` class. `gap` (default
/// [`Gutter::Md`]) is always included as a `gl-gap-*` class; `media_width` (a
/// CSS length for the media column; `""` uses the `--gl-media-width` token) is
/// prepended to `style` as `--gl-media-width: …;` when non-empty. `class`/
/// `style` are passthroughs; an empty `style` (with no `media_width`) emits no
/// `style` attribute. The Leptos equivalent is the `MediaObject` component
/// (whose media/body are passed via `Media` / `Body` slots).
///
/// [`Side::Left`]: crate::shared::Side::Left
/// [`Gutter::Md`]: crate::shared::Gutter::Md
pub fn media_object<D>(
    media: impl IntoIterator<Item = impl ChildNode<D>>,
    body: impl IntoIterator<Item = impl ChildNode<D>>,
    side: Side,
    gap: Gutter,
    media_width: &str,
    class: &str,
    style: &str,
) -> Div<D>
where
    D: Dom,
{
    let mut base = "gl-media-object".to_string();
    if side == Side::Right {
        base = merge_class(&base, "gl-media-object-right");
    }
    base = merge_class(&base, &format!("gl-gap{}", gap.class_suffix()));
    let class_str = merge_class(&base, class);

    let own = if media_width.is_empty() {
        String::new()
    } else {
        format!("--gl-media-width: {media_width};")
    };
    let style_str = merge_style(&own, style);

    let media_div = layout_div("gl-media-media", "", media);
    let body_div = layout_div("gl-media-body", "", body);

    // Source order follows the visual side: Left -> media then body,
    // Right -> body then media.
    let (first, second) = if side == Side::Right {
        (body_div, media_div)
    } else {
        (media_div, body_div)
    };

    layout_div(&class_str, &style_str, [first, second])
}

#[cfg(test)]
mod tests {
    use super::*;
    use silkenweb::dom::Dry;
    use silkenweb::elements::html::{div, Div};

    /// Render a Dry div to its HTML for assertion.
    fn render(el: Div<Dry>) -> String {
        el.freeze().to_string()
    }

    /// A single `<div>` child typed for the Dry DOM.
    fn one() -> [Div<Dry>; 1] {
        [div::<Dry>()]
    }

    #[test]
    fn media_object_left_renders_media_then_body() {
        assert_eq!(
            render(media_object::<Dry>(
                one(),
                one(),
                Side::Left,
                Gutter::Md,
                "",
                "",
                ""
            )),
            r#"<div class="gl-media-object gl-gap-md"><div class="gl-media-media"><div></div></div><div class="gl-media-body"><div></div></div></div>"#
        );
    }

    #[test]
    fn media_object_right_swaps_order_and_adds_right_class() {
        assert_eq!(
            render(media_object::<Dry>(
                one(),
                one(),
                Side::Right,
                Gutter::Md,
                "",
                "",
                ""
            )),
            r#"<div class="gl-media-object gl-media-object-right gl-gap-md"><div class="gl-media-body"><div></div></div><div class="gl-media-media"><div></div></div></div>"#
        );
    }

    #[test]
    fn media_object_gap_class_changes_with_gutter() {
        assert_eq!(
            render(media_object::<Dry>(
                one(),
                one(),
                Side::Left,
                Gutter::Lg,
                "",
                "",
                ""
            )),
            r#"<div class="gl-media-object gl-gap-lg"><div class="gl-media-media"><div></div></div><div class="gl-media-body"><div></div></div></div>"#
        );
    }

    #[test]
    fn media_object_media_width_becomes_custom_property() {
        assert_eq!(
            render(media_object::<Dry>(
                one(),
                one(),
                Side::Left,
                Gutter::Md,
                "200px",
                "",
                ""
            )),
            r#"<div class="gl-media-object gl-gap-md" style="--gl-media-width: 200px;"><div class="gl-media-media"><div></div></div><div class="gl-media-body"><div></div></div></div>"#
        );
    }

    #[test]
    fn media_object_media_width_merges_with_passthrough_style() {
        assert_eq!(
            render(media_object::<Dry>(
                one(),
                one(),
                Side::Right,
                Gutter::Md,
                "200px",
                "",
                "color: red;"
            )),
            r#"<div class="gl-media-object gl-media-object-right gl-gap-md" style="--gl-media-width: 200px; color: red;"><div class="gl-media-body"><div></div></div><div class="gl-media-media"><div></div></div></div>"#
        );
    }

    #[test]
    fn media_object_passthrough_style_only() {
        assert_eq!(
            render(media_object::<Dry>(
                one(),
                one(),
                Side::Left,
                Gutter::Md,
                "",
                "",
                "color: red;"
            )),
            r#"<div class="gl-media-object gl-gap-md" style="color: red;"><div class="gl-media-media"><div></div></div><div class="gl-media-body"><div></div></div></div>"#
        );
    }

    #[test]
    fn media_object_merges_passthrough_class() {
        assert_eq!(
            render(media_object::<Dry>(
                one(),
                one(),
                Side::Left,
                Gutter::Md,
                "",
                "item",
                ""
            )),
            r#"<div class="gl-media-object gl-gap-md item"><div class="gl-media-media"><div></div></div><div class="gl-media-body"><div></div></div></div>"#
        );
    }

    #[test]
    fn media_object_omits_style_attribute_when_empty() {
        let html = render(media_object::<Dry>(
            one(),
            one(),
            Side::Left,
            Gutter::Md,
            "",
            "",
            "",
        ));
        assert!(
            !html.contains(" style="),
            "no style attribute with empty width and passthrough: {html}"
        );
    }

    #[test]
    fn media_object_re_exports_resolve_via_silkenweb_module() {
        let el: Div<Dry> =
            crate::silkenweb::media_object(one(), one(), Side::Left, Gutter::Md, "", "", "");
        assert_eq!(
            render(el),
            r#"<div class="gl-media-object gl-gap-md"><div class="gl-media-media"><div></div></div><div class="gl-media-body"><div></div></div></div>"#
        );
    }
}
