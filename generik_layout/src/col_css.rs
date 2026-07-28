#![cfg_attr(not(any(feature = "leptos", feature = "silkenweb")), allow(dead_code))]
//! Framework-neutral responsive grid-column CSS generation.
//!
//! Extracted from the Leptos `Col` component so that both the Leptos adapter
//! (`components::grid`) and a future Silkenweb adapter share one source of
//! truth for the responsive 12-column rule generation. This module has no
//! framework dependency — only `std` and the crate's own breakpoint table
//! (`shared::BREAKPOINTS`).

use crate::shared::BREAKPOINTS;

/// Configuration for a `Col`'s responsive grid-column span and start, used as
/// the cache key for generated per-instance CSS.
///
/// All fields hold the *clamped* values (every span/start is at least 1); the
/// caller is responsible for clamping before constructing.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(crate) struct ColConfig {
    pub(crate) cols: usize,
    pub(crate) cols_sm: Option<usize>,
    pub(crate) cols_md: Option<usize>,
    pub(crate) cols_lg: Option<usize>,
    pub(crate) cols_xl: Option<usize>,
    pub(crate) cols_2xl: Option<usize>,
    pub(crate) start: Option<usize>,
    pub(crate) start_sm: Option<usize>,
    pub(crate) start_md: Option<usize>,
    pub(crate) start_lg: Option<usize>,
    pub(crate) start_xl: Option<usize>,
    pub(crate) start_2xl: Option<usize>,
}

impl ColConfig {
    /// True when the config matches the `Col` defaults (cols=12, no responsive
    /// cols, no starts) — i.e. no per-instance `<style>` is needed.
    ///
    /// This is the single source of truth for the `Col` early-return decision.
    /// `compute_col_css` has its own inline equivalent check (see the comment
    /// there) which must stay in sync with this method.
    pub(crate) fn is_all_default(&self) -> bool {
        self.cols == 12
            && self.cols_sm.is_none()
            && self.cols_md.is_none()
            && self.cols_lg.is_none()
            && self.cols_xl.is_none()
            && self.cols_2xl.is_none()
            && self.start.is_none()
            && self.start_sm.is_none()
            && self.start_md.is_none()
            && self.start_lg.is_none()
            && self.start_xl.is_none()
            && self.start_2xl.is_none()
    }
}

/// Pure CSS generation for a Col's responsive grid-column rules.
///
/// Returns `None` if all props are default (cols=12, no responsive cols, no starts) —
/// the caller then skips emitting a `<style>` block and relies on the `.gl-col` base
/// class. Returns `Some((class_name, css))` otherwise: `class_name` is the unique
/// `gl-col-{id}` class to apply to the div, and `css` is the full `<style>` block
/// contents (without `<style>` tags).
pub(crate) fn compute_col_css(
    id: usize,
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
) -> Option<(String, String)> {
    let responsive_cols = [cols_sm, cols_md, cols_lg, cols_xl, cols_2xl];
    let responsive_starts = [start_sm, start_md, start_lg, start_xl, start_2xl];

    // Must match `ColConfig::is_all_default` (see `Col`). Kept inline so this
    // pure function stays self-contained and unit-testable without a struct.
    let all_default = cols == 12
        && start.is_none()
        && responsive_cols.iter().all(Option::is_none)
        && responsive_starts.iter().all(Option::is_none);

    if all_default {
        return None;
    }

    let class_name = format!("gl-col-{id}");

    let mut css = String::new();
    css.push_str(&format!(".{class_name} {{ grid-column: "));
    if let Some(s) = start {
        css.push_str(&format!("{s} / span {cols}"));
    } else {
        css.push_str(&format!("span {cols}"));
    }
    css.push_str("; }\n");

    let mut eff_cols = cols;
    let mut eff_start = start;
    for (i, (&col_override, &start_override)) in responsive_cols
        .iter()
        .zip(responsive_starts.iter())
        .enumerate()
    {
        if let Some(c) = col_override {
            eff_cols = c;
        }
        if let Some(s) = start_override {
            eff_start = Some(s);
        }
        let has_override = col_override.is_some() || start_override.is_some();
        if !has_override {
            continue;
        }
        let (min_w, _name) = BREAKPOINTS.get(i).copied().unwrap_or((0, ""));
        css.push_str(&format!(
            "@media (min-width: {min_w}px) {{ .{class_name} {{ grid-column: "
        ));
        if let Some(s) = eff_start {
            css.push_str(&format!("{s} / span {eff_cols}"));
        } else {
            css.push_str(&format!("span {eff_cols}"));
        }
        css.push_str("; } }\n");
    }

    Some((class_name, css))
}

#[cfg(test)]
mod tests {
    use super::compute_col_css;

