//! Silkenweb adapters for the box-style layout primitives: `container`,
//! `center`, `measure`, `stack`, and `cluster`.
//!
//! Each adapter mirrors the corresponding Leptos component's generated
//! `gl-*` class list and inline-style semantics, mapping explicit prop
//! arguments (`width`, `measure`, `gap`, `align`, `justify`, `class`, `style`)
//! onto a [`Div<D>`] and accepting typed direct children. The shared CSS,
//! tokens, and value types live at the crate root and are reused verbatim, so
//! the two framework adapters stay in lock-step.
//!
//! [`Div<D>`]: silkenweb::elements::html::Div

use silkenweb::dom::Dom;
use silkenweb::elements::html::Div;
use silkenweb::node::ChildNode;

use crate::shared::{merge_class, AlignItems, Gutter, JustifyContent};

use super::helpers::{layout_div, merge_style};

/// A max-width page wrapper that centers content horizontally.
///
/// Pass `width` as a CSS length (`"800px"`, `"60rem"`) to override the
/// `--gl-max-width` token; `""` leaves it to the stylesheet. `class`/`style` are
/// passthroughs, merged after the crate's own `gl-container` class and
/// `max-width` style.
///
/// The Leptos equivalent is the `Container` component.
pub fn container<D>(
    width: &str,
    class: &str,
    style: &str,
    children: impl IntoIterator<Item = impl ChildNode<D>>,
) -> Div<D>
where
    D: Dom,
{
    let own = if width.is_empty() {
        String::new()
    } else {
        format!("max-width: {width};")
    };
    let class_str = merge_class("gl-container", class);
    let style_str = merge_style(&own, style);
    layout_div(&class_str, &style_str, children)
}

/// Flex-centers its children on both axes with `min-height: 100%`.
///
/// Use for Swiss hero sections; for full-viewport height pass `class="min-h-screen"`
/// or set the parent's height. `class`/`style` are passthroughs. The Leptos
/// equivalent is the `Center` component.
pub fn center<D>(
    class: &str,
    style: &str,
    children: impl IntoIterator<Item = impl ChildNode<D>>,
) -> Div<D>
where
    D: Dom,
{
    let class_str = merge_class("gl-center", class);
    let style_str = merge_style("", style);
    layout_div(&class_str, &style_str, children)
}

/// A prose line-length wrapper, constraining content to a readable measure.
///
/// Pass `measure` as a CSS length or ch-based value (`"65ch"`, `"42em"`) to
/// override the `--gl-measure` token; `""` leaves it to the stylesheet.
/// `class`/`style` are passthroughs. The Leptos equivalent is the `Measure`
/// component.
pub fn measure<D>(
    measure: &str,
    class: &str,
    style: &str,
    children: impl IntoIterator<Item = impl ChildNode<D>>,
) -> Div<D>
where
    D: Dom,
{
    let own = if measure.is_empty() {
        String::new()
    } else {
        format!("max-width: {measure};")
    };
    let class_str = merge_class("gl-measure", class);
    let style_str = merge_style(&own, style);
    layout_div(&class_str, &style_str, children)
}

/// A vertical flex stack with a configurable gap.
///
/// `gap` defaults to [`Gutter::Md`] at the call site; `class`/`style` are
/// passthroughs. The Leptos equivalent is the `Stack` component.
pub fn stack<D>(
    gap: Gutter,
    class: &str,
    style: &str,
    children: impl IntoIterator<Item = impl ChildNode<D>>,
) -> Div<D>
where
    D: Dom,
{
    let base = merge_class("gl-stack", &format!("gl-gap{}", gap.class_suffix()));
    let class_str = merge_class(&base, class);
    let style_str = merge_style("", style);
    layout_div(&class_str, &style_str, children)
}

