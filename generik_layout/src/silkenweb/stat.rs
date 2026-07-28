//! Silkenweb adapter for the `Stat` layout component.
//!
//! [`stat`] mirrors the Leptos `Stat` component: a big-number stat — a large
//! display figure with a small mono label below. The signature element of
//! Swiss poster design: information presented as typography, not decoration.

use silkenweb::dom::Dom;
use silkenweb::elements::html::{span, Div};
use silkenweb::node::element::TextParentElement;

use crate::shared::merge_class;

use super::helpers::{layout_div, merge_style, styled};

/// A big-number stat: a large display figure with a small mono label below.
///
/// Emits two child spans in source order: `gl-stat-number` (the value, rendered
/// in the display font at display size) and `gl-eyebrow` (the caption, rendered
/// in mono). `class`/`style` are passthroughs on the outer `gl-stat` `<div>`;
/// an empty `style` emits no `style` attribute. The Leptos equivalent is the
/// `Stat` component.
pub fn stat<D>(number: &str, label: &str, class: &str, style: &str) -> Div<D>
where
    D: Dom,
{
    let class_str = merge_class("gl-stat", class);
    let style_str = merge_style("", style);

    let number_span = styled(span::<D>(), "gl-stat-number", "").text(number);
    let label_span = styled(span::<D>(), "gl-eyebrow", "").text(label);

    layout_div(&class_str, &style_str, [number_span, label_span])
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
    fn stat_emits_number_then_label() {
        assert_eq!(
            render(stat::<Dry>("250k", "COPIES", "", "")),
            r#"<div class="gl-stat"><span class="gl-stat-number">250k</span><span class="gl-eyebrow">COPIES</span></div>"#
        );
    }

    #[test]
    fn stat_preserves_number_label_source_order() {
        let html = render(stat::<Dry>("250k", "COPIES", "", ""));
        let number = html.find("gl-stat-number").unwrap();
        let label = html.find("gl-eyebrow").unwrap();
        assert!(number < label, "number must precede label: {html}");
    }

    #[test]
    fn stat_merges_passthrough_class() {
        assert_eq!(
            render(stat::<Dry>("250k", "COPIES", "feature", "")),
            r#"<div class="gl-stat feature"><span class="gl-stat-number">250k</span><span class="gl-eyebrow">COPIES</span></div>"#
        );
    }

    #[test]
    fn stat_merges_passthrough_style() {
        assert_eq!(
            render(stat::<Dry>("250k", "COPIES", "", "color: red;")),
            r#"<div class="gl-stat" style="color: red;"><span class="gl-stat-number">250k</span><span class="gl-eyebrow">COPIES</span></div>"#
        );
    }

    #[test]
    fn stat_merges_class_and_style() {
        assert_eq!(
            render(stat::<Dry>("250k", "COPIES", "feature", "color: red;")),
            r#"<div class="gl-stat feature" style="color: red;"><span class="gl-stat-number">250k</span><span class="gl-eyebrow">COPIES</span></div>"#
        );
    }

    #[test]
    fn stat_omits_style_attribute_when_empty() {
        let html = render(stat::<Dry>("250k", "COPIES", "", ""));
        assert!(
            !html.contains(" style="),
            "no style attribute with empty passthrough: {html}"
        );
    }

    #[test]
    fn stat_re_exports_resolve_via_silkenweb_module() {
        let el: Div<Dry> = crate::silkenweb::stat("250k", "COPIES", "", "");
        assert_eq!(
            render(el),
            r#"<div class="gl-stat"><span class="gl-stat-number">250k</span><span class="gl-eyebrow">COPIES</span></div>"#
        );
    }
}
