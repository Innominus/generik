//! Pure, testable logic for the `generik_layout` build script.
//!
//! This module contains no I/O and no `env::var` calls — it takes paths/strings
//! in and returns strings out, so it can be `#[path]`-included from both
//! `build.rs` (build-script context) and `tests/build_css.rs` (test context).
//!
//! The pipeline is:
//!   1. `collect_component_files` — deterministic sorted glob of `*.css`.
//!   2. `assemble_css` — tokens first, then components alphabetically, joined
//!      with newlines (so adjacent selectors don't merge in the raw output;
//!      lightningcss merges them in release builds).
//!   3. `max_hash_run` — longest run of `#` inside the CSS, so the raw-string
//!      wrapper `r#"..."#` can pick a delimiter longer than any such run.
//!   4. `emit_layout_css_source` — wraps the assembled CSS in a
//!      `pub const LAYOUT_CSS: &str = minify_css!(r##"..."##);` Rust source.
//!
//! The actual minification + cross-component dedup happens inside the
//! `minify_css!` proc-macro (from `generik_css_macros`) at expansion time,
//! not here.

use std::path::{Path, PathBuf};

/// Sorted, deterministic list of `*.css` files in `dir` (empty if the dir
/// is missing — call sites treat a missing components dir as a hard error,
/// but tests pass a fixture dir they may legitimately leave empty).
pub fn collect_component_files(dir: &Path) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = match std::fs::read_dir(dir) {
        Ok(entries) => entries
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|ext| ext == "css"))
            .collect(),
        Err(_) => return Vec::new(),
    };
    files.sort();
    files
}

/// Concatenate `tokens` first, then each component file's contents, with a
/// newline between files. Returns the assembled CSS string only (component
/// paths are the caller's responsibility; this fn is pure string work).
pub fn assemble_css(tokens_css: &str, component_css: &[String]) -> String {
    let mut parts: Vec<String> = Vec::with_capacity(component_css.len() + 1);
    parts.push(tokens_css.to_string());
    for css in component_css {
        parts.push(css.clone());
    }
    parts.join("\n")
}

/// Length of the longest run of `#` characters *immediately following a `"`*
/// inside `css`. Only a `#`-run right after a `"` can prematurely close a raw
/// string `r#"..."#`; bare `#` runs elsewhere are harmless and ignored.
pub fn max_hash_run(css: &str) -> usize {
    css.split('"')
        .filter_map(|s| {
            let n = s.chars().take_while(|&c| c == '#').count();
            if n == 0 { None } else { Some(n) }
        })
        .max()
        .unwrap_or(0)
}

/// initialized from `minify_css!(r#"..."#)`, where the `#`-run in the raw
/// string delimiter is one longer than the longest `#`-run that immediately
/// follows a `"` inside `css` (the only thing that can close the raw string).
pub fn emit_layout_css_source(css: &str) -> String {
    let max_run = max_hash_run(css);
    let hashes = "#".repeat(max_run + 1);

    let mut src = String::new();
    src.push_str("use generik_css_macros::minify_css;\n\n");
    src.push_str("/// The single source of CSS truth for the crate.\n");
    src.push_str("///\n");
    src.push_str("/// Assembled at build time by `build.rs` from `src/tokens.css` (design tokens +\n");
    src.push_str("/// shared utility classes) and `src/components/*.css` (one per component), then\n");
    src.push_str("/// minified here at compile time via `minify_css!` (release-only; dev passes\n");
    src.push_str("/// through for readable output). lightningcss's `minify` merges identical\n");
    src.push_str("/// selectors across the concatenation, so cross-component dedup is real.\n");
    src.push_str("///\n");
    src.push_str("/// All classes are namespaced with `gl-`. Components reference class names from\n");
    src.push_str("/// this string; they do not inline their own CSS (the per-`Col` responsive\n");
    src.push_str("/// rules in `grid.rs` are the only exception, because they are per-instance).\n");
    src.push_str("///\n");
    src.push_str("/// Override the design tokens on `:root` to theme the layout.\n");
    src.push_str("pub const LAYOUT_CSS: &str = minify_css!(r");
    src.push_str(&hashes);
    src.push('"');
    src.push_str(css);
    src.push('"');
    src.push_str(&hashes);
    src.push_str(");\n");
    src
}

/// Full pipeline over the real crate tree: read `tokens_path`, glob
/// `components_dir/*.css`, assemble, and emit the `layout_css.rs` source.
/// Used by tests; `build.rs` calls the individual steps directly so it can
/// also write the raw `layout.css` debug artifact.
#[allow(dead_code)] // referenced only from tests/build_css.rs
pub fn build_layout_css_source(tokens_path: &Path, components_dir: &Path) -> String {
    let tokens_css = std::fs::read_to_string(tokens_path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", tokens_path.display()));

    let component_files = collect_component_files(components_dir);
    let component_css: Vec<String> = component_files
        .iter()
        .map(|p| {
            std::fs::read_to_string(p)
                .unwrap_or_else(|e| panic!("failed to read {}: {e}", p.display()))
        })
        .collect();

    let assembled = assemble_css(&tokens_css, &component_css);
    emit_layout_css_source(&assembled)
}