/// A horizontal flex that wraps, with vertical alignment and justification.
///
/// `align`/`justify` default to [`AlignItems::Center`] / [`JustifyContent::Start`]
/// at the call site and are emitted as inline `align-items` / `justify-content`
/// styles merged with any passthrough `style` — keeping the CSS small while
/// staying per-instance overridable. `class`/`style` are passthroughs. The
/// Leptos equivalent is the `Cluster` component.
pub fn cluster<D>(
    gap: Gutter,
    align: AlignItems,
    justify: JustifyContent,
    class: &str,
    style: &str,
    children: impl IntoIterator<Item = impl ChildNode<D>>,
) -> Div<D>
where
    D: Dom,
{
    let base = merge_class("gl-cluster", &format!("gl-gap{}", gap.class_suffix()));
    let class_str = merge_class(&base, class);
    let own = format!(
        "align-items: {}; justify-content: {};",
        align.css_value(),
        justify.css_value()
    );
    let style_str = merge_style(&own, style);
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

    /// An empty child iterator typed for the Dry DOM.
    fn empty() -> std::iter::Empty<Div<Dry>> {
        std::iter::empty()
    }

    #[test]
    fn container_defaults_omit_style() {
        assert_eq!(
            render(container::<Dry>("", "", "", empty())),
            r#"<div class="gl-container"></div>"#
        );
    }

    #[test]
    fn container_width_becomes_inline_style() {
        assert_eq!(
            render(container::<Dry>("800px", "", "", empty())),
            r#"<div class="gl-container" style="max-width: 800px;"></div>"#
        );
    }

    #[test]
    fn container_merges_class_and_style() {
        assert_eq!(
            render(container::<Dry>(
                "800px",
                "page",
                "margin: 0 auto;",
                empty()
            )),
            r#"<div class="gl-container page" style="max-width: 800px; margin: 0 auto;"></div>"#
        );
    }

    #[test]
    fn container_passthrough_style_only() {
        assert_eq!(
            render(container::<Dry>("", "", "color: red;", empty())),
            r#"<div class="gl-container" style="color: red;"></div>"#
        );
    }

    #[test]
    fn center_defaults_omit_style() {
        assert_eq!(
            render(center::<Dry>("", "", empty())),
            r#"<div class="gl-center"></div>"#
        );
    }

    #[test]
    fn center_merges_class_and_style() {
        assert_eq!(
            render(center::<Dry>("hero", "min-height: 100%;", empty())),
            r#"<div class="gl-center hero" style="min-height: 100%;"></div>"#
        );
    }

    #[test]
    fn measure_defaults_omit_style() {
        assert_eq!(
            render(measure::<Dry>("", "", "", empty())),
            r#"<div class="gl-measure"></div>"#
        );
    }

    #[test]
    fn measure_value_becomes_inline_style() {
        assert_eq!(
            render(measure::<Dry>("65ch", "", "", empty())),
            r#"<div class="gl-measure" style="max-width: 65ch;"></div>"#
        );
    }

    #[test]
    fn measure_merges_class_and_style() {
        assert_eq!(
            render(measure::<Dry>("65ch", "prose", "margin: auto;", empty())),
            r#"<div class="gl-measure prose" style="max-width: 65ch; margin: auto;"></div>"#
        );
    }

    #[test]
    fn stack_defaults_use_md_gap() {
        assert_eq!(
            render(stack::<Dry>(Gutter::Md, "", "", empty())),
            r#"<div class="gl-stack gl-gap-md"></div>"#
        );
    }

    #[test]
    fn stack_gap_suffix_changes() {
        assert_eq!(
            render(stack::<Dry>(Gutter::Sm, "", "", empty())),
            r#"<div class="gl-stack gl-gap-sm"></div>"#
        );
        assert_eq!(
            render(stack::<Dry>(Gutter::Lg, "", "", empty())),
            r#"<div class="gl-stack gl-gap-lg"></div>"#
        );
    }

    #[test]
    fn stack_merges_class_and_style() {
        assert_eq!(
            render(stack::<Dry>(
                Gutter::Lg,
                "section",
                "padding: 1rem;",
                empty()
            )),
            r#"<div class="gl-stack gl-gap-lg section" style="padding: 1rem;"></div>"#
        );
    }

    #[test]
    fn cluster_defaults_emit_align_and_justify() {
        assert_eq!(
            render(cluster::<Dry>(
                Gutter::Md,
                AlignItems::Center,
                JustifyContent::Start,
                "",
                "",
                empty()
            )),
            r#"<div class="gl-cluster gl-gap-md" style="align-items: center; justify-content: flex-start;"></div>"#
        );
    }

    #[test]
    fn cluster_align_and_justify_values() {
        assert_eq!(
            render(cluster::<Dry>(
                Gutter::Sm,
                AlignItems::End,
                JustifyContent::Between,
                "",
                "",
                empty()
            )),
            r#"<div class="gl-cluster gl-gap-sm" style="align-items: flex-end; justify-content: space-between;"></div>"#
        );
    }

    #[test]
    fn cluster_merges_class_and_passthrough_style() {
        assert_eq!(
            render(cluster::<Dry>(
                Gutter::Md,
                AlignItems::Center,
                JustifyContent::Start,
                "tags",
                "flex-wrap: wrap;",
                empty()
            )),
            r#"<div class="gl-cluster gl-gap-md tags" style="align-items: center; justify-content: flex-start; flex-wrap: wrap;"></div>"#
        );
    }

    #[test]
    fn children_are_appended_inside_the_layout_box() {
        assert_eq!(
            render(container::<Dry>("", "", "", [div::<Dry>(), div::<Dry>()])),
            r#"<div class="gl-container"><div></div><div></div></div>"#
        );
    }

    #[test]
    fn public_path_re_exports_the_adapters() {
        // The crate-level re-export under `generik_layout::silkenweb` resolves
        // to the same items the adapters produce.
        let el: Div<Dry> = crate::silkenweb::container("800px", "page", "", empty());
        assert_eq!(
            render(el),
            r#"<div class="gl-container page" style="max-width: 800px;"></div>"#
        );
    }
}
