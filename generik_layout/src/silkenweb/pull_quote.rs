//! Silkenweb adapter for the `PullQuote` layout component.
//!
//! [`pull_quote`] mirrors the Leptos `PullQuote` component: a short extracted
//! quote with optional attribution — a pure typography wrapper, no layout
//! mechanics. Unlike the box-primitive and grid adapters it returns a semantic
//! [`Blockquote<D>`] (`<blockquote>`) rather than a `<div>`.

use silkenweb::dom::Dom;
#[cfg(test)]
use silkenweb::elements::html::Div;
use silkenweb::elements::html::{blockquote, footer, Blockquote};
use silkenweb::node::element::ParentElement;
use silkenweb::node::ChildNode;

use crate::shared::merge_class;

use super::helpers::{layout_div, merge_style, styled};

/// A pull-quote: a short extracted quote with optional attribution.
///
/// Renders a semantic `<blockquote>` carrying `gl-pullquote`. Its direct
/// children, in source order, are: a `div.gl-pullquote-text` wrapping the
/// `quote_children`, and — when `attribution` is `Some` — a
/// `footer.gl-pullquote-attribution` wrapping the attribution children.
/// `class`/`style` are passthroughs on the outer `<blockquote>`; an empty
/// `style` emits no `style` attribute. The Leptos equivalent is the `PullQuote`
/// component (whose quote/attribution are passed via `Quote` / `Attribution`
/// slots).
pub fn pull_quote<D>(
    class: &str,
    style: &str,
    quote_children: impl IntoIterator<Item = impl ChildNode<D>>,
    attribution: Option<impl IntoIterator<Item = impl ChildNode<D>>>,
) -> Blockquote<D>
where
    D: Dom,
{
    let class_str = merge_class("gl-pullquote", class);
    let style_str = merge_style("", style);

    let quote_div = layout_div("gl-pullquote-text", "", quote_children);

    let blockquote = styled(blockquote::<D>(), &class_str, &style_str).child(quote_div);

    match attribution {
        Some(attr) => {
            blockquote.child(styled(footer::<D>(), "gl-pullquote-attribution", "").children(attr))
        }
        None => blockquote,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use silkenweb::dom::Dry;
    use silkenweb::elements::html::div;

    /// Render a Dry blockquote to its HTML for assertion.
    fn render(el: Blockquote<Dry>) -> String {
        el.freeze().to_string()
    }

    /// A single `<div>` child typed for the Dry DOM.
    fn one() -> [Div<Dry>; 1] {
        [div::<Dry>()]
    }

    /// An absent attribution, typed so `None` infers a concrete child iterator.
    fn none() -> Option<std::iter::Empty<Div<Dry>>> {
        None
    }

    #[test]
    fn pull_quote_without_attribution_emits_blockquote_with_text_div() {
        assert_eq!(
            render(pull_quote::<Dry>("", "", one(), none())),
            r#"<blockquote class="gl-pullquote"><div class="gl-pullquote-text"><div></div></div></blockquote>"#
        );
    }

    #[test]
    fn pull_quote_with_attribution_appends_footer_after_text() {
        assert_eq!(
            render(pull_quote::<Dry>("", "", one(), Some(one()))),
            r#"<blockquote class="gl-pullquote"><div class="gl-pullquote-text"><div></div></div><footer class="gl-pullquote-attribution"><div></div></footer></blockquote>"#
        );
    }

    #[test]
    fn pull_quote_attribution_is_omitted_when_none() {
        let html = render(pull_quote::<Dry>("", "", one(), none()));
        assert!(
            !html.contains("footer"),
            "no footer when attribution is None: {html}"
        );
    }

    #[test]
    fn pull_quote_preserves_text_then_attribution_source_order() {
        let html = render(pull_quote::<Dry>("", "", one(), Some(one())));
        let text = html.find("gl-pullquote-text").unwrap();
        let attr = html.find("gl-pullquote-attribution").unwrap();
        assert!(text < attr, "quote text must precede attribution: {html}");
    }

    #[test]
    fn pull_quote_appends_multiple_children_inside_text_div() {
        assert_eq!(
            render(pull_quote::<Dry>(
                "",
                "",
                [div::<Dry>(), div::<Dry>()],
                none()
            )),
            r#"<blockquote class="gl-pullquote"><div class="gl-pullquote-text"><div></div><div></div></div></blockquote>"#
        );
    }

    #[test]
    fn pull_quote_merges_class_and_passthrough_style() {
        assert_eq!(
            render(pull_quote::<Dry>("lede", "margin: 1rem;", one(), none())),
            r#"<blockquote class="gl-pullquote lede" style="margin: 1rem;"><div class="gl-pullquote-text"><div></div></div></blockquote>"#
        );
    }

    #[test]
    fn pull_quote_omits_style_attribute_when_empty() {
        let html = render(pull_quote::<Dry>("", "", one(), Some(one())));
        assert!(
            !html.contains(" style="),
            "no style attribute with empty passthrough: {html}"
        );
    }

    #[test]
    fn pull_quote_re_exports_resolve_via_silkenweb_module() {
        let el: Blockquote<Dry> = crate::silkenweb::pull_quote("lede", "", one(), Some(one()));
        assert_eq!(
            render(el),
            r#"<blockquote class="gl-pullquote lede"><div class="gl-pullquote-text"><div></div></div><footer class="gl-pullquote-attribution"><div></div></footer></blockquote>"#
        );
    }
}
