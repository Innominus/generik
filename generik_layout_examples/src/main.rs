use leptos::prelude::*;

use generik_layout_examples::app::App;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> });
}