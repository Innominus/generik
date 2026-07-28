use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{LazyLock, Mutex};

use leptos::prelude::*;

use crate::col_css::{compute_col_css, ColConfig};
use crate::shared::{merge_class, Gutter};

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
