//! Silkenweb adapter for the `Masonry` layout component.
//!
//! [`masonry`] mirrors the Leptos `Masonry` component: a CSS Grid auto-flow
//! dense mosaic. It emits the `gl-masonry gl-gap-*` classes and always sets an
//! inline `grid-template-columns: repeat(cols, 1fr);` style (prepended before
//! any passthrough `style`). Children set their own spans via their own classes
//! or styles — `masonry` itself imposes no spans, only the track count and auto
//! rows.

use silkenweb::dom::Dom;
use silkenweb::elements::html::Div;
use silkenweb::node::ChildNode;

use crate::shared::{merge_class, Gutter};

use super::helpers::{layout_div, merge_style};

/// A CSS Grid auto-flow dense mosaic.
///
/// `cols` is the track count (default 12 at the call site, clamped to at least
/// 1); `gap` defaults to [`Gutter::Md`]; `class`/`style` are passthroughs. The
/// track count is emitted as an inline `grid-template-columns: repeat(cols, 1fr)`
/// style, merged ahead of any passthrough `style`. Children set their own
/// `grid-column: span X` / `grid-row: span Y` via their own classes or
/// passthrough styles. The Leptos equivalent is the `Masonry` component.
pub fn masonry<D>(
    cols: usize,
    gap: Gutter,
    class: &str,
    style: &str,
    children: impl IntoIterator<Item = impl ChildNode<D>>,
) -> Div<D>
where
    D: Dom,
{
    let cols = cols.max(1);
    let base = merge_class("gl-masonry", &format!("gl-gap{}", gap.class_suffix()));
    let class_str = merge_class(&base, class);

    let own = format!("grid-template-columns: repeat({cols}, 1fr);");
    let style_str = merge_style(&own, style);

    layout_div(&class_str, &style_str, children)
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

    /// An empty child iterator typed for the Dry DOM.
    fn empty() -> std::iter::Empty<Div<Dry>> {
        std::iter::empty()
    }

    #[test]
    fn masonry_defaults_emit_template_columns() {
        // cols=12 (default), gap=Md (default): always carries the track-count style.
        assert_eq!(
            render(masonry::<Dry>(12, Gutter::Md, "", "", empty())),
            r#"<div class="gl-masonry gl-gap-md" style="grid-template-columns: repeat(12, 1fr);"></div>"#
        );
    }

    #[test]
    fn masonry_gap_suffix_changes() {
        assert_eq!(
            render(masonry::<Dry>(12, Gutter::Sm, "", "", empty())),
            r#"<div class="gl-masonry gl-gap-sm" style="grid-template-columns: repeat(12, 1fr);"></div>"#
        );
        assert_eq!(
            render(masonry::<Dry>(12, Gutter::Lg, "", "", empty())),
            r#"<div class="gl-masonry gl-gap-lg" style="grid-template-columns: repeat(12, 1fr);"></div>"#
        );
    }

    #[test]
    fn masonry_cols_override_template_columns() {
        assert_eq!(
            render(masonry::<Dry>(4, Gutter::Md, "", "", empty())),
            r#"<div class="gl-masonry gl-gap-md" style="grid-template-columns: repeat(4, 1fr);"></div>"#
        );
    }

    #[test]
    fn masonry_clamps_cols_below_one_to_one() {
        assert_eq!(
            render(masonry::<Dry>(0, Gutter::Md, "", "", empty())),
            r#"<div class="gl-masonry gl-gap-md" style="grid-template-columns: repeat(1, 1fr);"></div>"#
        );
    }

    #[test]
    fn masonry_merges_template_columns_with_passthrough_style() {
        assert_eq!(
            render(masonry::<Dry>(3, Gutter::Md, "", "row-gap: 2rem;", empty())),
            r#"<div class="gl-masonry gl-gap-md" style="grid-template-columns: repeat(3, 1fr); row-gap: 2rem;"></div>"#
        );
    }

    #[test]
    fn masonry_merges_class() {
        assert_eq!(
            render(masonry::<Dry>(12, Gutter::Md, "gallery", "", empty())),
            r#"<div class="gl-masonry gl-gap-md gallery" style="grid-template-columns: repeat(12, 1fr);"></div>"#
        );
    }

    #[test]
    fn masonry_appends_children() {
        assert_eq!(
            render(masonry::<Dry>(
                12,
                Gutter::Md,
                "",
                "",
                [div::<Dry>(), div::<Dry>()]
            )),
            r#"<div class="gl-masonry gl-gap-md" style="grid-template-columns: repeat(12, 1fr);"><div></div><div></div></div>"#
        );
    }
}
