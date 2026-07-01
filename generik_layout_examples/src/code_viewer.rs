//! Source-of-truth code viewer.
//!
//! Embeds `examples.rs` and `showcase.rs` at compile time via `include_str!`,
//! slices them by `// BEGIN <Name>` / `// END <Name>` markers at runtime, and
//! renders the extracted source in a `<pre><code class="language-rust">` block
//! that is syntax-highlighted by highlight.js (loaded from CDN in index.html).

use std::sync::atomic::{AtomicUsize, Ordering};

use leptos::prelude::*;

/// The full source of `examples.rs`, embedded at compile time.
const EXAMPLES_SRC: &str = include_str!("examples.rs");
/// The full source of `showcase.rs`, embedded at compile time.
const SHOWCASE_SRC: &str = include_str!("showcase.rs");

/// Extract a named region from `src` between `// BEGIN <name>` and `// END <name>`
/// markers. Returns the trimmed code between the markers (markers excluded).
/// Returns an error message string if the markers aren't found.
fn extract_region(src: &str, name: &str) -> String {
    // Markers are always on their own line; newline-bound to avoid false matches
    // inside string literals or comments.
    let begin = format!("\n// BEGIN {name}\n");
    let end = format!("\n// END {name}\n");
    let start = match src.find(&begin) {
        Some(i) => i + begin.len(),
        None => return format!("// markers not found for {name}"),
    };
    let end_idx = match src[start..].find(&end) {
        Some(j) => start + j,
        None => return format!("// end marker not found for {name}"),
    };
    src[start..end_idx].trim().to_string()
}

/// Returns the source code for a named example.
/// `name` is the marker name, e.g. `"GridExample"` or `"Showcase"`.
pub fn example_source(name: &str) -> String {
    if name == "Showcase" {
        extract_region(SHOWCASE_SRC, name)
    } else {
        extract_region(EXAMPLES_SRC, name)
    }
}

/// Monotonic counter used to give each `<code>` element a unique id so the
/// global JS highlighter can look it up by id.
static CODE_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Renders a syntax-highlighted code block. On mount, calls the global
/// `window.__hljsHighlight(id)` JS function (defined in index.html), which
/// looks up the `<code>` element by id and calls `hljs.highlightElement(el)`.
#[component]
pub fn CodeBlock(code: String) -> impl IntoView {
    let id = CODE_COUNTER.fetch_add(1, Ordering::Relaxed);
    let el_id = format!("gl-code-{id}");
    let el_id_for_js = el_id.clone();
    let code_ref = NodeRef::<leptos::html::Code>::new();
    code_ref.on_load(move |_| {
        let _ = js_sys::eval(&format!(
            "window.__hljsHighlight && window.__hljsHighlight('{el_id_for_js}');"
        ));
    });

    view! {
        <pre class="rounded-md bg-neutral-900 text-sm overflow-x-auto p-4 max-h-[600px]">
            <code node_ref=code_ref id=el_id class="language-rust">{code}</code>
        </pre>
    }
}