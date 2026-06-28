use leptos::prelude::*;

use crate::styles::LAYOUT_CSS;

/// Zero-sized marker placed in the reactive context so that a second mount of
/// `LayoutStyles` within the same tree is a no-op.
#[derive(Clone, Copy)]
pub struct LayoutStylesMarker;

/// Renders the crate's CSS once into the DOM as a single `<style id="gl-layout">` element.
///
/// Self-deduplicating: uses a reactive context marker so mounting it more than
/// once in the same tree is a no-op.
#[component]
pub fn LayoutStyles() -> AnyView {
    if use_context::<LayoutStylesMarker>().is_some() {
        return ().into_any();
    }
    provide_context(LayoutStylesMarker);
    view! { <style id="gl-layout">{LAYOUT_CSS}</style> }.into_any()
}