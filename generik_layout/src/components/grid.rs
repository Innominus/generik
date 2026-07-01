use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{LazyLock, Mutex};

use leptos::prelude::*;

use crate::shared::{merge_class, Gutter, BREAKPOINTS};

/// A centered 12-column CSS grid container. Use `Col` children for responsive
/// column spans; empty `max_width` uses the `--gl-max-width` token (capped + centered).
///
/// Props: `gap` (gap size, default `Md`), `max_width` (inline override or `""`
/// for the CSS default), `class`/`style` (passthrough).
#[component]
pub fn Grid(
    children: Children,
    #[prop(optional)] gap: Gutter,
    #[prop(default = "")] max_width: &'static str,
    #[prop(default = "")] class: &'static str,
    #[prop(default = "")] style: &'static str,
) -> impl IntoView {
    let base = merge_class("gl-grid", &format!("gl-gap{}", gap.class_suffix()));
    let class_str = merge_class(&base, class);

    let own_style = if max_width.is_empty() {
        String::new()
    } else {
        format!("max-width: {max_width};")
    };
    let style_str = if style.is_empty() {
        own_style
    } else if own_style.is_empty() {
        style.to_string()
    } else {
        format!("{own_style} {style}")
    };

    view! {
        <div class=class_str style=style_str>{children()}</div>
    }
}

static COL_COUNTER: AtomicUsize = AtomicUsize::new(0);

static COL_CSS_CACHE: LazyLock<Mutex<HashMap<ColConfig, (usize, String)>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct ColConfig {
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
}

