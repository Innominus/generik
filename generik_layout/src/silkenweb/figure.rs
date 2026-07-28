//! Silkenweb adapter for the `Figure` layout component.
//!
//! [`figure`] mirrors the Leptos `Figure` component: an editorial figure with
//! required media and an optional caption. Unlike the box-primitive and grid
//! adapters it returns a semantic [`Figure<D>`] (`<figure>`) rather than a
//! `<div>`, wrapping the media in `div.gl-figure-media` and the optional caption
//! in `figcaption.gl-figure-caption`, preserving media-first source order.
//!
//! [`Figure<D>`]: silkenweb::elements::html::Figure

use silkenweb::dom::Dom;
use silkenweb::elements::html::{figcaption, figure as html_figure, Figure};
use silkenweb::node::element::ParentElement;
use silkenweb::node::ChildNode;

use crate::shared::merge_class;

use super::helpers::{layout_div, merge_style, styled};

/// An editorial figure: required media with an optional caption below.
///
/// The media is wrapped in a `div.gl-figure-media`; when a caption is present it
/// is wrapped in a `figcaption.gl-figure-caption`, placed after the media to
/// preserve the media-first source order of the Leptos component. `class`/
/// `style` are passthroughs on the outer `<figure>`; an empty `style` emits no
/// `style` attribute. The Leptos equivalent is the `Figure` component (whose
/// media/caption are passed via `FigureMedia` / `FigureCaption` slots).
pub fn figure<D>(
    media: impl IntoIterator<Item = impl ChildNode<D>>,
    caption: Option<impl IntoIterator<Item = impl ChildNode<D>>>,
    class: &str,
    style: &str,
) -> Figure<D>
where
    D: Dom,
{
    let class_str = merge_class("gl-figure", class);
    let style_str = merge_style("", style);

    let media_div = layout_div("gl-figure-media", "", media);
    let fig = styled(html_figure::<D>(), &class_str, &style_str).child(media_div);

    match caption {
        Some(caption) => {
            let caption_el = styled(figcaption::<D>(), "gl-figure-caption", "").children(caption);
            fig.child(caption_el)
        }
        None => fig,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use silkenweb::dom::Dry;
    use silkenweb::elements::html::{div, Div};

    /// Render a Dry figure to its HTML for assertion.
    fn render(el: Figure<Dry>) -> String {
        el.freeze().to_string()
    }

    /// A single `<div>` child typed for the Dry DOM.
    fn one() -> [Div<Dry>; 1] {
        [div::<Dry>()]
    }

    /// An absent caption, typed so `None` infers a concrete child iterator.
    fn none() -> Option<std::iter::Empty<Div<Dry>>> {
        None
    }

    #[test]
    fn figure_media_only_emits_semantic_figure_with_media_div() {
        assert_eq!(
            render(figure::<Dry>(one(), none(), "", "")),
            r#"<figure class="gl-figure"><div class="gl-figure-media"><div></div></div></figure>"#
        );
    }

    #[test]
    fn figure_with_caption_emits_figcaption_after_media() {
        assert_eq!(
            render(figure::<Dry>(one(), Some(one()), "", "")),
            r#"<figure class="gl-figure"><div class="gl-figure-media"><div></div></div><figcaption class="gl-figure-caption"><div></div></figcaption></figure>"#
        );
    }

    #[test]
    fn figure_caption_is_omitted_when_none() {
        let html = render(figure::<Dry>(one(), none(), "", ""));
        assert!(
            !html.contains("figcaption"),
            "no figcaption when caption is None: {html}"
        );
    }

    #[test]
    fn figure_preserves_media_first_source_order() {
        let html = render(figure::<Dry>(one(), Some(one()), "", ""));
        let media_pos = html.find("gl-figure-media").unwrap();
        let caption_pos = html.find("gl-figure-caption").unwrap();
        assert!(
            media_pos < caption_pos,
            "media must precede caption: {html}"
        );
    }

    #[test]
    fn figure_merges_class_and_passthrough_style() {
        assert_eq!(
            render(figure::<Dry>(one(), none(), "photo", "margin: 1rem;")),
            r#"<figure class="gl-figure photo" style="margin: 1rem;"><div class="gl-figure-media"><div></div></div></figure>"#
        );
    }

    #[test]
    fn figure_omits_style_attribute_when_empty() {
        let html = render(figure::<Dry>(one(), Some(one()), "", ""));
        assert!(
            !html.contains(" style="),
            "no style attribute with empty passthrough: {html}"
        );
    }

    #[test]
    fn figure_re_exports_resolve_via_silkenweb_module() {
        let el: Figure<Dry> = crate::silkenweb::figure(one(), Some(one()), "photo", "");
        assert_eq!(
            render(el),
            r#"<figure class="gl-figure photo"><div class="gl-figure-media"><div></div></div><figcaption class="gl-figure-caption"><div></div></figcaption></figure>"#
        );
    }
}
