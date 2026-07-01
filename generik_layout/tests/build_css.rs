//! Integration tests for the `generik_layout` build-time CSS pipeline.
//!
//! These exercise the pure logic extracted into `build_logic.rs` — the same
//! module `build.rs` includes via `#[path]` — against the real crate CSS tree
//! (`src/tokens.css` + `src/components/*.css`) and against synthetic fixtures
//! for the edge cases the real tree doesn't cover.
//!
//! What's under test:
//!   - `collect_component_files` — deterministic sorted glob.
//!   - `assemble_css` — tokens-first, alphabetical, newline-joined.
//!   - `max_hash_run` / `emit_layout_css_source` — raw-string delimiter safety.
//!   - lightningcss minification + cross-component selector dedup, run
//!     directly on the assembled CSS (the proc-macro does this at compile
//!     time; here we drive the same `lightningcss` API to assert properties of
//!     the output without doing a brittle text comparison).
//!
//! Assertions are on byte-size bounds and structural properties (non-empty,
//! shrinks under minification, no duplicate selectors survive, every
//! `gl-` component class from the source survives) rather than exact text —
//! minifier formatting is not a stable contract.

#[path = "../build_logic.rs"]
mod build_logic;

use std::path::PathBuf;

/// Drive the same lightningcss minify pipeline the `minify_css!` proc-macro
/// uses (release path). Returns the minified CSS string, or panics on
/// parse/minify/serialize error — mirroring the macro's `unwrap_or_else(raw)`
/// fallback would hide failures from the assertions, so here we want hard
/// failures during testing.
fn minify(css: &str) -> String {
    use lightningcss::stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet};
    let mut sheet = StyleSheet::parse(css, ParserOptions::default())
        .expect("lightningcss parse failed");
    sheet.minify(MinifyOptions::default())
        .expect("lightningcss minify failed");
    let out = sheet
        .to_css(PrinterOptions {
            minify: true,
            ..PrinterOptions::default()
        })
        .expect("lightningcss serialize failed");
    out.code
}

/// Locate the crate's `src/` dir from the test working dir
/// (`generik_layout/tests/` → `generik_layout/src/`).
fn crate_src_dir() -> PathBuf {
    // CARGO_MANIFEST_DIR points at generik_layout/ during test builds.
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest.join("src")
}

// ---------------------------------------------------------------------------
// collect_component_files
// ---------------------------------------------------------------------------

#[test]
fn collect_finds_all_component_css_files() {
    let dir = crate_src_dir().join("components");
    let files = build_logic::collect_component_files(&dir);
    // Every entry is a .css file.
    assert!(files.iter().all(|p| p.extension().unwrap() == "css"));
    // The crate ships 20 component CSS files (grid, stack, cluster, sidebar,
    // center, masonry, figure, media_object, pull_quote, card, container,
    // measure, rule, metric, stat, teaser, masthead, index_item,
    // sticky_header, dropcap via its own file if present).
    assert!(
        files.len() >= 18,
        "expected at least 18 component CSS files, found {}: {files:?}",
        files.len()
    );
}

#[test]
fn collect_is_sorted_alphabetically() {
    let dir = crate_src_dir().join("components");
    let files = build_logic::collect_component_files(&dir);
    let names: Vec<String> = files
        .iter()
        .map(|p| p.file_name().unwrap().to_string_lossy().into_owned())
        .collect();
    let mut sorted = names.clone();
    sorted.sort();
    assert_eq!(names, sorted, "component files must be sorted for determinism");
}

#[test]
fn collect_ignores_non_css_files() {
    let dir = crate_src_dir().join("components");
    let files = build_logic::collect_component_files(&dir);
    // The components dir also contains .rs files; none should leak through.
    assert!(
        files.iter().all(|p| p.extension().unwrap() == "css"),
        "non-.css files leaked into collect_component_files"
    );
}

#[test]
fn collect_missing_dir_returns_empty() {
    let files = build_logic::collect_component_files(&PathBuf::from("/nonexistent/dir/xyz"));
    assert!(files.is_empty(), "missing dir should yield empty, not panic");
}

// ---------------------------------------------------------------------------
// assemble_css
// ---------------------------------------------------------------------------

