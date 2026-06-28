use leptos::prelude::*;

use crate::shared::{merge_class, Gutter, Side};

/// Slot for the narrow side content of a `Sidebar`. Pass via `<SidebarSide slot>...</SidebarSide>`.
#[slot]
pub struct SidebarSide {
    children: Children,
}

/// Asymmetric two-column layout: a wide main column plus a narrow side column.
///
/// On viewports below 768px the layout collapses to a single column (mobile-first).
/// `side` controls which side the narrow column sits on (`Left` default or `Right`).
/// `side_width` overrides the narrow column width (empty uses `--gl-sidebar-width`).
///
/// Source order swaps to match the intended visual order: when `side == Left`
/// the side is rendered first, when `side == Right` the main is rendered first.
///
/// Main content goes in children; narrow content goes in the `SidebarSide` slot.
#[component]
pub fn Sidebar(
    children: Children,
    side_slot: SidebarSide,
    #[prop(optional)] side: Side,
    #[prop(default = "")] side_width: &'static str,
    #[prop(optional)] gap: Gutter,
    #[prop(default = "")] class: &'static str,
    #[prop(default = "")] style: &'static str,
) -> AnyView {
    let mut base = "gl-sidebar".to_string();
    if side == Side::Right {
        base = merge_class(&base, "gl-sidebar-right");
    }
    base = merge_class(&base, &format!("gl-gap{}", gap.class_suffix()));
    let class_str = merge_class(&base, class);

    let style_str = if side_width.is_empty() {
        style.to_string()
    } else if style.is_empty() {
        format!("--gl-sidebar-width: {side_width};")
    } else {
        format!("--gl-sidebar-width: {side_width}; {style}")
    };

    if side == Side::Right {
        view! {
            <div class=class_str style=style_str>
                <div class="gl-sidebar-main">{children()}</div>
                <div class="gl-sidebar-side">{(side_slot.children)()}</div>
            </div>
        }
        .into_any()
    } else {
        view! {
            <div class=class_str style=style_str>
                <div class="gl-sidebar-side">{(side_slot.children)()}</div>
                <div class="gl-sidebar-main">{children()}</div>
            </div>
        }
        .into_any()
    }
}