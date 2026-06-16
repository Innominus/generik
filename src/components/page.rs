use crate::extensions::str_toggle::StrToggler;
use leptos::prelude::*;

pub const BASE_X_PADDING: &str = "px-2 md:px-14";
pub const BASE_X_PADDING_WITH_OFFSET: &str = "pl-2 md:pl-14 pr-[calc(theme(spacing.2)-var(--pad-offset))] md:pr-[calc(theme(spacing.14)-var(--pad-offset))]";
pub const NEGATIVE_X_MARGIN: &str = "-ml-2 md:-ml-14 -mr-[calc(theme(spacing.2)-var(--pad-offset))] md:-mr-[calc(theme(spacing.14)-var(--pad-offset))]";
pub const BASE_Y_PADDING: &str = "py-4 lg:py-6";
pub const NEGATIVE_Y_BOTTOM_MARGIN: &str = "-mb-4 lg:-mb-6";
pub const BASE_PADDING: &str = "px-2 md:px-14 py-4 lg:py-6";

#[component]
pub fn PageShell(children: Children, #[prop(optional)] class: String) -> impl IntoView {
    view! {
        <section class="w-full mb-20 last:mb-0"
            .push_class(BASE_Y_PADDING)
            .push_class(BASE_X_PADDING_WITH_OFFSET)
            .push_class("flex flex-col")
            .toggle_class(class.clone(), &move || !class.is_empty())>{children()}</section>
    }
}

#[component]
pub fn PageSection(
    children: Children,
    #[prop(default = "")] class: &'static str,
    #[prop(default = true)] full_page: bool,
) -> impl IntoView {
    view! {
        <PageShell class=class
            .toggle_class("min-h-full flex-none", &|| full_page)>{children()}</PageShell>
    }
}
