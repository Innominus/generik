//! Silkenweb adapter for the `Metric` layout component.
//!
//! [`metric`] mirrors the Leptos `Metric` component: a data-poster metric with
//! a big display figure, a small mono label, and an optional delta (trend
//! indicator). It is a richer [`stat`](super::stat) for editorial data posters
//! where the change matters as much as the value.

use silkenweb::dom::Dom;
use silkenweb::elements::html::{span, Div, Span};
use silkenweb::node::element::TextParentElement;

use crate::shared::merge_class;

use super::helpers::{layout_div, merge_style, styled};

/// A data-poster metric: a big display figure, a small mono label, and an
/// optional delta (trend indicator).
///
/// Emits three child spans in source order: `gl-metric-value` (the big number,
/// e.g. "14" or "1.2k"), an optional `gl-metric-delta` (the trend, e.g. "+12%"
/// or "-3%"), and `gl-eyebrow` (the caption). The delta's inline color is
/// `var(--gl-accent)` when it begins with ASCII `+` and `var(--gl-ink-muted)`
/// otherwise. `class`/`style` are passthroughs on the outer `gl-metric` `<div>`;
/// an empty `style` emits no `style` attribute. The Leptos equivalent is the
/// `Metric` component.
pub fn metric<D>(value: &str, label: &str, delta: Option<&str>, class: &str, style: &str) -> Div<D>
where
    D: Dom,
{
    let class_str = merge_class("gl-metric", class);
    let style_str = merge_style("", style);

    let mut children: Vec<Span<D>> = Vec::new();
    children.push(styled(span::<D>(), "gl-metric-value", "").text(value));
    if let Some(delta) = delta {
        let color = if delta.starts_with('+') {
            "color: var(--gl-accent);"
        } else {
            "color: var(--gl-ink-muted);"
        };
        children.push(styled(span::<D>(), "gl-metric-delta", color).text(delta));
    }
    children.push(styled(span::<D>(), "gl-eyebrow", "").text(label));

    layout_div(&class_str, &style_str, children)
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
    fn metric_without_delta_emits_value_then_label() {
        assert_eq!(
            render(metric::<Dry>("14", "ISSUES", None, "", "")),
            r#"<div class="gl-metric"><span class="gl-metric-value">14</span><span class="gl-eyebrow">ISSUES</span></div>"#
        );
    }

    #[test]
    fn metric_with_positive_delta_uses_accent_color() {
        assert_eq!(
            render(metric::<Dry>("1.2k", "READERS", Some("+12%"), "", "")),
            r#"<div class="gl-metric"><span class="gl-metric-value">1.2k</span><span class="gl-metric-delta" style="color: var(--gl-accent);">+12%</span><span class="gl-eyebrow">READERS</span></div>"#
        );
    }

    #[test]
    fn metric_with_non_positive_delta_uses_ink_muted_color() {
        assert_eq!(
            render(metric::<Dry>("3", "NEW", Some("-3%"), "", "")),
            r#"<div class="gl-metric"><span class="gl-metric-value">3</span><span class="gl-metric-delta" style="color: var(--gl-ink-muted);">-3%</span><span class="gl-eyebrow">NEW</span></div>"#
        );
    }

    #[test]
    fn metric_preserves_value_delta_label_source_order() {
        let html = render(metric::<Dry>("1.2k", "READERS", Some("+12%"), "", ""));
        let value = html.find("gl-metric-value").unwrap();
        let delta = html.find("gl-metric-delta").unwrap();
        let label = html.find("gl-eyebrow").unwrap();
        assert!(
            value < delta && delta < label,
            "value, delta, label order required: {html}"
        );
    }

    #[test]
    fn metric_delta_is_omitted_when_none() {
        let html = render(metric::<Dry>("14", "ISSUES", None, "", ""));
        assert!(
            !html.contains("gl-metric-delta"),
            "no delta span when delta is None: {html}"
        );
    }

    #[test]
    fn metric_merges_passthrough_class() {
        assert_eq!(
            render(metric::<Dry>("14", "ISSUES", None, "hero", "")),
            r#"<div class="gl-metric hero"><span class="gl-metric-value">14</span><span class="gl-eyebrow">ISSUES</span></div>"#
        );
    }

    #[test]
    fn metric_merges_passthrough_style() {
        assert_eq!(
            render(metric::<Dry>("14", "ISSUES", None, "", "padding: 1rem;")),
            r#"<div class="gl-metric" style="padding: 1rem;"><span class="gl-metric-value">14</span><span class="gl-eyebrow">ISSUES</span></div>"#
        );
    }

    #[test]
    fn metric_merges_class_and_style_with_delta() {
        assert_eq!(
            render(metric::<Dry>(
                "1.2k",
                "READERS",
                Some("+12%"),
                "hero",
                "padding: 1rem;"
            )),
            r#"<div class="gl-metric hero" style="padding: 1rem;"><span class="gl-metric-value">1.2k</span><span class="gl-metric-delta" style="color: var(--gl-accent);">+12%</span><span class="gl-eyebrow">READERS</span></div>"#
        );
    }

    #[test]
    fn metric_omits_outer_style_attribute_when_empty() {
        let html = render(metric::<Dry>("14", "ISSUES", None, "", ""));
        // Only the delta span carries an inline style; the outer div must not.
        assert!(
            !html.starts_with(r#"<div class="gl-metric" style="#),
            "no style attribute on outer div with empty passthrough: {html}"
        );
    }

    #[test]
    fn metric_re_exports_resolve_via_silkenweb_module() {
        let el: Div<Dry> = crate::silkenweb::metric("1.2k", "READERS", Some("+12%"), "", "");
        assert_eq!(
            render(el),
            r#"<div class="gl-metric"><span class="gl-metric-value">1.2k</span><span class="gl-metric-delta" style="color: var(--gl-accent);">+12%</span><span class="gl-eyebrow">READERS</span></div>"#
        );
    }
}