#[test]
fn assemble_puts_tokens_first() {
    let out = build_logic::assemble_css("TOKENS", &["C1".to_string(), "C2".to_string()]);
    // Tokens block comes before any component block.
    let tokens_idx = out.find("TOKENS").unwrap();
    let c1_idx = out.find("C1").unwrap();
    assert!(tokens_idx < c1_idx, "tokens must precede components");
}

#[test]
fn assemble_joins_with_newlines() {
    let out = build_logic::assemble_css("TOKENS", &["C1".to_string(), "C2".to_string()]);
    // Three parts → exactly two newline separators between them (plus any
    // internal newlines; here the inputs are single-line so we can assert
    // exactly).
    assert_eq!(out, "TOKENS\nC1\nC2");
}

#[test]
fn assemble_empty_components_yields_just_tokens() {
    let out = build_logic::assemble_css("TOKENS", &[]);
    assert_eq!(out, "TOKENS");
}

// ---------------------------------------------------------------------------
// max_hash_run / emit_layout_css_source
// ---------------------------------------------------------------------------

#[test]
fn max_hash_run_counts_longest_run_after_quote() {
    // The function measures the longest run of `#` immediately following a
    // `"` in the CSS — because only a `#`-run right after a `"` can
    // prematurely close a raw string `r#"..."#`. A bare `###` elsewhere is
    // harmless and correctly ignored. Strings are built via `concat!` to
    // avoid raw-string lexing ambiguity from embedded `"##` runs.
    assert_eq!(build_logic::max_hash_run("no hashes here"), 0);
    assert_eq!(build_logic::max_hash_run("a # b ## c ### d"), 0);
    // A quoted string with a leading hash run: the segment after the `"`
    // starts with `##`, so the run is 2.
    let two = concat!("content: \"", "##", "\"; color: red;");
    assert_eq!(build_logic::max_hash_run(two), 2);
    // Three hashes right after a quote — the real threat.
    let three = concat!("content: \"", "###", "\"; /* x */");
    assert_eq!(build_logic::max_hash_run(three), 3);
    // No `#` after the quote → 0 despite other hashes in the string.
    let none = concat!("content: \"", "x#", "\"; a: #fff;");
    assert_eq!(build_logic::max_hash_run(none), 0);
}

#[test]
fn emit_produces_compilable_const_declaration() {
    let src = build_logic::emit_layout_css_source(".foo { color: red; }");
    assert!(
        src.contains("pub const LAYOUT_CSS: &str = minify_css!("),
        "emitted source must define LAYOUT_CSS via minify_css!"
    );
    assert!(
        src.contains("use generik_css_macros::minify_css;"),
        "emitted source must import the macro"
    );
}

#[test]
fn emit_picks_safe_hash_delimiter() {
    // CSS containing a run of N hashes immediately after a `"` (the only
    // thing that can prematurely close a raw string) must get a delimiter
    // with N+1 hashes, so the terminator can't appear inside the content.
    // Built via concat! to avoid raw-string lexing ambiguity.
    let css = concat!("content: \"", "###", "\"; .x { color: red; }");
    let src = build_logic::emit_layout_css_source(css);
    // The delimiter run is the longest unbroken `#` sequence in the emitted
    // raw string opener (after `r` and before the closing quote).
    let longest = src
        .split('"')
        .filter_map(|s| {
            let n = s.chars().take_while(|&c| c == '#').count();
            if n == 0 { None } else { Some(n) }
        })
        .max()
        .unwrap_or(0);
    assert_eq!(longest, 4, "delimiter must be one longer than the 3-hash run after a quote in CSS");
}

#[test]
fn emit_no_hashes_in_css_uses_min_delimiter() {
    let src = build_logic::emit_layout_css_source(".foo { color: red; }");
    // No `#` in the CSS → minimal delimiter of a single `#`.
    assert!(
        src.contains("r#\""),
        "no-hash CSS should use the minimal r#\" delimiter"
    );
}

#[test]
fn emit_preserves_css_content_verbatim() {
    let css = ".gl-x { color: var(--gl-accent); }";
    let src = build_logic::emit_layout_css_source(css);
    assert!(
        src.contains(css),
        "CSS content must appear verbatim inside the raw string"
    );
}

