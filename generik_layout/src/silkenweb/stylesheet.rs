//! Once-only mounting of the crate's layout stylesheet into the document
//! `<head>`.
//!
//! [`mount_styles`] is the Silkenweb counterpart of the Leptos `LayoutStyles`
//! component: it injects the single [`LAYOUT_CSS`](crate::LAYOUT_CSS) string as
//! one `<style>` element. Unlike the Leptos component (which deduplicates via a
//! reactive-context marker), the Silkenweb adapter uses Silkenweb's documented
//! [`DefaultDom`] document-head API guarded by a process-wide [`Once`], so a
//! second call is a true no-op.

use std::sync::Once;

use silkenweb::document::{Document, DocumentHead};
use silkenweb::dom::DefaultDom;
use silkenweb::elements::html::style;
use silkenweb::node::element::TextParentElement;

use crate::styles::LAYOUT_CSS;

/// Stable head-mount identifier used by the once guard. Silkenweb records this
/// as a `data-silkenweb-head-id` attribute for hydration; it is not a DOM `id`.
const STYLE_MOUNT_ID: &str = "gl-layout-styles";

/// Mount the crate's [`LAYOUT_CSS`] into the document `<head>` exactly once.
///
/// This is the Silkenweb equivalent of the Leptos `LayoutStyles` component.
/// Call it once during app setup — before or alongside the layout primitives —
/// so the `gl-*` classes resolve. It uses Silkenweb's documented
/// [`DefaultDom::mount_in_head`] document-head API behind a process-wide
/// [`Once`], so repeated calls are a no-op and never mount a duplicate
/// `<style>`.
///
/// On the server (`Dry`), this must be called from within a
/// [`silkenweb::task::scope`] so the document-head task local is available; on
/// the client it works in any context.
///
/// [`LAYOUT_CSS`]: crate::LAYOUT_CSS
/// [`DefaultDom::mount_in_head`]: silkenweb::document::Document::mount_in_head
pub fn mount_styles() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        let head = DocumentHead::new().child(style().text(LAYOUT_CSS));
        DefaultDom::mount_in_head(STYLE_MOUNT_ID, head);
    });
}

#[cfg(test)]
mod tests {
    use crate::silkenweb::mount_styles;

    /// The once-only behaviour is exercised on the host (`Dry`) DOM, where
    /// `mount_in_head` / `head_inner_html` require the document-head task
    /// local provided by [`silkenweb::task::scope`]. On `wasm32` the head is
    /// backed by the real DOM and is covered by browser integration tests, not
    /// the host suite.
    #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
    #[test]
    fn mount_styles_mounts_once_then_is_a_noop() {
        use silkenweb::document::Document;
        use silkenweb::dom::Dry;
        use silkenweb::task;

        task::server::block_on(task::scope(async {
            // First call mounts the stylesheet into this scope's <head>.
            mount_styles();
            task::render_now().await;
            let first = Dry::head_inner_html();

            assert_eq!(
                first.matches("<style").count(),
                1,
                "first mount inserts exactly one <style>: {first}"
            );
            assert!(
                first.contains("gl-"),
                "head should carry the layout CSS: {first}"
            );

            // The Once guard makes every subsequent call a no-op: the head is
            // unchanged and no duplicate <style> is inserted.
            mount_styles();
            task::render_now().await;
            let second = Dry::head_inner_html();
            assert_eq!(second, first, "second mount_styles() must not duplicate");
        }));
    }
}