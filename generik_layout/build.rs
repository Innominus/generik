//! Build script: globs `src/components/*.css` + `src/tokens.css`, concatenates
//! them (tokens first, then components alphabetically for determinism), wraps
//! the raw CSS in a `minify_css!(r#"..."#)` call, and writes the resulting
//! Rust source to `OUT_DIR/layout_css.rs`. `styles.rs` then `include!`s that
//! file, which defines `pub const LAYOUT_CSS: &str`. The `minify_css!`
//! proc-macro (from `generik_css_macros`) does the actual minification at
//! compile time (release-only; dev passes through for readable output) with
//! cross-component dedup via lightningcss.
//!
//! This indirection is necessary because `minify_css!` requires a string
//! *literal* at expansion time — `include_str!(...)` does not expand to a
//! `LitStr` before the proc-macro runs, so `minify_css!(include_str!(...))`
//! does not compile. Emitting the literal from `build.rs` sidesteps that while
//! keeping the source CSS split across per-component files.
//!
//! All the pure logic (collect / assemble / hash-run / emit) lives in
//! `build_logic.rs` so it can be unit-tested in `tests/build_css.rs` without
//! a build-script environment. This file is just the I/O + env-var wrapper.
//!
//! Transparent to library consumers — they just call `<LayoutStyles/>`.

#[path = "build_logic.rs"]
mod build_logic;

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // Re-run if any CSS file changes.
    println!("cargo:rerun-if-changed=src/tokens.css");
    println!("cargo:rerun-if-changed=src/components");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let tokens_path = manifest_dir.join("src/tokens.css");
    let components_dir = manifest_dir.join("src/components");

    // Also write the raw assembled CSS to OUT_DIR/layout.css for inspection/debugging.
    let tokens_css = fs::read_to_string(&tokens_path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", tokens_path.display()));
    let component_files = build_logic::collect_component_files(&components_dir);
    let component_css: Vec<String> = component_files
        .iter()
        .map(|p| {
            fs::read_to_string(p).unwrap_or_else(|e| panic!("failed to read {}: {e}", p.display()))
        })
        .collect();
    let assembled = build_logic::assemble_css(&tokens_css, &component_css);

    let src = build_logic::emit_layout_css_source(&assembled);

    let out_path = out_dir.join("layout_css.rs");
    fs::write(&out_path, src)
        .unwrap_or_else(|e| panic!("failed to write {}: {e}", out_path.display()));

    let raw_out_path = out_dir.join("layout.css");
    fs::write(&raw_out_path, &assembled)
        .unwrap_or_else(|e| panic!("failed to write {}: {e}", raw_out_path.display()));
}