    #[test]
    fn all_default() {
        let res = compute_col_css(
            0, 12, None, None, None, None, None, None, None, None, None, None, None,
        );
        assert!(res.is_none(), "all-default Col should yield None");
    }

    #[test]
    fn cols_only_6() {
        let (class, css) = compute_col_css(
            1, 6, None, None, None, None, None, None, None, None, None, None, None,
        )
        .expect("non-default Col should yield Some");
        assert_eq!(class, "gl-col-1");
        assert!(css.contains(".gl-col-1 { grid-column: span 6; }"));
        assert!(!css.contains("@media"), "no media queries expected");
    }

    #[test]
    fn cols_md_6() {
        let (class, css) = compute_col_css(
            2,
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
        )
        .expect("Some");
        assert_eq!(class, "gl-col-2");
        assert!(css.contains(".gl-col-2 { grid-column: span 12; }"));
        assert!(css.contains("@media (min-width: 768px) { .gl-col-2 { grid-column: span 6; } }"));
        // Only one media query expected (md).
        let media_count = css.matches("@media").count();
        assert_eq!(media_count, 1, "expected exactly one @media block (md)");
    }

    #[test]
    fn inheritance() {
        let (_class, css) = compute_col_css(
            3,
            12,
            None,
            Some(6),
            None,
            Some(4),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .expect("Some");
        assert!(css.contains(".gl-col-3 { grid-column: span 12; }"));
        assert!(css.contains("@media (min-width: 768px) { .gl-col-3 { grid-column: span 6; } }"));
        assert!(
            !css.contains("@media (min-width: 1024px)"),
            "lg block must be skipped (inherits md)"
        );
        assert!(css.contains("@media (min-width: 1280px) { .gl-col-3 { grid-column: span 4; } }"));
    }

    #[test]
    fn start_base() {
        let (class, css) = compute_col_css(
            4,
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
        )
        .expect("Some");
        assert_eq!(class, "gl-col-4");
        assert!(css.contains(".gl-col-4 { grid-column: 3 / span 6; }"));
        assert!(!css.contains("@media"), "no media queries expected");
    }

    #[test]
    fn start_md_only() {
        let (_class, css) = compute_col_css(
            5,
            12,
            None,
            Some(6),
            None,
            None,
            None,
            None,
            None,
            Some(3),
            None,
            None,
            None,
        )
        .expect("Some");
        assert!(css.contains(".gl-col-5 { grid-column: span 12; }"));
        assert!(
            css.contains("@media (min-width: 768px) { .gl-col-5 { grid-column: 3 / span 6; } }")
        );
    }

    #[test]
    fn start_inherits() {
        // cols=12, start_md=Some(3), cols_md=None, cols_lg=None, start_lg=None.
        // md: cols_md=None -> eff_cols=12, start_md=Some(3) -> eff_start=Some(3), has_override=true
        //     -> emit md block: 3 / span 12.
        // lg: both None -> has_override=false -> skip.
        let (_class, css) = compute_col_css(
            6,
            12,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(3),
            None,
            None,
            None,
        )
        .expect("Some");
        let expected = ".gl-col-6 { grid-column: span 12; }\n\
@media (min-width: 768px) { .gl-col-6 { grid-column: 3 / span 12; } }\n";
        assert_eq!(css, expected);
    }

    #[test]
    fn class_name_format() {
        let (class, _css) = compute_col_css(
            42, 6, None, None, None, None, None, None, None, None, None, None, None,
        )
        .expect("Some");
        assert_eq!(class, "gl-col-42");
    }

    #[test]
    fn start_base_with_default_cols() {
        // cols=12 (default), start=Some(3), all responsive None.
        // Before F1 fix this panicked; now it must yield Some with the base rule.
        let (class, css) = compute_col_css(
            7,
            12,
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
        )
        .expect("cols=12 + base start must NOT be all-default");
        assert_eq!(class, "gl-col-7");
        assert!(css.contains(".gl-col-7 { grid-column: 3 / span 12; }"));
        assert!(
            !css.contains("@media"),
            "no responsive overrides -> no media queries"
        );
    }

    #[test]
    fn start_base_and_start_md() {
        // start=Some(1) at base, start_md=Some(7) at md. Base rule uses start=1; md rule uses start=7.
        let (_class, css) = compute_col_css(
            8,
            6,
            None,
            Some(6),
            None,
            None,
            None,
            Some(1),
            None,
            Some(7),
            None,
            None,
            None,
        )
        .expect("Some");
        assert!(
            css.contains(".gl-col-8 { grid-column: 1 / span 6; }"),
            "base uses start=1"
        );
        assert!(
            css.contains("@media (min-width: 768px) { .gl-col-8 { grid-column: 7 / span 6; } }"),
            "md uses start=7"
        );
    }
}
