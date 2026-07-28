//! Adapter-private helpers shared by every Silkenweb layout adapter.
//!
//! [`merge_style`] and [`layout_div`] were originally inlined in the
//! box-primitive adapters (`primitives`); they now live here so the grid,
//! masonry, sidebar, and col adapters can reuse the exact same class-splitting
//! and style-merging behaviour. [`bare_div`] factors out the "build a `<div>`
//! with class tokens and an optional `style` attribute, no children yet" step
//! so [`layout_div`] and the non-default `col` path (which must prepend a
//! generated `<style>` before user children) share one implementation.

use silkenweb::dom::Dom;
use silkenweb::elements::html::{div, Div};
use silkenweb::node::element::{Element, ParentElement};
use silkenweb::node::ChildNode;

/// Merge an own inline-style fragment with a passthrough `style` string.
///
/// Mirrors the Leptos components exactly: `own` first, then `passthrough`,
/// joined by a single space. An empty side yields the other side verbatim, so
/// an empty result means "emit no `style` attribute at all".
pub(super) fn merge_style(own: &str, passthrough: &str) -> String {
    if passthrough.is_empty() {
        own.to_string()
    } else if own.is_empty() {
        passthrough.to_string()
    } else {
        format!("{own} {passthrough}")
    }
}

/// Apply `class_str` (whitespace-split into tokens) and, when non-empty, a
/// single `style` attribute to any element.
///
/// This is the same safe class-splitting and style-omitting behaviour as
/// [`bare_div`], generalised to semantic elements such as
/// [`Figure`](silkenweb::elements::html::Figure) and
/// [`Figcaption`](silkenweb::elements::html::Figcaption) that [`bare_div`] —
/// which is hardcoded to `<div>` — cannot build. Splitting on whitespace and
/// routing through [`Element::classes`] (never [`Element::class`]) keeps
/// multi-class strings safe; an empty `style_str` emits no `style` attribute.
pub(super) fn styled<E, D>(el: E, class_str: &str, style_str: &str) -> E
where
    E: Element<Dom = D>,
    D: Dom,
{
    let classes = class_str.split_whitespace().collect::<Vec<_>>();
    let el = el.classes(classes.as_slice());
    if style_str.is_empty() {
        el
    } else {
        el.attribute("style", style_str)
    }
}

/// Build a `<div>` carrying `class_str` (split into individual tokens) and, when
/// non-empty, a single `style` attribute — but no children yet.
///
/// Splitting on whitespace and routing through [`Element::classes`] — never
/// [`Element::class`] — keeps multi-class strings safe: `.class()` panics on
/// any whitespace, so a merged `gl-stack gl-gap-md extra` string must be split
/// first. The split is collected to a `&[&str]` slice because
/// [`Element::classes`] takes a Silkenweb value (a `RefSignalOrValue`, e.g.
/// an array or slice), not a bare iterator.
pub(super) fn bare_div<D>(class_str: &str, style_str: &str) -> Div<D>
where
    D: Dom,
{
    styled(div::<D>(), class_str, style_str)
}

/// [`bare_div`] with `children` appended.
///
/// This is the common case for adapters whose first child is user content;
/// `col`'s non-default path calls [`bare_div`] directly so it can prepend a
/// generated `<style>` element first.
pub(super) fn layout_div<D>(
    class_str: &str,
    style_str: &str,
    children: impl IntoIterator<Item = impl ChildNode<D>>,
) -> Div<D>
where
    D: Dom,
{
    bare_div::<D>(class_str, style_str).children(children)
}
