//! Silkenweb adapter for the `Masthead` layout component.
//!
//! [`masthead`] mirrors the Leptos `Masthead` component: the publication
//! nameplate at the top of an editorial page — the display-font title, an
//! issue/date line in mono, an optional tagline, and a thick
//! [`rule`](super::rule) beneath as the masthead baseline.

use silkenweb::dom::Dom;
use silkenweb::elements::html::{div, h1, span, Div, Span};
use silkenweb::node::element::{ParentElement, TextParentElement};

use crate::shared::{merge_class, RuleVariant};

use super::helpers::{layout_div, merge_style, styled};
use super::rule::rule;

/// A publication masthead: the title block at the top of an editorial page.
///
/// Renders the publication name in the display font, with an issue/date line
/// below in mono and an optional tagline, then a thick
/// [`rule`](super::rule) as the final child — the masthead baseline. The outer
/// `gl-masthead` `<div>`'s direct children, in source order, are: an
/// `h1.gl-masthead-title` carrying `name`, a `div.gl-masthead-meta` containing a
/// `span.gl-meta` with `issue` and — when `tagline` is `Some` — a second
/// `span.gl-meta`, and finally the thick rule (`rule(RuleVariant::Thick, None,
/// "", "")`). `class`/`style` are passthroughs on the outer `<div>`; an empty
/// `style` emits no `style` attribute. The Leptos equivalent is the `Masthead`
/// component.
pub fn masthead<D>(
    name: &str,
    issue: &str,
    tagline: Option<&str>,
    class: &str,
    style: &str,
) -> Div<D>
where
    D: Dom,
{
    let class_str = merge_class("gl-masthead", class);
    let style_str = merge_style("", style);

    let title = styled(h1::<D>(), "gl-masthead-title", "").text(name);

    let mut meta_children: Vec<Span<D>> = Vec::new();
    meta_children.push(styled(span::<D>(), "gl-meta", "").text(issue));
    if let Some(t) = tagline {
        meta_children.push(styled(span::<D>(), "gl-meta", "").text(t));
    }
    let meta_div = layout_div("gl-masthead-meta", "", meta_children);

    let rule_div = rule::<D>(RuleVariant::Thick, None, "", "");

    styled(div::<D>(), &class_str, &style_str)
        .child(title)
        .child(meta_div)
        .child(rule_div)
}

#[cfg(test)]
mod tests {
    use super::*;
    use silkenweb::dom::Dry;

    /// Render a Dry div to its HTML for assertion.
    fn render(el: Div<Dry>) -> String {
        el.freeze().to_string()
    }

    /// The thick rule the masthead appends as its final child.
    const THICK_RULE: &str =
        r#"<div class="gl-rule"><span class="gl-rule-line gl-rule-thick"></span></div>"#;

    #[test]
    fn masthead_without_tagline_emits_title_meta_then_thick_rule() {
        assert_eq!(
            render(masthead::<Dry>("The Review", "Issue 14", None, "", "")),
            format!(
                r#"<div class="gl-masthead"><h1 class="gl-masthead-title">The Review</h1><div class="gl-masthead-meta"><span class="gl-meta">Issue 14</span></div>{THICK_RULE}</div>"#
            )
        );
    }

    #[test]
    fn masthead_with_tagline_appends_tagline_span_inside_meta() {
        assert_eq!(
            render(masthead::<Dry>(
                "The Review",
                "Issue 14",
                Some("Independent since 2010"),
                "",
                ""
            )),
            format!(
                r#"<div class="gl-masthead"><h1 class="gl-masthead-title">The Review</h1><div class="gl-masthead-meta"><span class="gl-meta">Issue 14</span><span class="gl-meta">Independent since 2010</span></div>{THICK_RULE}</div>"#
            )
        );
    }

    #[test]
    fn masthead_tagline_is_omitted_when_none() {
        let html = render(masthead::<Dry>("The Review", "Issue 14", None, "", ""));
        // Exactly one gl-meta span (the issue), no second one for a tagline.
        assert_eq!(
            html.matches("gl-meta").count(),
            1,
            "no tagline span when tagline is None: {html}"
        );
    }

    #[test]
    fn masthead_appends_thick_rule_as_final_child() {
        let html = render(masthead::<Dry>("The Review", "Issue 14", None, "", ""));
        assert!(
            html.ends_with(&format!("{THICK_RULE}</div>")),
            "thick rule must be the last child before the closing div: {html}"
        );
        assert!(
            html.contains("gl-rule-thick"),
            "rule must use the thick variant: {html}"
        );
    }

    #[test]
    fn masthead_preserves_title_meta_rule_source_order() {
        let html = render(masthead::<Dry>("The Review", "Issue 14", None, "", ""));
        let title = html.find("gl-masthead-title").unwrap();
        let meta = html.find("gl-masthead-meta").unwrap();
        let rule = html.find("gl-rule").unwrap();
        assert!(
            title < meta && meta < rule,
            "title, meta, rule order required: {html}"
        );
    }

    #[test]
    fn masthead_merges_class_and_passthrough_style() {
        assert_eq!(
            render(masthead::<Dry>(
                "The Review",
                "Issue 14",
                None,
                "feature",
                "margin: 1rem;"
            )),
            format!(
                r#"<div class="gl-masthead feature" style="margin: 1rem;"><h1 class="gl-masthead-title">The Review</h1><div class="gl-masthead-meta"><span class="gl-meta">Issue 14</span></div>{THICK_RULE}</div>"#
            )
        );
    }

    #[test]
    fn masthead_omits_style_attribute_when_empty() {
        let html = render(masthead::<Dry>("The Review", "Issue 14", None, "", ""));
        assert!(
            !html.contains(" style="),
            "no style attribute with empty passthrough: {html}"
        );
    }

    #[test]
    fn masthead_re_exports_resolve_via_silkenweb_module() {
        let el: Div<Dry> =
            crate::silkenweb::masthead("The Review", "Issue 14", None, "feature", "");
        assert_eq!(
            render(el),
            format!(
                r#"<div class="gl-masthead feature"><h1 class="gl-masthead-title">The Review</h1><div class="gl-masthead-meta"><span class="gl-meta">Issue 14</span></div>{THICK_RULE}</div>"#
            )
        );
    }
}
