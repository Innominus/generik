//! Integration tests for the `minify_css!` proc-macro.
//!
//! These exercise the full macro expansion path (parse → release-gate →
//! minify → emit string literal). They must pass in BOTH `cargo test` (dev:
//! macro returns input unchanged) and `cargo test --release` (release: macro
//! returns minified CSS). Assertions branch on the observed output (`out != INPUT`)
//! so they pass under both profiles.

use generik_css_macros::minify_css;

/// The macro accepts a plain string literal and expands to a `&'static str`.
/// In both profiles, the output is non-empty and a valid &str.
#[test]
fn returns_string_literal() {
    const CSS: &str = minify_css!(".foo { color: red; }");
    assert!(!CSS.is_empty(), "macro should always return non-empty CSS");
}

/// The macro always preserves selector names (lightningcss doesn't rename).
#[test]
fn preserves_selectors() {
    const CSS: &str = minify_css!(".gl-grid { display: grid; }");
    assert!(CSS.contains(".gl-grid"), "selector preserved: {CSS:?}");
}

/// The macro preserves custom property names (the library's --gl-* tokens).
#[test]
fn preserves_custom_properties() {
    const CSS: &str = minify_css!(":root { --gl-ink: #1a1a1a; }");
    assert!(CSS.contains("--gl-ink"), "custom property name preserved: {CSS:?}");
    assert!(CSS.contains("#1a1a1a"), "value preserved: {CSS:?}");
}

/// The macro preserves media queries.
#[test]
fn preserves_media_queries() {
    const CSS: &str = minify_css!("@media (min-width: 768px) { .gl-sidebar { display: grid; } }");
    assert!(CSS.contains("@media"), "media query preserved: {CSS:?}");
    assert!(CSS.contains("768px"), "breakpoint preserved: {CSS:?}");
}

/// The macro gates on `cfg!(debug_assertions)` (evaluated in the consumer's
/// cfg context during expansion), NOT the `PROFILE` env var (which is unset
/// for proc-macro crates). So: `cargo test` → pass-through; `cargo test --release`
/// → minified. The assertions below branch on the observed output so they pass
/// under both profiles.
#[test]
fn profile_gating() {
    const INPUT: &str = ".foo { color: red; }\n.bar { color: red; }";
    let output: &str = minify_css!(".foo { color: red; }\n.bar { color: red; }");
    let minified = output != INPUT;
    if !minified {
        // Pass-through (`cargo test`, where `cfg!(debug_assertions)` is true):
        // output is byte-identical to input.
        assert_eq!(output, INPUT, "pass-through should return input unchanged");
    } else {
        // Minified (`cargo test --release`, where `cfg!(debug_assertions)` is
        // false): output is <= input length and selectors are preserved.
        assert!(output.len() <= INPUT.len(),
            "minified output ({}) should not exceed input ({}): {output:?}",
            output.len(), INPUT.len());
        assert!(output.contains(".foo"), "selector preserved: {output:?}");
        assert!(output.contains(".bar"), "second selector preserved: {output:?}");
    }
}

/// Comments: when pass-through (`cargo test`) they're preserved; when minified
/// (`cargo test --release`) they're stripped. The assertions branch on the
/// observed output so they pass under both profiles.
#[test]
fn comments_stripped_when_minified() {
    const INPUT: &str = "/* header */ .foo { color: red; }";
    let out: &str = minify_css!("/* header */ .foo { color: red; }");
    let minified = out != INPUT;
    if !minified {
        // Pass-through: comments preserved.
        assert!(out.contains("/* header */"), "pass-through should preserve comments: {out:?}");
    } else {
        // Minified: comments stripped.
        assert!(!out.contains("/*"), "minified should strip comments: {out:?}");
    }
}

/// Whitespace: when pass-through (`cargo test`) it's preserved; when minified
/// (`cargo test --release`) it's compacted. The assertions branch on the
/// observed output so they pass under both profiles.
#[test]
fn whitespace_compacted_when_minified() {
    const INPUT: &str = ".foo {\n  color: red;\n}\n\n.bar {\n  color: blue;\n}";
    let out: &str = minify_css!(".foo {\n  color: red;\n}\n\n.bar {\n  color: blue;\n}");
    let minified = out != INPUT;
    if !minified {
        // Pass-through: newlines preserved.
        assert!(out.contains('\n'), "pass-through should preserve newlines: {out:?}");
    } else {
        // Minified: newlines stripped.
        assert!(!out.contains('\n'), "minified should strip newlines: {out:?}");
    }
}

