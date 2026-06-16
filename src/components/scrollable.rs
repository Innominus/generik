use crate::{
    extensions::element_extensions::ElementExtensions,
    router_utils::get_fragment_element,
};
use leptos::{html::Section, prelude::*};
use web_sys::{HtmlDivElement, HtmlElement};

#[component]
pub fn Scrollable(
    #[prop(default = Callback::new(|_| {}))] with_scroll_node: Callback<NodeRef<Section>>,
    children: Children,
    #[prop(default = 80.0)] scroll_offset: f64,
    #[prop(default = "--pad-offset")] pad_offset_var: &'static str,
    #[prop(default = "scroll-parent")] scroll_parent_id: &'static str,
) -> impl IntoView {
    let (padding_offset, set_padding_offset) = signal(0);
    let outer_ref = NodeRef::<Section>::new();
    let inner_ref = NodeRef::<leptos::html::Div>::new();

    outer_ref.on_load(move |_| {
        let outer_el: HtmlElement = outer_ref.get_untracked().unwrap();
        let inner_el: HtmlDivElement = inner_ref.get_untracked().unwrap().into();

        set_padding_offset.set(
            (outer_el.get_bounding_client_rect().width()
                - inner_el.get_bounding_client_rect().width()) as i32,
        );

        let fragment_element = get_fragment_element();
        if let Some(element) = fragment_element {
            let scroll_offset = scroll_offset;
            request_animation_frame(move || {
                element.scroll_element_to_with_offset(outer_el.into(), scroll_offset);
            });
        }

        with_scroll_node.run(outer_ref);
    });

    view! {
        <section
            data-scrollable
            id=scroll_parent_id
            node_ref=outer_ref
            style=move || { format!("{}: {}px;", pad_offset_var, padding_offset.get()) }
            class="flex overflow-y-scroll flex-col items-center w-full h-full min-h-full max-h-full bg-white scroll-bar scroll-smooth scroll-pt-[var(--header-height)]"
        >

            <div
                node_ref=inner_ref
                class="flex flex-col items-center w-full h-[calc(100%-var(--header-height))] min-h-[calc(100%-var(--header-height))] mt-[var(--header-height)]"
            >
                {children()}
            </div>
        </section>
    }
}
