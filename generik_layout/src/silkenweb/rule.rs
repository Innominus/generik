//! Silkenweb adapter for the `Rule` layout component.
//!
//! [`rule`] mirrors the Leptos `Rule` component: a horizontal rule with
//! thickness variants and an optional inline label. It keeps the Leptos
//! component's semantic `<div>` (not `<hr>`) so the label can sit inline on the
//! line. The outer `gl-rule` `<div>` always starts with an empty
//! `gl-rule-line gl-rule-{thin|thick|dotted}` `<span>`; when a label is present
//! a `gl-rule-label` `<span>` is followed by a second identical line span,
//! breaking the line around the label.

use silkenweb::dom::Dom;
use silkenweb::elements::html::{span, Div, Span};
use silkenweb::node::element::TextParentElement;

use crate::shared::{merge_class, RuleVariant};

use super::helpers::{layout_div, merge_style, styled};

/// A horizontal rule with thickness variants and an optional inline label.
///
/// Renders a `<hr>`-equivalent `<div>` (so it can carry a label child) styled
/// via the `gl-rule` class. The outer `<div>` always contains an empty
/// `gl-rule-line gl-rule-{variant}` `<span>`; when `label` is `Some` a
/// `gl-rule-label` `<span>` carrying the label text is followed by a second
/// identical line span, so the label sits inline on the broken line — the
/// classic editorial section-break with a centered eyebrow ("FEATURE",
/// "ISSUE 14"). `class`/`style` are passthroughs on the outer `<div>`; an empty
/// `style` emits no `style` attribute. The Leptos equivalent is the `Rule`
/// component.
pub fn rule<D>(variant: RuleVariant, label: Option<&str>, class: &str, style: &str) -> Div<D>
where
    D: Dom,
{
    let variant_class = rule_variant_class(variant);
    let line_class = format!("gl-rule-line {variant_class}");
    let class_str = merge_class("gl-rule", class);
    let style_str = merge_style("", style);

    let mut children: Vec<Span<D>> = Vec::new();
    children.push(styled(span::<D>(), &line_class, ""));
    if let Some(label) = label {
        children.push(styled(span::<D>(), "gl-rule-label", "").text(label));
        children.push(styled(span::<D>(), &line_class, ""));
    }

    layout_div(&class_str, &style_str, children)
}

/// Returns the CSS modifier class for a [`RuleVariant`] (`Thin` -> `gl-rule-thin`,
/// etc.).
fn rule_variant_class(v: RuleVariant) -> &'static str {
    match v {
        RuleVariant::Thin => "gl-rule-thin",
        RuleVariant::Thick => "gl-rule-thick",
        RuleVariant::Dotted => "gl-rule-dotted",
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
    fn variant_class_mapping() {
        assert_eq!(rule_variant_class(RuleVariant::Thin), "gl-rule-thin");
        assert_eq!(rule_variant_class(RuleVariant::Thick), "gl-rule-thick");
        assert_eq!(rule_variant_class(RuleVariant::Dotted), "gl-rule-dotted");
    }

    #[test]
    fn rule_default_thin_no_label_emits_single_line_span() {
        assert_eq!(
            render(rule::<Dry>(RuleVariant::Thin, None, "", "")),
            r#"<div class="gl-rule"><span class="gl-rule-line gl-rule-thin"></span></div>"#
        );
    }

    #[test]
    fn rule_thick_no_label_uses_thick_modifier() {
        assert_eq!(
            render(rule::<Dry>(RuleVariant::Thick, None, "", "")),
            r#"<div class="gl-rule"><span class="gl-rule-line gl-rule-thick"></span></div>"#
        );
    }

    #[test]
    fn rule_dotted_no_label_uses_dotted_modifier() {
        assert_eq!(
            render(rule::<Dry>(RuleVariant::Dotted, None, "", "")),
            r#"<div class="gl-rule"><span class="gl-rule-line gl-rule-dotted"></span></div>"#
        );
    }

    #[test]
    fn rule_with_label_breaks_line_around_label() {
        assert_eq!(
            render(rule::<Dry>(RuleVariant::Thin, Some("FEATURE"), "", "")),
            r#"<div class="gl-rule"><span class="gl-rule-line gl-rule-thin"></span><span class="gl-rule-label">FEATURE</span><span class="gl-rule-line gl-rule-thin"></span></div>"#
        );
    }

    #[test]
    fn rule_label_inherits_variant_on_second_line_span() {
        let html = render(rule::<Dry>(RuleVariant::Dotted, Some("ISSUE 14"), "", ""));
        assert_eq!(
            html,
            r#"<div class="gl-rule"><span class="gl-rule-line gl-rule-dotted"></span><span class="gl-rule-label">ISSUE 14</span><span class="gl-rule-line gl-rule-dotted"></span></div>"#
        );
    }

    #[test]
    fn rule_preserves_line_label_line_source_order() {
        let html = render(rule::<Dry>(RuleVariant::Thin, Some("FEATURE"), "", ""));
        let line1 = html.find("gl-rule-line").unwrap();
        let label = html.find("gl-rule-label").unwrap();
        let line2 = html.rfind("gl-rule-line").unwrap();
        assert!(
            line1 < label && label < line2,
            "line, label, line order required: {html}"
        );
    }

    #[test]
    fn rule_merges_passthrough_class() {
        assert_eq!(
            render(rule::<Dry>(RuleVariant::Thin, None, "section", "")),
            r#"<div class="gl-rule section"><span class="gl-rule-line gl-rule-thin"></span></div>"#
        );
    }

    #[test]
    fn rule_merges_passthrough_style() {
        assert_eq!(
            render(rule::<Dry>(RuleVariant::Thin, None, "", "margin: 1rem;")),
            r#"<div class="gl-rule" style="margin: 1rem;"><span class="gl-rule-line gl-rule-thin"></span></div>"#
        );
    }

    #[test]
    fn rule_merges_class_and_style_with_label() {
        assert_eq!(
            render(rule::<Dry>(
                RuleVariant::Thick,
                Some("BREAK"),
                "divider",
                "margin: 2rem 0;"
            )),
            r#"<div class="gl-rule divider" style="margin: 2rem 0;"><span class="gl-rule-line gl-rule-thick"></span><span class="gl-rule-label">BREAK</span><span class="gl-rule-line gl-rule-thick"></span></div>"#
        );
    }

    #[test]
    fn rule_omits_style_attribute_when_empty() {
        let html = render(rule::<Dry>(RuleVariant::Thin, Some("FEATURE"), "", ""));
        assert!(
            !html.contains(" style="),
            "no style attribute with empty passthrough: {html}"
        );
    }

    #[test]
    fn rule_re_exports_resolve_via_silkenweb_module() {
        let el: Div<Dry> = crate::silkenweb::rule(RuleVariant::Thin, Some("FEATURE"), "", "");
        assert_eq!(
            render(el),
            r#"<div class="gl-rule"><span class="gl-rule-line gl-rule-thin"></span><span class="gl-rule-label">FEATURE</span><span class="gl-rule-line gl-rule-thin"></span></div>"#
        );
    }
}
