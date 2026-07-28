//! Silkenweb adapter for the `StickyHeader` layout component.
//!
//! [`sticky_header`] mirrors the Leptos `StickyHeader` component: a fixed
//! running header that sticks to the top of the viewport on scroll — a wordmark
//! (publication name), optional section navigation, and an optional issue/date
//! line. The running header of a magazine.
//!
//! Unlike the box-primitive and grid adapters it returns a semantic
//! [`Header<D>`] (`<header>`) rather than a `<div>`.

use silkenweb::dom::Dom;
use silkenweb::elements::html::{header, nav, span, Header, Span};
use silkenweb::node::element::{ParentElement, TextParentElement};

use crate::shared::merge_class;

use super::helpers::{merge_style, styled};

/// A fixed-position running header: a wordmark, optional section navigation,
/// and an optional issue/date line.
///
/// Renders a semantic `<header>` carrying `gl-sticky-header`. Its direct
/// children, in source order, are: a `gl-sticky-header-wordmark` `<span>`, an
/// optional `nav.gl-sticky-header-nav`, and an optional
/// `gl-sticky-header-issue` `<span>`. The nav holds one `<span>` per label; from
/// the second label onward each span's text is prefixed with a middot (`" · "`)
/// so the items read as an inline `Features · Culture · Index` list. `class`/
/// `style` are passthroughs on the outer `<header>`; an empty `style` emits no
/// `style` attribute. The Leptos equivalent is the `StickyHeader` component.
pub fn sticky_header<D>(
    wordmark: &str,
    nav_items: Option<&[&str]>,
    issue: Option<&str>,
    class: &str,
    style: &str,
) -> Header<D>
where
    D: Dom,
{
    let class_str = merge_class("gl-sticky-header", class);
    let style_str = merge_style("", style);

    let wordmark_span = styled(span::<D>(), "gl-sticky-header-wordmark", "").text(wordmark);

    let header = styled(header::<D>(), &class_str, &style_str).child(wordmark_span);

    let header = match nav_items {
        Some(items) => {
            let nav_spans: Vec<Span<D>> = items
                .iter()
                .copied()
                .enumerate()
                .map(|(i, label)| {
                    let text = if i > 0 {
                        format!(" · {label}")
                    } else {
                        label.to_string()
                    };
                    span::<D>().text(text)
                })
                .collect();
            header.child(styled(nav::<D>(), "gl-sticky-header-nav", "").children(nav_spans))
        }
        None => header,
    };

    match issue {
        Some(i) => header.child(styled(span::<D>(), "gl-sticky-header-issue", "").text(i)),
        None => header,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use silkenweb::dom::Dry;

    /// Render a Dry header to its HTML for assertion.
    fn render(el: Header<Dry>) -> String {
        el.freeze().to_string()
    }

    #[test]
    fn sticky_header_wordmark_only_emits_header_with_wordmark() {
        assert_eq!(
            render(sticky_header::<Dry>("The Review", None, None, "", "")),
            r#"<header class="gl-sticky-header"><span class="gl-sticky-header-wordmark">The Review</span></header>"#
        );
    }

    #[test]
    fn sticky_header_single_nav_item_has_no_middot_prefix() {
        assert_eq!(
            render(sticky_header::<Dry>("The Review", Some(&["Features"]), None, "", "")),
            r#"<header class="gl-sticky-header"><span class="gl-sticky-header-wordmark">The Review</span><nav class="gl-sticky-header-nav"><span>Features</span></nav></header>"#
        );
    }

    #[test]
    fn sticky_header_nav_prefixes_second_item_onward_with_middot() {
        assert_eq!(
            render(sticky_header::<Dry>(
                "The Review",
                Some(&["Features", "Culture", "Index"]),
                None,
                "",
                ""
            )),
            r#"<header class="gl-sticky-header"><span class="gl-sticky-header-wordmark">The Review</span><nav class="gl-sticky-header-nav"><span>Features</span><span> · Culture</span><span> · Index</span></nav></header>"#
        );
    }

    #[test]
    fn sticky_header_nav_is_omitted_when_none() {
        let html = render(sticky_header::<Dry>("The Review", None, None, "", ""));
        assert!(
            !html.contains("gl-sticky-header-nav"),
            "no nav when nav is None: {html}"
        );
    }

    #[test]
    fn sticky_header_with_issue_appends_issue_span() {
        assert_eq!(
            render(sticky_header::<Dry>("The Review", None, Some("Issue 14"), "", "")),
            r#"<header class="gl-sticky-header"><span class="gl-sticky-header-wordmark">The Review</span><span class="gl-sticky-header-issue">Issue 14</span></header>"#
        );
    }

    #[test]
    fn sticky_header_issue_is_omitted_when_none() {
        let html = render(sticky_header::<Dry>("The Review", None, None, "", ""));
        assert!(
            !html.contains("gl-sticky-header-issue"),
            "no issue span when issue is None: {html}"
        );
    }

    #[test]
    fn sticky_header_preserves_wordmark_nav_issue_source_order() {
        let html = render(sticky_header::<Dry>(
            "The Review",
            Some(&["Features", "Culture"]),
            Some("Issue 14"),
            "",
            "",
        ));
        let wordmark = html.find("gl-sticky-header-wordmark").unwrap();
        let nav = html.find("gl-sticky-header-nav").unwrap();
        let issue = html.find("gl-sticky-header-issue").unwrap();
        assert!(
            wordmark < nav && nav < issue,
            "wordmark, nav, issue order required: {html}"
        );
    }

    #[test]
    fn sticky_header_merges_class_and_passthrough_style() {
        assert_eq!(
            render(sticky_header::<Dry>("The Review", None, None, "site", "top: 0;")),
            r#"<header class="gl-sticky-header site" style="top: 0;"><span class="gl-sticky-header-wordmark">The Review</span></header>"#
        );
    }

    #[test]
    fn sticky_header_omits_style_attribute_when_empty() {
        let html = render(sticky_header::<Dry>(
            "The Review",
            Some(&["Features"]),
            Some("Issue 14"),
            "",
            "",
        ));
        assert!(
            !html.contains(" style="),
            "no style attribute with empty passthrough: {html}"
        );
    }

    #[test]
    fn sticky_header_re_exports_resolve_via_silkenweb_module() {
        let el: Header<Dry> =
            crate::silkenweb::sticky_header("The Review", Some(&["Features", "Culture"]), None, "site", "");
        assert_eq!(
            render(el),
            r#"<header class="gl-sticky-header site"><span class="gl-sticky-header-wordmark">The Review</span><nav class="gl-sticky-header-nav"><span>Features</span><span> · Culture</span></nav></header>"#
        );
    }
}