impl ColConfig {
    /// True when the config matches the `Col` defaults (cols=12, no responsive
    /// cols, no starts) — i.e. no per-instance `<style>` is needed.
    ///
    /// This is the single source of truth for the `Col` early-return decision.
    /// `compute_col_css` has its own inline equivalent check (see the comment
    /// there) which must stay in sync with this method.
    fn is_all_default(&self) -> bool {
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

/// A column inside a `Grid`. Mobile-first responsive span/start with cascade
/// inheritance: the effective span at a breakpoint is the most recently specified
/// `cols_*` value going up from the base (same mental model as Tailwind).
///
/// Defaults to `cols = 12` with no responsive overrides, in which case no
/// per-instance `<style>` is emitted — the base `.gl-col` class handles it.
///
/// Props: `cols` (base span, default 12), `cols_sm`/`cols_md`/`cols_lg`/`cols_xl`/`cols_2xl`
/// (optional responsive spans), `start` + `start_*` (optional grid-column-start),
/// `class`/`style` (passthrough).
///
/// Note: `Col` assumes a 12-column parent grid. For `Masonry` (which may use a
/// different track count), set spans on children directly via their own classes.
#[component]
pub fn Col(
    children: Children,
    #[prop(default = 12)] cols: usize,
    #[prop(optional)] cols_sm: Option<usize>,
    #[prop(optional)] cols_md: Option<usize>,
    #[prop(optional)] cols_lg: Option<usize>,
    #[prop(optional)] cols_xl: Option<usize>,
    #[prop(optional)] cols_2xl: Option<usize>,
    #[prop(optional)] start: Option<usize>,
    #[prop(optional)] start_sm: Option<usize>,
    #[prop(optional)] start_md: Option<usize>,
    #[prop(optional)] start_lg: Option<usize>,
    #[prop(optional)] start_xl: Option<usize>,
    #[prop(optional)] start_2xl: Option<usize>,
    #[prop(default = "")] class: &'static str,
    #[prop(default = "")] style: &'static str,
) -> AnyView {
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
        return view! {
            <div class=class_str style=style>{children()}</div>
        }
        .into_any();
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

    view! {
        <style>{css}</style>
        <div class=class_str style=style>{children()}</div>
    }
    .into_any()
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
        css.push_str(&format!("@media (min-width: {min_w}px) {{ .{class_name} {{ grid-column: "));
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
        let res = compute_col_css(0, 12, None, None, None, None, None, None, None, None, None, None, None);
        assert!(res.is_none(), "all-default Col should yield None");
    }

    #[test]
    fn cols_only_6() {
        let (class, css) = compute_col_css(1, 6, None, None, None, None, None, None, None, None, None, None, None)
            .expect("non-default Col should yield Some");
        assert_eq!(class, "gl-col-1");
        assert!(css.contains(".gl-col-1 { grid-column: span 6; }"));
        assert!(!css.contains("@media"), "no media queries expected");
    }

    #[test]
    fn cols_md_6() {
        let (class, css) = compute_col_css(2, 12, None, Some(6), None, None, None, None, None, None, None, None, None)
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
        let (_class, css) = compute_col_css(3, 12, None, Some(6), None, Some(4), None, None, None, None, None, None, None)
            .expect("Some");
        assert!(css.contains(".gl-col-3 { grid-column: span 12; }"));
        assert!(css.contains("@media (min-width: 768px) { .gl-col-3 { grid-column: span 6; } }"));
        assert!(!css.contains("@media (min-width: 1024px)"), "lg block must be skipped (inherits md)");
        assert!(css.contains("@media (min-width: 1280px) { .gl-col-3 { grid-column: span 4; } }"));
    }

    #[test]
    fn start_base() {
        let (class, css) = compute_col_css(4, 6, None, None, None, None, None, Some(3), None, None, None, None, None)
            .expect("Some");
        assert_eq!(class, "gl-col-4");
        assert!(css.contains(".gl-col-4 { grid-column: 3 / span 6; }"));
        assert!(!css.contains("@media"), "no media queries expected");
    }

    #[test]
    fn start_md_only() {
        let (_class, css) = compute_col_css(5, 12, None, Some(6), None, None, None, None, None, Some(3), None, None, None)
            .expect("Some");
        assert!(css.contains(".gl-col-5 { grid-column: span 12; }"));
        assert!(css.contains("@media (min-width: 768px) { .gl-col-5 { grid-column: 3 / span 6; } }"));
    }

    #[test]
    fn start_inherits() {
        // cols=12, start_md=Some(3), cols_md=None, cols_lg=None, start_lg=None.
        // md: cols_md=None -> eff_cols=12, start_md=Some(3) -> eff_start=Some(3), has_override=true
        //     -> emit md block: 3 / span 12.
        // lg: both None -> has_override=false -> skip.
        let (_class, css) = compute_col_css(6, 12, None, None, None, None, None, None, None, Some(3), None, None, None)
            .expect("Some");
        let expected = ".gl-col-6 { grid-column: span 12; }\n\
@media (min-width: 768px) { .gl-col-6 { grid-column: 3 / span 12; } }\n";
        assert_eq!(css, expected);
    }

    #[test]
    fn class_name_format() {
        let (class, _css) = compute_col_css(42, 6, None, None, None, None, None, None, None, None, None, None, None)
            .expect("Some");
        assert_eq!(class, "gl-col-42");
    }

    #[test]
    fn start_base_with_default_cols() {
        // cols=12 (default), start=Some(3), all responsive None.
        // Before F1 fix this panicked; now it must yield Some with the base rule.
        let (class, css) = compute_col_css(7, 12, None, None, None, None, None, Some(3), None, None, None, None, None)
            .expect("cols=12 + base start must NOT be all-default");
        assert_eq!(class, "gl-col-7");
        assert!(css.contains(".gl-col-7 { grid-column: 3 / span 12; }"));
        assert!(!css.contains("@media"), "no responsive overrides -> no media queries");
    }

    #[test]
    fn start_base_and_start_md() {
        // start=Some(1) at base, start_md=Some(7) at md. Base rule uses start=1; md rule uses start=7.
        let (_class, css) = compute_col_css(8, 6, None, Some(6), None, None, None, Some(1), None, Some(7), None, None, None)
            .expect("Some");
        assert!(css.contains(".gl-col-8 { grid-column: 1 / span 6; }"), "base uses start=1");
        assert!(css.contains("@media (min-width: 768px) { .gl-col-8 { grid-column: 7 / span 6; } }"), "md uses start=7");
    }
}