// ---------------------------------------------------------------------------
// Full pipeline against the real crate CSS tree
// ---------------------------------------------------------------------------

#[test]
fn real_tree_assembles_non_empty_css() {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let tokens_path = manifest.join("src/tokens.css");
    let components_dir = manifest.join("src/components");

    let tokens_css = std::fs::read_to_string(&tokens_path).unwrap();
    let files = build_logic::collect_component_files(&components_dir);
    let component_css: Vec<String> = files
        .iter()
        .map(|p| std::fs::read_to_string(p).unwrap())
        .collect();
    let assembled = build_logic::assemble_css(&tokens_css, &component_css);

    assert!(!assembled.trim().is_empty(), "assembled CSS must be non-empty");
    // Sanity: the assembled CSS is on the order of ~11 KB of source (tokens
    // ~4 KB + 20 component files ~7 KB). Assert a loose floor + ceiling so
    // the test survives minor edits but catches a gross regression (e.g. a
    // missing tokens file or a doubled assembly).
    let len = assembled.len();
    assert!(
        (4000..=40000).contains(&len),
        "assembled CSS length {len} outside the 4–40 KB expected band"
    );
}

#[test]
fn real_tree_emitted_source_is_self_consistent() {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let src = build_logic::build_layout_css_source(
        &manifest.join("src/tokens.css"),
        &manifest.join("src/components"),
    );
    assert!(src.contains("pub const LAYOUT_CSS: &str = minify_css!("));
    // The emitted source should contain every `gl-` class name that appears
    // in the source CSS (round-trip: nothing dropped during assembly).
    let tokens = std::fs::read_to_string(manifest.join("src/tokens.css")).unwrap();
    for line in tokens.lines() {
        if let Some(class) = line.trim_start().strip_prefix(".gl-").and_then(|rest| {
            rest.find('{').map(|i| &rest[..i])
        }) {
            let needle = format!(".gl-{}", class.trim());
            assert!(
                src.contains(&needle),
                "emitted source lost class `{needle}` present in tokens.css"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// lightningcss minification + cross-component dedup
// ---------------------------------------------------------------------------

#[test]
fn minified_output_is_smaller_than_raw() {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let tokens_css = std::fs::read_to_string(manifest.join("src/tokens.css")).unwrap();
    let files = build_logic::collect_component_files(&manifest.join("src/components"));
    let component_css: Vec<String> = files
        .iter()
        .map(|p| std::fs::read_to_string(p).unwrap())
        .collect();
    let assembled = build_logic::assemble_css(&tokens_css, &component_css);

    let minified = minify(&assembled);
    assert!(
        minified.len() < assembled.len(),
        "minified CSS ({} B) must be smaller than raw ({} B)",
        minified.len(),
        assembled.len()
    );
}

#[test]
fn minified_output_size_in_expected_band() {
    // The minified bundle should land in a stable KB band. As of the current
    // tree the minified output is ~5–7 KB; assert 2–12 KB to tolerate token
    // tweaks while catching a gross regression (empty output, unminified
    // passthrough, or a 10x blowup from a broken concatenation).
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let tokens_css = std::fs::read_to_string(manifest.join("src/tokens.css")).unwrap();
    let files = build_logic::collect_component_files(&manifest.join("src/components"));
    let component_css: Vec<String> = files
        .iter()
        .map(|p| std::fs::read_to_string(p).unwrap())
        .collect();
    let assembled = build_logic::assemble_css(&tokens_css, &component_css);
    let minified = minify(&assembled);

    let kb = minified.len() as f64 / 1024.0;
    assert!(
        (2.0..=12.0).contains(&kb),
        "minified CSS is {kb:.1} KB, outside the 2–12 KB expected band"
    );
}

#[test]
fn minify_dedups_identical_selectors_across_files() {
    // Two files with the same selector body should merge to a single rule
    // after minification — this is the cross-component dedup the build.rs
    // doc promises.
    let css = ".gl-a { color: red; }\n.gl-a { color: red; }\n.gl-b { color: blue; }";
    let minified = minify(css);
    // `.gl-a{color:red}` should appear exactly once.
    let occurrences = minified.matches(".gl-a{color:red}").count();
    assert_eq!(
        occurrences, 1,
        "duplicate .gl-a rule should be merged to one occurrence, got: {minified}"
    );
}

#[test]
fn minify_dedups_repeated_selector_across_fake_components() {
    // Simulate the real scenario: two "components" each contributing the same
    // utility class. Assemble them through the build pipeline and minify.
    let tokens = "";
    let component_css = vec![
        ".gl-shared { gap: 1rem; }".to_string(),
        ".gl-shared { gap: 1rem; }".to_string(),
    ];
    let assembled = build_logic::assemble_css(tokens, &component_css);
    let minified = minify(&assembled);
    assert_eq!(
        minified.matches(".gl-shared{gap:1rem}").count(),
        1,
        "two identical .gl-shared rules must dedup to one, got: {minified}"
    );
}

#[test]
fn minified_output_preserves_all_gl_component_classes() {
    // Every `gl-` class present in the source CSS must survive minification.
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let tokens_css = std::fs::read_to_string(manifest.join("src/tokens.css")).unwrap();
    let files = build_logic::collect_component_files(&manifest.join("src/components"));
    let component_css: Vec<String> = files
        .iter()
        .map(|p| std::fs::read_to_string(p).unwrap())
        .collect();
    let assembled = build_logic::assemble_css(&tokens_css, &component_css);
    let minified = minify(&assembled);

    // Collect every `.gl-xxx` selector name from the raw source.
    let mut source_classes: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for line in assembled.lines() {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix(".gl-") {
            if let Some(brace) = rest.find('{') {
                let name = rest[..brace].trim().to_string();
                // Skip compound selectors with combinators / pseudo compound
                // forms like `gl-x::first-letter` — only assert simple class
                // names where the selector is just `.gl-name`.
                if !name.contains(' ') && !name.contains('.') && !name.contains(':') {
                    source_classes.insert(format!("gl-{}", name));
                }
            }
        }
    }

    assert!(!source_classes.is_empty(), "no simple gl- classes found in source");
    for class in &source_classes {
        let needle = format!(".{}", class);
        assert!(
            minified.contains(&needle),
            "minified output lost class `{needle}`"
        );
    }
}

#[test]
fn minified_output_is_valid_css_round_trip() {
    // The minified output must itself be parseable by lightningcss (no
    // broken syntax introduced by the minifier). Re-parsing is a structural
    // validity check.
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let tokens_css = std::fs::read_to_string(manifest.join("src/tokens.css")).unwrap();
    let files = build_logic::collect_component_files(&manifest.join("src/components"));
    let component_css: Vec<String> = files
        .iter()
        .map(|p| std::fs::read_to_string(p).unwrap())
        .collect();
    let assembled = build_logic::assemble_css(&tokens_css, &component_css);
    let minified = minify(&assembled);

    // Re-parse the minified output; if it fails, this will panic via expect.
    use lightningcss::stylesheet::{ParserOptions, StyleSheet};
    let _ = StyleSheet::parse(&minified, ParserOptions::default())
        .expect("minified output must re-parse as valid CSS");
}

// ---------------------------------------------------------------------------
// Determinism
// ---------------------------------------------------------------------------

#[test]
fn assembly_is_deterministic_across_calls() {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let tokens_css = std::fs::read_to_string(manifest.join("src/tokens.css")).unwrap();
    let files = build_logic::collect_component_files(&manifest.join("src/components"));
    let component_css: Vec<String> = files
        .iter()
        .map(|p| std::fs::read_to_string(p).unwrap())
        .collect();

    let a = build_logic::assemble_css(&tokens_css, &component_css);
    let b = build_logic::assemble_css(&tokens_css, &component_css);
    assert_eq!(a, b, "assemble_css must be deterministic");
}

#[test]
fn emit_is_deterministic_across_calls() {
    let css = ".foo { color: red; } .bar { color: blue; }";
    let a = build_logic::emit_layout_css_source(css);
    let b = build_logic::emit_layout_css_source(css);
    assert_eq!(a, b, "emit_layout_css_source must be deterministic");
}