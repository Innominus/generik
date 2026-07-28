//! Silkenweb adapter for the `Dropcap` layout component.
//!
//! [`dropcap`] mirrors the Leptos `Dropcap` component: an editorial drop cap
//! that enlarges the first letter of its child content via `::first-letter`.
//! Pure CSS, no JS, no layout shift — the `.gl-dropcap::first-letter` rule is
//! provided by the crate's global CSS.

use silkenweb::dom::Dom;
use silkenweb::elements::html::Div;
use silkenweb::node::ChildNode;

use crate::shared::merge_class;

use super::helpers::{layout_div, merge_style};

/// An editorial drop cap: enlarges the first letter of the child content via
/// `::first-letter`. Pure CSS, no JS, no layout shift.
///
/// Wrap a single `<p>` (or any block with text content) to give its first
/// letter a 3-line display-font capital in the accent color. `class`/`style`
/// are passthroughs on the outer `gl-dropcap` `<div>`; an empty `style` emits
/// no `style` attribute. The Leptos equivalent is the `Dropcap` component.
pub fn dropcap<D>(
    class: &str,
    style: &str,
    children: impl IntoIterator<Item = impl ChildNode<D>>,
) -> Div<D>
where
    D: Dom,
{
    let class_str = merge_class("gl-dropcap", class);
    let style_str = merge_style("", style);
    layout_div(&class_str, &style_str, children)
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

    /// An empty child iterator typed for the Dry DOM.
    fn empty() -> std::iter::Empty<Div<Dry>> {
        std::iter::empty()
    }

    #[test]
    fn dropcap_wraps_children_in_gl_dropcap() {
        assert_eq!(
            render(dropcap::<Dry>("", "", one())),
            r#"<div class="gl-dropcap"><div></div></div>"#
        );
    }

    #[test]
    fn dropcap_appends_multiple_children() {
        assert_eq!(
            render(dropcap::<Dry>("", "", [div::<Dry>(), div::<Dry>()])),
            r#"<div class="gl-dropcap"><div></div><div></div></div>"#
        );
    }

    #[test]
    fn dropcap_merges_passthrough_class() {
        assert_eq!(
            render(dropcap::<Dry>("lede", "", one())),
            r#"<div class="gl-dropcap lede"><div></div></div>"#
        );
    }

    #[test]
    fn dropcap_merges_passthrough_style() {
        assert_eq!(
            render(dropcap::<Dry>("", "color: red;", one())),
            r#"<div class="gl-dropcap" style="color: red;"><div></div></div>"#
        );
    }

    #[test]
    fn dropcap_merges_class_and_style() {
        assert_eq!(
            render(dropcap::<Dry>("lede", "margin: 1rem;", one())),
            r#"<div class="gl-dropcap lede" style="margin: 1rem;"><div></div></div>"#
        );
    }

    #[test]
    fn dropcap_omits_style_attribute_when_empty() {
        let html = render(dropcap::<Dry>("", "", empty()));
        assert!(
            !html.contains(" style="),
            "no style attribute with empty passthrough: {html}"
        );
    }

    #[test]
    fn dropcap_re_exports_resolve_via_silkenweb_module() {
        let el: Div<Dry> = crate::silkenweb::dropcap("lede", "", one());
        assert_eq!(
            render(el),
            r#"<div class="gl-dropcap lede"><div></div></div>"#
        );
    }
}
