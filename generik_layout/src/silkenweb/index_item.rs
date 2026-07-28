//! Silkenweb adapter for the `IndexItem` layout component.
//!
//! [`index_item`] mirrors the Leptos `IndexItem` component: a single entry in
//! an editorial index (contents page, "in this issue" list, related-reading
//! rail) — a number, a category eyebrow, a title, and optional meta in a tight
//! row. The atomic unit of an editorial navigation list; stack several inside a
//! [`stack`](super::primitives::stack) to build a full contents page.

use silkenweb::dom::Dom;
use silkenweb::elements::html::{span, Div};
use silkenweb::node::element::{ParentElement, TextParentElement};

use crate::shared::merge_class;

use super::helpers::{bare_div, layout_div, merge_style, styled};

/// A single entry in an editorial index: a number, a category eyebrow, a title,
/// and optional meta.
///
/// Renders a `gl-index-item` `<div>` whose direct children, in source order,
/// are: a `gl-index-item-number` `<span>`, a `div.gl-index-item-body` wrapping a
/// `gl-index-item-category` `<span>` and a `gl-index-item-title` `<span>`, and —
/// when `meta` is `Some` — a `gl-index-item-meta` `<span>`. `class`/`style` are
/// passthroughs on the outer `<div>`; an empty `style` emits no `style`
/// attribute. The Leptos equivalent is the `IndexItem` component.
pub fn index_item<D>(
    number: &str,
    category: &str,
    title: &str,
    meta: Option<&str>,
    class: &str,
    style: &str,
) -> Div<D>
where
    D: Dom,
{
    let class_str = merge_class("gl-index-item", class);
    let style_str = merge_style("", style);

    let number_span = styled(span::<D>(), "gl-index-item-number", "").text(number);
    let body_div = layout_div(
        "gl-index-item-body",
        "",
        [
            styled(span::<D>(), "gl-index-item-category", "").text(category),
            styled(span::<D>(), "gl-index-item-title", "").text(title),
        ],
    );

    let item = bare_div::<D>(&class_str, &style_str)
        .child(number_span)
        .child(body_div);

    match meta {
        Some(m) => item.child(styled(span::<D>(), "gl-index-item-meta", "").text(m)),
        None => item,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use silkenweb::dom::Dry;

    /// Render a Dry div to its HTML for assertion.
    fn render(el: Div<Dry>) -> String {
        el.freeze().to_string()
    }

    #[test]
    fn index_item_without_meta_emits_number_body_with_category_title() {
        assert_eq!(
            render(index_item::<Dry>(
                "01",
                "Feature",
                "The Big Story",
                None,
                "",
                ""
            )),
            r#"<div class="gl-index-item"><span class="gl-index-item-number">01</span><div class="gl-index-item-body"><span class="gl-index-item-category">Feature</span><span class="gl-index-item-title">The Big Story</span></div></div>"#
        );
    }

    #[test]
    fn index_item_with_meta_appends_meta_span_after_body() {
        assert_eq!(
            render(index_item::<Dry>(
                "01",
                "Feature",
                "The Big Story",
                Some("p. 14"),
                "",
                ""
            )),
            r#"<div class="gl-index-item"><span class="gl-index-item-number">01</span><div class="gl-index-item-body"><span class="gl-index-item-category">Feature</span><span class="gl-index-item-title">The Big Story</span></div><span class="gl-index-item-meta">p. 14</span></div>"#
        );
    }

    #[test]
    fn index_item_meta_is_omitted_when_none() {
        let html = render(index_item::<Dry>(
            "01",
            "Feature",
            "The Big Story",
            None,
            "",
            "",
        ));
        assert!(
            !html.contains("gl-index-item-meta"),
            "no meta span when meta is None: {html}"
        );
    }

    #[test]
    fn index_item_preserves_number_body_meta_source_order() {
        let html = render(index_item::<Dry>(
            "01",
            "Feature",
            "The Big Story",
            Some("p. 14"),
            "",
            "",
        ));
        let number = html.find("gl-index-item-number").unwrap();
        let body = html.find("gl-index-item-body").unwrap();
        let meta = html.find("gl-index-item-meta").unwrap();
        assert!(
            number < body && body < meta,
            "number, body, meta order required: {html}"
        );
    }

    #[test]
    fn index_item_body_preserves_category_then_title_order() {
        let html = render(index_item::<Dry>(
            "01",
            "Feature",
            "The Big Story",
            None,
            "",
            "",
        ));
        let category = html.find("gl-index-item-category").unwrap();
        let title = html.find("gl-index-item-title").unwrap();
        assert!(
            category < title,
            "category must precede title inside the body: {html}"
        );
    }

    #[test]
    fn index_item_merges_class_and_passthrough_style() {
        assert_eq!(
            render(index_item::<Dry>(
                "01",
                "Feature",
                "The Big Story",
                None,
                "entry",
                "padding: 0;"
            )),
            r#"<div class="gl-index-item entry" style="padding: 0;"><span class="gl-index-item-number">01</span><div class="gl-index-item-body"><span class="gl-index-item-category">Feature</span><span class="gl-index-item-title">The Big Story</span></div></div>"#
        );
    }

    #[test]
    fn index_item_omits_style_attribute_when_empty() {
        let html = render(index_item::<Dry>(
            "01",
            "Feature",
            "The Big Story",
            Some("p. 14"),
            "",
            "",
        ));
        assert!(
            !html.contains(" style="),
            "no style attribute with empty passthrough: {html}"
        );
    }

    #[test]
    fn index_item_re_exports_resolve_via_silkenweb_module() {
        let el: Div<Dry> =
            crate::silkenweb::index_item("01", "Feature", "The Big Story", None, "entry", "");
        assert_eq!(
            render(el),
            r#"<div class="gl-index-item entry"><span class="gl-index-item-number">01</span><div class="gl-index-item-body"><span class="gl-index-item-category">Feature</span><span class="gl-index-item-title">The Big Story</span></div></div>"#
        );
    }
}
