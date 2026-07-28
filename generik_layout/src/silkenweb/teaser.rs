//! Silkenweb adapter for the `Teaser` layout component.
//!
//! [`teaser`] mirrors the Leptos `Teaser` component: a single editorial teaser
//! card — a media placeholder, an eyebrow, a title, and optional meta. The
//! atomic unit of a "more from this issue" or "recent features" row; wrap
//! several in a [`grid`](super::grid) with [`col`](super::col)s for a 2- or
//! 3-up layout.
//!
//! Unlike the box-primitive and grid adapters it returns a semantic
//! [`Article<D>`] (`<article>`) rather than a `<div>`.

use silkenweb::dom::Dom;
use silkenweb::elements::html::{article, span, Article};
use silkenweb::node::element::{ParentElement, TextParentElement};

use crate::shared::merge_class;

use super::helpers::{bare_div, merge_style, styled};

/// A single editorial teaser card: a media placeholder, an eyebrow, a title,
/// and optional meta.
///
/// Renders a semantic `<article>` carrying `gl-teaser`. Its direct children, in
/// source order, are: a `div.gl-teaser-media` placeholder (its class merged with
/// the `media` passthrough, e.g. a gradient or bg-image class; empty `media`
/// yields a plain placeholder), a `gl-teaser-eyebrow` `<span>`, a
/// `gl-teaser-title` `<span>`, and — when `meta` is `Some` — a `gl-teaser-meta`
/// `<span>`. `class`/`style` are passthroughs on the outer `<article>`; an empty
/// `style` emits no `style` attribute. The Leptos equivalent is the `Teaser`
/// component.
pub fn teaser<D>(
    eyebrow: &str,
    title: &str,
    media: &str,
    meta: Option<&str>,
    class: &str,
    style: &str,
) -> Article<D>
where
    D: Dom,
{
    let class_str = merge_class("gl-teaser", class);
    let style_str = merge_style("", style);

    let media_div = bare_div(&merge_class("gl-teaser-media", media), "");
    let eyebrow_span = styled(span::<D>(), "gl-teaser-eyebrow", "").text(eyebrow);
    let title_span = styled(span::<D>(), "gl-teaser-title", "").text(title);

    let article = styled(article::<D>(), &class_str, &style_str)
        .child(media_div)
        .child(eyebrow_span)
        .child(title_span);

    match meta {
        Some(m) => article.child(styled(span::<D>(), "gl-teaser-meta", "").text(m)),
        None => article,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use silkenweb::dom::Dry;

    /// Render a Dry article to its HTML for assertion.
    fn render(el: Article<Dry>) -> String {
        el.freeze().to_string()
    }

    #[test]
    fn teaser_without_meta_emits_article_media_eyebrow_title() {
        assert_eq!(
            render(teaser::<Dry>("Feature", "The Big Story", "", None, "", "")),
            r#"<article class="gl-teaser"><div class="gl-teaser-media"></div><span class="gl-teaser-eyebrow">Feature</span><span class="gl-teaser-title">The Big Story</span></article>"#
        );
    }

    #[test]
    fn teaser_with_meta_appends_meta_span_after_title() {
        assert_eq!(
            render(teaser::<Dry>(
                "Feature",
                "The Big Story",
                "",
                Some("8 min"),
                "",
                ""
            )),
            r#"<article class="gl-teaser"><div class="gl-teaser-media"></div><span class="gl-teaser-eyebrow">Feature</span><span class="gl-teaser-title">The Big Story</span><span class="gl-teaser-meta">8 min</span></article>"#
        );
    }

    #[test]
    fn teaser_media_class_merges_with_gl_teaser_media() {
        assert_eq!(
            render(teaser::<Dry>(
                "Feature",
                "The Big Story",
                "gl-gradient-warm",
                None,
                "",
                ""
            )),
            r#"<article class="gl-teaser"><div class="gl-teaser-media gl-gradient-warm"></div><span class="gl-teaser-eyebrow">Feature</span><span class="gl-teaser-title">The Big Story</span></article>"#
        );
    }

    #[test]
    fn teaser_meta_is_omitted_when_none() {
        let html = render(teaser::<Dry>("Feature", "The Big Story", "", None, "", ""));
        assert!(
            !html.contains("gl-teaser-meta"),
            "no meta span when meta is None: {html}"
        );
    }

    #[test]
    fn teaser_preserves_media_eyebrow_title_meta_source_order() {
        let html = render(teaser::<Dry>(
            "Feature",
            "The Big Story",
            "",
            Some("8 min"),
            "",
            "",
        ));
        let media = html.find("gl-teaser-media").unwrap();
        let eyebrow = html.find("gl-teaser-eyebrow").unwrap();
        let title = html.find("gl-teaser-title").unwrap();
        let meta = html.find("gl-teaser-meta").unwrap();
        assert!(
            media < eyebrow && eyebrow < title && title < meta,
            "media, eyebrow, title, meta order required: {html}"
        );
    }

    #[test]
    fn teaser_merges_class_and_passthrough_style() {
        assert_eq!(
            render(teaser::<Dry>(
                "Feature",
                "The Big Story",
                "",
                None,
                "hero",
                "margin: 1rem;"
            )),
            r#"<article class="gl-teaser hero" style="margin: 1rem;"><div class="gl-teaser-media"></div><span class="gl-teaser-eyebrow">Feature</span><span class="gl-teaser-title">The Big Story</span></article>"#
        );
    }

    #[test]
    fn teaser_omits_style_attribute_when_empty() {
        let html = render(teaser::<Dry>(
            "Feature",
            "The Big Story",
            "",
            Some("8 min"),
            "",
            "",
        ));
        assert!(
            !html.contains(" style="),
            "no style attribute with empty passthrough: {html}"
        );
    }

    #[test]
    fn teaser_re_exports_resolve_via_silkenweb_module() {
        let el: Article<Dry> =
            crate::silkenweb::teaser("Feature", "The Big Story", "", None, "hero", "");
        assert_eq!(
            render(el),
            r#"<article class="gl-teaser hero"><div class="gl-teaser-media"></div><span class="gl-teaser-eyebrow">Feature</span><span class="gl-teaser-title">The Big Story</span></article>"#
        );
    }
}
