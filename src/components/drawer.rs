use crate::extensions::str_toggle::StrToggler;
use leptos::prelude::*;

/// This slot can be used for content and side drawer content
#[slot]
pub struct DrawerSlot {
    children: Children,
}

/// DaisyUI Drawer Component
/// Requires DaisyUI to use
#[component]
pub fn Drawer(
    #[prop(into)] open: RwSignal<bool>,
    #[prop(default = "main-drawer")] id: &'static str,
    drawer_content: DrawerSlot,
    drawer_side: DrawerSlot,
) -> impl IntoView {
    let drawer_toggle = open;

    let id_string = id.to_string();
    let id_for_label = id.to_string();

    view! {
        <div class="h-full drawer">
            <label for=id_for_label class="sr-only">
                "Toggle navigation menu"
            </label>
            <input
                id=id_string
                prop:checked=drawer_toggle
                type="checkbox"
                class="drawer-toggle"
                aria-label="Toggle navigation menu"
            />
            <div class="min-h-0 drawer-content">{(drawer_content.children)()}</div>
            <div class="drawer-side z-60">
                <button
                    on:click=move |_| {
                        open.update(|state| *state = !*state);
                    }
                    aria-label="Close menu"
                    class="drawer-overlay"
                ></button>
                <ul class="flex relative flex-col p-4 w-full min-h-full xs:w-72 bg-base-200">
                    {(drawer_side.children)()}
                    <div class="absolute right-2 top-1/2 -translate-y-1/2">

                        <button
                            on:click=move |_| {
                                open.update(|state| *state = !*state);
                            }
                            class="btn btn-circle btn-ghost btn-sm"
                        >
                            <div class="border-gray-500 arrow arrow-right"></div>
                        </button>

                    </div>
                </ul>
            </div>
        </div>
    }
}

#[component]
pub fn DrawerSideItem(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    view! { <li class="py-2 first:pt-0 last-of-type:pb-0".push_class(class)>{children()}</li> }
}
