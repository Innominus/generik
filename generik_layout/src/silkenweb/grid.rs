//! Silkenweb adapters for the `Grid` container and its `Col` children.
//!
//! [`grid`] mirrors the Leptos `Grid` component's `gl-grid gl-gap-*` classes and
//! conditional `max-width` inline style. [`col`] mirrors `Col`: it takes all 12
//! responsive span/start arguments, clamps each to at least 1, and either emits
//! just `gl-col` (all-default) or a generated `gl-col-N` class backed by a
//! nested `<style>` element. Because a Silkenweb adapter returns a single
//! [`Div<D>`] rather than a view fragment, the non-default `col` nests that
//! `<style>` as the div's first child — ahead of the user children — instead of
//! the Leptos component's sibling `<style>` + `<div>` pair. No shadow DOM is
//! used.
//!
//! [`Div<D>`]: silkenweb::elements::html::Div

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{LazyLock, Mutex};

use silkenweb::dom::Dom;
use silkenweb::elements::html::Div;
use silkenweb::node::element::{ParentElement, TextParentElement};
use silkenweb::node::ChildNode;

use crate::col_css::{compute_col_css, ColConfig};
use crate::shared::{merge_class, Gutter};

use super::helpers::{bare_div, layout_div, merge_style};

/// A centered 12-column CSS grid container.
///
/// Pass `Col` children for responsive column spans. `max_width` overrides the
/// `--gl-max-width` token inline (`""` leaves it to the stylesheet); `gap`
/// defaults to [`Gutter::Md`] at the call site; `class`/`style` are
/// passthroughs, merged after the crate's own `gl-grid gl-gap-*` classes and any
/// `max-width` style. The Leptos equivalent is the `Grid` component.
pub fn grid<D>(
    gap: Gutter,
    max_width: &str,
    class: &str,
    style: &str,
    children: impl IntoIterator<Item = impl ChildNode<D>>,
) -> Div<D>
where
    D: Dom,
{
    let base = merge_class("gl-grid", &format!("gl-gap{}", gap.class_suffix()));
    let class_str = merge_class(&base, class);

    let own = if max_width.is_empty() {
        String::new()
    } else {
        format!("max-width: {max_width};")
    };
    let style_str = merge_style(&own, style);

    layout_div(&class_str, &style_str, children)
}

/// Process-wide id counter and CSS cache for non-default `col` instances.
///
/// Kept local to the Silkenweb adapter (and separate from the Leptos `Col`
/// cache) so the two framework adapters are independently usable. The cache is
/// keyed by the framework-neutral [`ColConfig`]; the generated CSS depends only
/// on the config, so a hit reuses both the id and the stylesheet text.
static COL_COUNTER: AtomicUsize = AtomicUsize::new(0);

