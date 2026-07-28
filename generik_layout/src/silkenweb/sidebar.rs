//! Silkenweb adapter for the `Sidebar` layout component.
//!
//! [`sidebar`] mirrors the Leptos `Sidebar` component: an asymmetric two-column
//! layout (a wide main column plus a narrow side column) that collapses to one
//! column below 768px. Unlike the Leptos component, which takes the side
//! content via a `SidebarSide` slot, the adapter takes the main and side
//! content as two separate typed child iterators and preserves the same
//! source-order swap: `Left` renders the side column first, `Right` renders the
//! main column first.

use silkenweb::dom::Dom;
use silkenweb::elements::html::Div;
use silkenweb::node::ChildNode;

use crate::shared::{merge_class, Gutter, Side};

use super::helpers::{layout_div, merge_style};

/// Asymmetric two-column layout: a wide main column plus a narrow side column.
///
/// `side` controls which side the narrow column sits on ([`Side::Left`] default
/// or [`Side::Right`]); `side_width` overrides the narrow column width (`""`
/// uses the `--gl-sidebar-width` token); `gap` defaults to [`Gutter::Md`];
/// `class`/`style` are passthroughs. The `gl-sidebar-right` class is added only
/// for [`Side::Right`]; the `--gl-sidebar-width` style is prepended only when
/// `side_width` is non-empty.
///
/// `main` holds the wide-column content and `aside` the narrow-column content.
/// Source order follows the visual order: `Left` renders `aside` first, `Right`
/// renders `main` first. The Leptos equivalent is the `Sidebar` component (whose
/// narrow content is passed via a `SidebarSide` slot).
pub fn sidebar<D>(
    side: Side,
    side_width: &str,
    gap: Gutter,
    class: &str,
    style: &str,
    main: impl IntoIterator<Item = impl ChildNode<D>>,
    aside: impl IntoIterator<Item = impl ChildNode<D>>,
) -> Div<D>
where
    D: Dom,
{
    let mut base = "gl-sidebar".to_string();
    if side == Side::Right {
        base = merge_class(&base, "gl-sidebar-right");
    }
    base = merge_class(&base, &format!("gl-gap{}", gap.class_suffix()));
    let class_str = merge_class(&base, class);

    let own = if side_width.is_empty() {
        String::new()
    } else {
        format!("--gl-sidebar-width: {side_width};")
    };
    let style_str = merge_style(&own, style);

    let main_div = layout_div("gl-sidebar-main", "", main);
    let aside_div = layout_div("gl-sidebar-side", "", aside);

    // Source order matches the visual side: Left -> aside first, Right -> main first.
    let (first, second) = if side == Side::Right {
        (main_div, aside_div)
    } else {
        (aside_div, main_div)
    };

    layout_div(&class_str, &style_str, [first, second])
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

    /// A single `<div>` child typed for the Dry DOM.
    fn one() -> [Div<Dry>; 1] {
        [div::<Dry>()]
    }

    #[test]
    fn sidebar_left_renders_side_first() {
        // Left (default): aside (gl-sidebar-side) comes before main.
        assert_eq!(
            render(sidebar::<Dry>(
                Side::Left,
                "",
                Gutter::Md,
                "",
                "",
                one(),
                one()
            )),
            r#"<div class="gl-sidebar gl-gap-md"><div class="gl-sidebar-side"><div></div></div><div class="gl-sidebar-main"><div></div></div></div>"#
        );
    }

    #[test]
    fn sidebar_right_renders_main_first_and_adds_right_class() {
        // Right: main first, plus the gl-sidebar-right class.
        assert_eq!(
            render(sidebar::<Dry>(
                Side::Right,
                "",
                Gutter::Md,
                "",
                "",
                one(),
                one()
            )),
            r#"<div class="gl-sidebar gl-sidebar-right gl-gap-md"><div class="gl-sidebar-main"><div></div></div><div class="gl-sidebar-side"><div></div></div></div>"#
        );
    }

    #[test]
    fn sidebar_side_width_becomes_custom_property() {
        assert_eq!(
            render(sidebar::<Dry>(
                Side::Left,
                "240px",
                Gutter::Md,
                "",
                "",
                empty(),
                empty()
            )),
            r#"<div class="gl-sidebar gl-gap-md" style="--gl-sidebar-width: 240px;"><div class="gl-sidebar-side"></div><div class="gl-sidebar-main"></div></div>"#
        );
    }

    #[test]
    fn sidebar_side_width_merges_with_passthrough_style() {
        assert_eq!(
            render(sidebar::<Dry>(
                Side::Right,
                "240px",
                Gutter::Md,
                "",
                "background: #fff;",
                empty(),
                empty()
            )),
            r#"<div class="gl-sidebar gl-sidebar-right gl-gap-md" style="--gl-sidebar-width: 240px; background: #fff;"><div class="gl-sidebar-main"></div><div class="gl-sidebar-side"></div></div>"#
        );
    }

    #[test]
    fn sidebar_passthrough_style_only() {
        assert_eq!(
            render(sidebar::<Dry>(
                Side::Left,
                "",
                Gutter::Md,
                "",
                "color: red;",
                empty(),
                empty()
            )),
            r#"<div class="gl-sidebar gl-gap-md" style="color: red;"><div class="gl-sidebar-side"></div><div class="gl-sidebar-main"></div></div>"#
        );
    }

    #[test]
    fn sidebar_merges_class() {
        assert_eq!(
            render(sidebar::<Dry>(
                Side::Left,
                "",
                Gutter::Lg,
                "article",
                "",
                empty(),
                empty()
            )),
            r#"<div class="gl-sidebar gl-gap-lg article"><div class="gl-sidebar-side"></div><div class="gl-sidebar-main"></div></div>"#
        );
    }

    #[test]
    fn sidebar_appends_children_into_each_column() {
        // main gets two divs, aside gets one; verify each column nests its own.
        let html = render(sidebar::<Dry>(
            Side::Left,
            "",
            Gutter::Md,
            "",
            "",
            [div::<Dry>(), div::<Dry>()],
            [div::<Dry>()],
        ));
        // aside (side) first with one child, then main with two.
        assert_eq!(
            html,
            r#"<div class="gl-sidebar gl-gap-md"><div class="gl-sidebar-side"><div></div></div><div class="gl-sidebar-main"><div></div><div></div></div></div>"#
        );
    }
}
