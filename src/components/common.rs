use leptos::prelude::*;

#[component]
pub fn LoadingSpinner(#[prop(default = "loading-lg")] class: &'static str) -> impl IntoView {
    view! { <span class="fade-in-transition loading loading-spinner ".to_string() + class></span> }
}

#[component]
pub fn Spacer(class: &'static str) -> impl IntoView {
    view! { <div class=class></div> }
}