static COL_CSS_CACHE: LazyLock<Mutex<HashMap<ColConfig, (usize, String)>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// A column inside a [`grid`]. Mobile-first responsive span/start with cascade
/// inheritance: the effective span at a breakpoint is the most recently
/// specified `cols_*` value going up from the base (same mental model as
/// Tailwind).
///
/// Takes all 12 responsive span/start arguments plus `class`/`style`. Every
/// supplied span/start is clamped to at least 1. With all defaults (`cols = 12`
/// and no responsive overrides or starts) only the base `gl-col` class is
/// emitted. Any non-default config generates a unique `gl-col-N` class whose
/// responsive rules live in a `<style>` element nested as the div's first child,
/// ahead of `children` — Silkenweb adapters return a single [`Div<D>`], not a
/// fragment, so the Leptos component's sibling `<style>` is folded inside
/// instead. The Leptos equivalent is the `Col` component.
///
/// Note: `col` assumes a 12-column parent grid. For [`masonry`](super::masonry)
/// (which may use a different track count), set spans on children directly via
/// their own classes.
// 15 parameters mirror the Leptos `Col` prop set exactly: 12 responsive
// span/start arguments plus `class`, `style`, and `children`. Collapsing it
// would break parity, so silence clippy's arg-count lint for this adapter.
#[allow(clippy::too_many_arguments)]
pub fn col<D>(
    cols: usize,
    cols_sm: Option<usize>,
    cols_md: Option<usize>,
    cols_lg: Option<usize>,
    cols_xl: Option<usize>,
    cols_2xl: Option<usize>,
    start: Option<usize>,
    start_sm: Option<usize>,
    start_md: Option<usize>,
    start_lg: Option<usize>,
    start_xl: Option<usize>,
    start_2xl: Option<usize>,
    class: &str,
    style: &str,
    children: impl IntoIterator<Item = impl ChildNode<D>>,
) -> Div<D>
where
    D: Dom,
{
    let cols = cols.max(1);
    let cols_sm = cols_sm.map(|c| c.max(1));
    let cols_md = cols_md.map(|c| c.max(1));
    let cols_lg = cols_lg.map(|c| c.max(1));
    let cols_xl = cols_xl.map(|c| c.max(1));
    let cols_2xl = cols_2xl.map(|c| c.max(1));
    let start = start.map(|s| s.max(1));
    let start_sm = start_sm.map(|s| s.max(1));
    let start_md = start_md.map(|s| s.max(1));
    let start_lg = start_lg.map(|s| s.max(1));
    let start_xl = start_xl.map(|s| s.max(1));
    let start_2xl = start_2xl.map(|s| s.max(1));

    let config = ColConfig {
        cols,
        cols_sm,
        cols_md,
        cols_lg,
        cols_xl,
        cols_2xl,
        start,
        start_sm,
        start_md,
        start_lg,
        start_xl,
        start_2xl,
    };

    if config.is_all_default() {
        let class_str = merge_class("gl-col", class);
        return layout_div(&class_str, style, children);
    }

    let (class_name, css) = {
        let mut cache = COL_CSS_CACHE.lock().unwrap_or_else(|e| e.into_inner());
        if let Some((existing_id, existing_css)) = cache.get(&config) {
            (format!("gl-col-{}", *existing_id), existing_css.clone())
        } else {
            let new_id = COL_COUNTER.fetch_add(1, Ordering::Relaxed);
            let (new_class_name, new_css) = compute_col_css(
                new_id, cols, cols_sm, cols_md, cols_lg, cols_xl, cols_2xl, start, start_sm,
                start_md, start_lg, start_xl, start_2xl,
            )
            .expect("non-default Col always yields Some");
            cache.insert(config, (new_id, new_css.clone()));
            (new_class_name, new_css)
        }
    };

    let class_str = merge_class(&format!("gl-col {class_name}"), class);

    bare_div::<D>(&class_str, style)
        .child(silkenweb::elements::html::style::<D>().text(css))
        .children(children)
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
    fn grid_defaults_omit_style() {
        assert_eq!(
            render(grid::<Dry>(Gutter::Md, "", "", "", empty())),
            r#"<div class="gl-grid gl-gap-md"></div>"#
        );
    }

    #[test]
    fn grid_gap_suffix_changes() {
        assert_eq!(
            render(grid::<Dry>(Gutter::Sm, "", "", "", empty())),
            r#"<div class="gl-grid gl-gap-sm"></div>"#
        );
        assert_eq!(
            render(grid::<Dry>(Gutter::Lg, "", "", "", empty())),
            r#"<div class="gl-grid gl-gap-lg"></div>"#
        );
    }

    #[test]
    fn grid_max_width_becomes_inline_style() {
        assert_eq!(
            render(grid::<Dry>(Gutter::Md, "1200px", "", "", empty())),
            r#"<div class="gl-grid gl-gap-md" style="max-width: 1200px;"></div>"#
        );
    }

    #[test]
    fn grid_merges_max_width_with_passthrough_style() {
        assert_eq!(
            render(grid::<Dry>(
                Gutter::Md,
                "1200px",
                "",
                "margin: 0 auto;",
                empty()
            )),
            r#"<div class="gl-grid gl-gap-md" style="max-width: 1200px; margin: 0 auto;"></div>"#
        );
    }

    #[test]
    fn grid_passthrough_style_only() {
        assert_eq!(
            render(grid::<Dry>(Gutter::Md, "", "", "color: red;", empty())),
            r#"<div class="gl-grid gl-gap-md" style="color: red;"></div>"#
        );
    }

    #[test]
    fn grid_merges_class() {
        assert_eq!(
            render(grid::<Dry>(Gutter::Md, "", "page", "", empty())),
            r#"<div class="gl-grid gl-gap-md page"></div>"#
        );
    }

    #[test]
    fn grid_appends_children() {
        assert_eq!(
            render(grid::<Dry>(
                Gutter::Md,
                "",
                "",
                "",
                [div::<Dry>(), div::<Dry>()]
            )),
            r#"<div class="gl-grid gl-gap-md"><div></div><div></div></div>"#
        );
    }

    #[test]
    fn col_all_default_emits_only_gl_col() {
        // cols=12, no responsive spans, no starts -> no generated class, no <style>.
        assert_eq!(
            render(col::<Dry>(
                12,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                "",
                "",
                [div::<Dry>()]
            )),
            r#"<div class="gl-col"><div></div></div>"#
        );
    }

    #[test]
    fn col_all_default_merges_class_and_style() {
        assert_eq!(
            render(col::<Dry>(
                12,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                "feature",
                "color: blue;",
                [div::<Dry>()]
            )),
            r#"<div class="gl-col feature" style="color: blue;"><div></div></div>"#
        );
    }

    #[test]
    fn col_non_default_emits_generated_class_and_nested_style_first() {
        // cols=6 -> non-default. The generated id is process-global and shared
        // across tests (cached by config), so assert on structure and CSS
        // content rather than the exact `gl-col-N` number.
        let html = render(col::<Dry>(
            6,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            "",
            "",
            [div::<Dry>()],
        ));

        // Class is `gl-col` plus a generated `gl-col-<n>`.
        assert!(
            html.starts_with("<div class=\"gl-col gl-col-"),
            "class should be `gl-col gl-col-<n>`: {html}"
        );

        // A <style> carrying the responsive rule is nested inside the div.
        assert!(html.contains("<style>"), "nested <style> present: {html}");
        assert!(
            html.contains("grid-column: span 6;"),
            "generated CSS carries the span-6 rule: {html}"
        );

        // The <style> precedes the user children.
        let style_pos = html.find("<style>").unwrap();
        let user_pos = html.find("<div></div>").unwrap();
        assert!(
            style_pos < user_pos,
            "<style> must be the first child, before user children: {html}"
        );

        // The outer div closes after the user div.
        assert!(
            html.ends_with("</div></div>"),
            "outer div must close after its children: {html}"
        );
    }

    #[test]
    fn col_non_default_responsive_rule_appears_in_nested_style() {
        // cols_md=6 with default base cols=12 -> base span-12 plus an md media
        // query. Assert the CSS content (independent of the generated id).
        let html = render(col::<Dry>(
            12,
            None,
            Some(6),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            "",
            "",
            empty(),
        ));
        assert!(
            html.contains("grid-column: span 12;"),
            "base rule present: {html}"
        );
        assert!(
            html.contains("@media (min-width: 768px)"),
            "md media query present: {html}"
        );
        assert!(
            html.contains("grid-column: span 6; }"),
            "md override rule present: {html}"
        );
    }

    #[test]
    fn col_non_default_start_rule_appears_in_nested_style() {
        // start=3, cols=6 -> base rule `3 / span 6`.
        let html = render(col::<Dry>(
            6,
            None,
            None,
            None,
            None,
            None,
            Some(3),
            None,
            None,
            None,
            None,
            None,
            "",
            "",
            empty(),
        ));
        assert!(
            html.contains("grid-column: 3 / span 6;"),
            "start rule present: {html}"
        );
        assert!(
            !html.contains("@media"),
            "no responsive overrides -> no media queries: {html}"
        );
    }

    #[test]
    fn col_clamps_span_below_one_to_one() {
        // cols=0 clamps to 1 -> non-default, base rule is `span 1` (not `span 0`).
        let html = render(col::<Dry>(
            0,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            "",
            "",
            empty(),
        ));
        assert!(
            html.contains("grid-column: span 1;"),
            "cols=0 clamps to 1: {html}"
        );
        assert!(
            !html.contains("span 0"),
            "no zero-span rule should be emitted: {html}"
        );
    }

    #[test]
    fn col_non_default_merges_passthrough_class_and_style() {
        let html = render(col::<Dry>(
            6,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            "feature",
            "color: blue;",
            empty(),
        ));
        assert!(
            html.starts_with("<div class=\"gl-col gl-col-"),
            "generated class kept: {html}"
        );
        assert!(
            html.contains(" feature"),
            "passthrough class merged: {html}"
        );
        assert!(
            html.contains("color: blue;"),
            "passthrough style merged: {html}"
        );
    }

    #[test]
    fn col_re_exports_resolve_via_silkenweb_module() {
        let el: Div<Dry> = crate::silkenweb::col(
            12,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            "",
            "",
            empty(),
        );
        assert_eq!(render(el), r#"<div class="gl-col"></div>"#);
    }
}
