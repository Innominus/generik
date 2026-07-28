//! Silkenweb adapter for the `Card` layout component.
//!
//! [`card`] mirrors the Leptos `Card` component: a general-purpose container
//! with optional header, body, and footer regions. Each present region is
//! wrapped in its own `gl-card-header` / `gl-card-body` / `gl-card-footer`
//! `<div>` and rendered in source order header → body → footer inside the outer
//! `gl-card` `<div>`.

use silkenweb::dom::Dom;
use silkenweb::elements::html::Div;
use silkenweb::node::ChildNode;

use crate::shared::merge_class;

use super::helpers::{layout_div, merge_style};

/// A general-purpose card with optional header, body, and footer regions.
///
/// Each present region is wrapped in a `gl-card-header`, `gl-card-body`, or
/// `gl-card-footer` `<div>` and rendered in that source order inside the outer
/// `gl-card` `<div>`. Omit a region by passing `None`. `class`/`style` are
/// passthroughs on the outer `<div>`; an empty `style` emits no `style`
/// attribute. The Leptos equivalent is the `Card` component (whose regions are
/// passed via `CardHeader` / `CardBody` / `CardFooter` slots).
pub fn card<D>(
    header: Option<impl IntoIterator<Item = impl ChildNode<D>>>,
    body: Option<impl IntoIterator<Item = impl ChildNode<D>>>,
    footer: Option<impl IntoIterator<Item = impl ChildNode<D>>>,
    class: &str,
    style: &str,
) -> Div<D>
where
    D: Dom,
{
    let class_str = merge_class("gl-card", class);
    let style_str = merge_style("", style);

    let mut regions: Vec<Div<D>> = Vec::new();
    if let Some(header) = header {
        regions.push(layout_div("gl-card-header", "", header));
    }
    if let Some(body) = body {
        regions.push(layout_div("gl-card-body", "", body));
    }
    if let Some(footer) = footer {
        regions.push(layout_div("gl-card-footer", "", footer));
    }

    layout_div(&class_str, &style_str, regions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use silkenweb::dom::Dry;
    use silkenweb::elements::html::div;

    /// Render a Dry div to its HTML for assertion.
    fn render(el: Div<Dry>) -> String {
        el.freeze().to_string()
    }

    /// A single `<div>` child typed for the Dry DOM.
    fn one() -> [Div<Dry>; 1] {
        [div::<Dry>()]
    }

    /// An absent region, typed so `None` infers a concrete child iterator.
    fn none() -> Option<std::iter::Empty<Div<Dry>>> {
        None
    }

    #[test]
    fn card_no_regions_is_empty_outer_div() {
        assert_eq!(
            render(card::<Dry>(none(), none(), none(), "", "")),
            r#"<div class="gl-card"></div>"#
        );
    }

    #[test]
    fn card_header_only_wraps_in_header_region() {
        assert_eq!(
            render(card::<Dry>(Some(one()), none(), none(), "", "")),
            r#"<div class="gl-card"><div class="gl-card-header"><div></div></div></div>"#
        );
    }

    #[test]
    fn card_body_only_wraps_in_body_region() {
        assert_eq!(
            render(card::<Dry>(none(), Some(one()), none(), "", "")),
            r#"<div class="gl-card"><div class="gl-card-body"><div></div></div></div>"#
        );
    }

    #[test]
    fn card_footer_only_wraps_in_footer_region() {
        assert_eq!(
            render(card::<Dry>(none(), none(), Some(one()), "", "")),
            r#"<div class="gl-card"><div class="gl-card-footer"><div></div></div></div>"#
        );
    }

    #[test]
    fn card_all_regions_preserve_header_body_footer_order() {
        assert_eq!(
            render(card::<Dry>(Some(one()), Some(one()), Some(one()), "", "")),
            r#"<div class="gl-card"><div class="gl-card-header"><div></div></div><div class="gl-card-body"><div></div></div><div class="gl-card-footer"><div></div></div></div>"#
        );
    }

    #[test]
    fn card_merges_class_and_passthrough_style() {
        assert_eq!(
            render(card::<Dry>(
                none(),
                Some(one()),
                none(),
                "feature",
                "color: red;"
            )),
            r#"<div class="gl-card feature" style="color: red;"><div class="gl-card-body"><div></div></div></div>"#
        );
    }

    #[test]
    fn card_omits_style_attribute_when_empty() {
        let html = render(card::<Dry>(Some(one()), Some(one()), Some(one()), "", ""));
        assert!(
            !html.contains(" style="),
            "no style attribute with empty passthrough: {html}"
        );
    }

    #[test]
    fn card_appends_multiple_children_inside_a_region() {
        assert_eq!(
            render(card::<Dry>(
                Some([div::<Dry>(), div::<Dry>()]),
                none(),
                none(),
                "",
                ""
            )),
            r#"<div class="gl-card"><div class="gl-card-header"><div></div><div></div></div></div>"#
        );
    }

    #[test]
    fn card_re_exports_resolve_via_silkenweb_module() {
        let el: Div<Dry> = crate::silkenweb::card(none(), Some(one()), none(), "feature", "");
        assert_eq!(
            render(el),
            r#"<div class="gl-card feature"><div class="gl-card-body"><div></div></div></div>"#
        );
    }
}