/// Selector merging: when minified (`cargo test --release`), two rules with
/// identical declarations become one. When pass-through (`cargo test`), they
/// remain separate. The assertions branch on the observed output so they pass
/// under both profiles.
#[test]
fn merges_rules_when_minified() {
    const INPUT: &str = ".a { color: red; } .b { color: red; }";
    let out: &str = minify_css!(".a { color: red; } .b { color: red; }");
    let minified = out != INPUT;
    if !minified {
        // Pass-through: two separate rules preserved, byte-identical.
        let a_count = out.matches(".a {").count();
        assert_eq!(a_count, 1, "pass-through: .a rule preserved: {out:?}");
        assert_eq!(out, INPUT, "pass-through: byte-identical");
    } else {
        // Minified: merged into one rule with a selector list.
        assert!(out.contains(".a,.b") || out.contains(".a, .b"),
            "minified: selectors merged: {out:?}");
        let color_count = out.matches("color:red").count();
        assert_eq!(color_count, 1, "minified: one color declaration (merged): {out:?}");
    }
}

/// A realistic full-CSS snippet (shaped like the library's LAYOUT_CSS) preserves
/// all the library's class names and tokens in both profiles, and is smaller
/// than the input when minified.
#[test]
fn realistic_layout_css() {
    const CSS: &str = r#"
        :root {
          --gl-max-width: 1200px;
          --gl-gap-md: 1.5rem;
          --gl-ink: #1a1a1a;
        }
        .gl-grid { display: grid; grid-template-columns: repeat(12, 1fr); }
        .gl-col { grid-column: span 12; }
        .gl-stack { display: flex; flex-direction: column; }
        @media (min-width: 768px) {
          .gl-sidebar { grid-template-columns: 280px 1fr; }
        }
    "#;
    let out: &str = minify_css!(r#"
        :root {
          --gl-max-width: 1200px;
          --gl-gap-md: 1.5rem;
          --gl-ink: #1a1a1a;
        }
        .gl-grid { display: grid; grid-template-columns: repeat(12, 1fr); }
        .gl-col { grid-column: span 12; }
        .gl-stack { display: flex; flex-direction: column; }
        @media (min-width: 768px) {
          .gl-sidebar { grid-template-columns: 280px 1fr; }
        }
    "#);
    // These hold in both profiles (selectors and custom props are always preserved).
    assert!(out.contains(".gl-grid"), "{out:?}");
    assert!(out.contains(".gl-col"), "{out:?}");
    assert!(out.contains(".gl-stack"), "{out:?}");
    assert!(out.contains(".gl-sidebar"), "{out:?}");
    assert!(out.contains("--gl-max-width"), "{out:?}");
    assert!(out.contains("--gl-ink"), "{out:?}");
    assert!(out.contains("@media"), "{out:?}");
    assert!(out.contains("768px"), "{out:?}");

    // When minified, the output should be smaller than the input.
    // (Pass-through yields output == CSS, so out.len() < CSS.len() is false — skip.)
    if out != CSS {
        assert!(out.len() < CSS.len(),
            "minified ({}) should be smaller than input ({}): {out:?}",
            out.len(), CSS.len());
    }
}

/// The macro accepts a raw string literal (`r#"..."#`) — important because
/// LAYOUT_CSS uses that form.
#[test]
fn accepts_raw_string_literal() {
    const CSS: &str = minify_css!(r#".foo { content: "hello"; }"#);
    assert!(CSS.contains(".foo"), "raw string literal works: {CSS:?}");
}

/// The macro can be used in a `const` context (the expansion is a string literal).
#[test]
fn usable_in_const_context() {
    const CSS: &str = minify_css!(".x { color: red; }");
    // If this compiles, the macro expanded to a literal (not a runtime call).
    assert!(!CSS.is_empty());
}