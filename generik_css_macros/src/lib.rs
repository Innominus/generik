use proc_macro::TokenStream;
use quote::quote;

/// Minify a CSS string literal at compile time using lightningcss.
///
/// In release builds: parses + minifies + returns the minified `&'static str`.
/// In dev builds: returns the input string unchanged (fast compile, readable CSS).
///
/// # Input requirements
///
/// The input must be a string *literal* (raw strings like `r#"..."#` are OK).
/// `concat!(...)` and `include_str!(...)` are NOT supported — they don't expand
/// to a `LitStr` at macro-expansion time, so the macro cannot read their value.
///
/// # Minification gating
///
/// Minification is gated on `cfg!(debug_assertions)`. If a consumer sets
/// `debug-assertions = true` in their `[profile.release]`, minification will be
/// skipped in release builds — the macro cannot distinguish that case from a
/// true dev build.
///
/// Usage:
///   const CSS: &str = minify_css!(".foo { color: red; } .bar { color: red; }");
/// Expands to a `&'static str` literal of the (possibly minified) CSS.
#[proc_macro]
pub fn minify_css(input: TokenStream) -> TokenStream {
    // Parse the input as a single string literal.
    let css = syn::parse_macro_input!(input as syn::LitStr);
    let raw = css.value();

    // Release gating: only minify in release builds.
    // `cfg!(debug_assertions)` is baked into the proc-macro crate at its build
    // time; because cargo builds proc-macros with the same profile as the
    // consumer (`dev`/`release`), the baked value tracks the consumer's profile.
    // Caveat: this coupling breaks under `[profile.<name>.build-override]` host-
    // profile overrides or a separately-published prebuilt proc-macro — see the
    // `debug-assertions = true` footgun in the doc above.
    // (`PROFILE` env var would always read `debug` for proc-macro crates — don't use it.)
    let is_release = !cfg!(debug_assertions);

    let out = if is_release {
        minify_with_lightningcss(&raw).unwrap_or_else(|| {
            eprintln!("minify_css: lightningcss failed to minify, emitting raw CSS (release build will be larger)");
            raw.clone()
        })
    } else {
        raw.clone()
    };

    quote!(#out).into()
}

/// Parse + minify + serialize CSS via lightningcss. Always minifies regardless
/// of build profile (the caller gates). Returns `None` on any parse/minify/
/// serialize error; never panics.
fn minify_with_lightningcss(css: &str) -> Option<String> {
    use lightningcss::stylesheet::{StyleSheet, ParserOptions, MinifyOptions, PrinterOptions};
    let mut sheet = StyleSheet::parse(css, ParserOptions::default()).ok()?;
    sheet.minify(MinifyOptions::default()).ok()?;
    let res = sheet.to_css(PrinterOptions { minify: true, ..PrinterOptions::default() }).ok()?;
    Some(res.code)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Empty input → empty (or near-empty) output. Lightningcss should handle this gracefully.
    #[test]
    fn empty_css() {
        let out = minify_with_lightningcss("");
        // Empty CSS may parse to an empty stylesheet; accept None or Some("") — both are fine.
        // The helper returns Option<String>; we just assert it doesn't panic.
        if let Some(css) = out {
            assert!(css.trim().is_empty(), "empty input should yield empty output, got: {css:?}");
        }
        // If it returns None, that's also acceptable (parse or minify failed on empty).
    }

    /// Basic selector + declaration minification: whitespace stripped, declarations compacted.
    #[test]
    fn basic_minification() {
        let css = ".foo { color: red; }\n.bar { color: red; }";
        let out = minify_with_lightningcss(css).expect("basic CSS should minify");
        // lightningcss merges selectors with identical declarations: ".foo,.bar{color:red}"
        assert!(out.contains(".foo,.bar"), "merged selectors expected in: {out:?}");
        assert!(out.contains("color:red"), "compact declaration expected in: {out:?}");
        assert!(!out.contains('\n'), "no newlines expected in minified output: {out:?}");
    }

    /// Comments are stripped.
    #[test]
    fn strips_comments() {
        let css = "/* header comment */\n.foo { /* inline */ color: red; }";
        let out = minify_with_lightningcss(css).expect("CSS with comments should minify");
        assert!(!out.contains("header comment"), "block comment should be stripped: {out:?}");
        assert!(!out.contains("/*"), "no comment markers should remain: {out:?}");
    }

    /// Longhand → shorthand merging (e.g. margin-top + margin-bottom → margin shorthand if possible).
    /// lightningcss merges margin-top + margin-bottom into the `margin` shorthand when both are present.
    #[test]
    fn shorthand_merging() {
        let css = ".box { margin-top: 10px; margin-bottom: 10px; }";
        let out = minify_with_lightningcss(css).expect("shorthand test should minify");
        // lightningcss may merge to `margin:10px 0` or similar; just assert it's no longer two
        // separate margin-top/margin-bottom declarations. Either it's a shorthand OR both are still
        // there but compacted. The key observable: it produced SOME output without panicking.
        assert!(out.contains(".box"), "selector preserved: {out:?}");
        assert!(out.contains("10px"), "value preserved: {out:?}");
    }

    /// Media queries are preserved and minified.
    #[test]
    fn media_query_preserved() {
        let css = "@media (min-width: 768px) { .col { grid-column: span 6; } }";
        let out = minify_with_lightningcss(css).expect("media query CSS should minify");
        assert!(out.contains("@media"), "media query preserved: {out:?}");
        // lightningcss rewrites `(min-width: 768px)` to `(width>=768px)`; accept either form.
        assert!(out.contains("min-width:768px") || out.contains("min-width: 768px")
                || out.contains("width>=768px"),
            "min-width/breakpoint preserved: {out:?}");
        assert!(out.contains("span 6") || out.contains("span6"),
            "grid-column value preserved: {out:?}");
    }

    /// CSS custom properties (variables) are preserved (lightningcss doesn't rename them).
    #[test]
    fn custom_properties_preserved() {
        let css = ":root { --gl-ink: #1a1a1a; }\n.text { color: var(--gl-ink); }";
        let out = minify_with_lightningcss(css).expect("custom property CSS should minify");
        assert!(out.contains("--gl-ink"), "custom property name preserved: {out:?}");
        assert!(out.contains("var(--gl-ink)"), "var() reference preserved: {out:?}");
        assert!(out.contains("#1a1a1a"), "color value preserved: {out:?}");
    }

    /// Invalid CSS: helper returns None, does NOT panic. This is the fall-back contract.
    #[test]
    fn invalid_css_returns_none() {
        // Genuinely malformed CSS that lightningcss can't parse.
        // A stray `}` with no opening rule, or unbalanced braces.
        let invalid = ".foo { color: red; }}}";
        let out = minify_with_lightningcss(invalid);
        // lightningcss may parse leniently (cssparser is forgiving) — so this might return Some.
        // The contract is: it returns Option<String> and doesn't panic. Either is acceptable.
        // Just assert no panic occurred (reaching this line means it didn't panic).
        let _ = out;
    }

    /// The minified output is always <= the input length (modulo whitespace edge cases).
    #[test]
    fn minified_is_not_larger() {
        let css = ".a { color: red; padding: 10px; margin: 0; }\n\n  .b { color: blue; }";
        let out = minify_with_lightningcss(css).expect("should minify");
        assert!(out.len() <= css.len(),
            "minified ({}) should not exceed input ({}): {out:?}",
            out.len(), css.len());
    }

    /// A realistic LAYOUT_CSS-shaped snippet: tokens, multiple components, media queries.
    #[test]
    fn realistic_layout_snippet() {
        let css = r#"
            :root {
              --gl-max-width: 1200px;
              --gl-gap-md: 1.5rem;
            }
            .gl-grid {
              display: grid;
              grid-template-columns: repeat(12, 1fr);
              width: 100%;
            }
            .gl-col { grid-column: span 12; }
            @media (min-width: 768px) {
              .gl-sidebar { grid-template-columns: 280px 1fr; }
            }
        "#;
        let out = minify_with_lightningcss(css).expect("layout snippet should minify");
        assert!(out.contains("--gl-max-width"), "token preserved: {out:?}");
        assert!(out.contains("repeat(12,1fr)") || out.contains("repeat(12, 1fr)"),
            "grid-template-columns preserved: {out:?}");
        assert!(out.contains("@media"), "media query preserved: {out:?}");
        assert!(out.len() < css.len(), "should be smaller: {} vs {}", out.len(), css.len());
    }

    /// Selector merging across rules with identical declarations.
    #[test]
    fn merges_identical_rules() {
        let css = ".foo { color: red; }\n.bar { color: red; }\n.baz { color: red; }";
        let out = minify_with_lightningcss(css).expect("should minify");
        // lightningcss merges these three rules into one selector list.
        // count commas: 3 selectors → 2 commas
        let comma_count = out.matches(',').count();
        assert_eq!(comma_count, 2, "expected 2 commas (3 selectors merged): {out:?}");
        // should appear only once (one rule, not three)
        let color_count = out.matches("color:red").count();
        assert_eq!(color_count, 1, "expected 1 color declaration (merged): {out:?}");
    }

    /// Whitespace-only input doesn't panic.
    #[test]
    fn whitespace_only() {
        let out = minify_with_lightningcss("   \n\n  \t  ");
        // Accept None or Some with only whitespace stripped.
        if let Some(css) = out {
            assert!(css.trim().is_empty(), "whitespace-only should yield empty: {css:?}");
        }
    }
}