use crate::scroll_storyteller::{create_element_storyteller_with_config, ScrollStorytellerConfig};
use leptos::prelude::*;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::HtmlElement;

/// Simple test component to verify scroll functionality is working
#[component]
pub fn ScrollTest() -> impl IntoView {
    let container_ref = NodeRef::<leptos::html::Div>::new();
    let scroll_value = RwSignal::new(0.0);
    let debug_info = RwSignal::new("Not initialized".to_string());

    Effect::new(move |_| {
        web_sys::console::log_1(&"Effect running, checking for container...".into());

        if let Some(container) = container_ref.get_untracked() {
            web_sys::console::log_1(&"Container found, creating storyteller...".into());

            let config = ScrollStorytellerConfig {
                throttle_ms: 50, // Slower throttle for debugging
                smooth_scroll: true,
                offset_top: 0.0,
                offset_bottom: 0.0,
                run_straight_away: false,
                resize_debounce_ms: 250,
            };

            match create_element_storyteller_with_config(
                container.clone().unchecked_into::<HtmlElement>(),
                config,
            ) {
                Ok(storyteller) => {
                    web_sys::console::log_1(&"Storyteller created successfully!".into());
                    debug_info.set("Storyteller created - waiting for scroll".to_string());

                    // Simple scroll callback
                    storyteller.on_scroll(move |progress| {
                        web_sys::console::log_1(
                            &format!("SCROLL EVENT! Progress: {}", progress.progress).into(),
                        );
                        scroll_value.set(progress.progress);
                        debug_info.set(format!(
                            "Progress: {:.3} | ScrollY: {:.1} | Height: {:.1}",
                            progress.progress, progress.scroll_y, progress.scroll_height
                        ));
                    });
                }
                Err(e) => {
                    web_sys::console::error_1(
                        &format!("Failed to create storyteller: {:?}", e).into(),
                    );
                    debug_info.set(format!("Error: {:?}", e));
                }
            }
        } else {
            web_sys::console::log_1(&"Container not found yet...".into());
        }
    });

    view! {
        <div class="p-4 mx-auto max-w-lg">
            <h2 class="mb-4 text-xl font-bold">"Scroll Test"</h2>

            <div class="p-3 mb-4 bg-gray-100 rounded">
                <p class="font-mono text-sm">{move || debug_info.get()}</p>
                <div class="mt-2">
                    <div class="text-lg font-bold">
                        "Progress: " {move || format!("{:.3}", scroll_value.get())}
                    </div>
                    <div class="mt-1 w-full h-3 bg-gray-300 rounded">
                        <div
                            class="h-full bg-blue-500 rounded transition-all duration-100"
                            style=move || format!("width: {}%", scroll_value.get() * 100.0)
                        />
                    </div>
                </div>
            </div>

            <div
                node_ref=container_ref
                class="overflow-y-auto h-64 bg-gray-50 rounded border-2 border-blue-300"
                style="scrollbar-width: thin;"
            >
                <div class="p-4 bg-gradient-to-b from-red-100 to-blue-100 h-[800px]">
                    <div class="sticky top-0 p-2 mb-4 bg-white rounded border">
                        <p class="text-sm">"SCROLL ME! Height: 800px in 256px container"</p>
                    </div>

                    {(0..20)
                        .map(|i| {
                            view! {
                                <div class="p-3 mb-4 bg-white rounded border">
                                    <h3 class="font-bold">"Item " {i + 1}</h3>
                                    <p class="text-sm text-gray-600">
                                        "This is scrollable content item " {i + 1}
                                        ". Keep scrolling to see the progress change above."
                                    </p>
                                    {if i == 10 {
                                        view! {
                                            <div class="p-2 mt-2 text-sm bg-yellow-100 rounded">
                                                "📍 Middle marker - you should see ~0.5 progress"
                                            </div>
                                        }
                                            .into_any()
                                    } else {
                                        view! {}.into_any()
                                    }}
                                </div>
                            }
                        })
                        .collect::<Vec<_>>()}

                    <div class="p-4 mt-8 bg-green-100 rounded">
                        <p class="font-bold">"🎉 Bottom reached!"</p>
                        <p class="text-sm">"Progress should be 1.0 when this is visible"</p>
                    </div>
                </div>
            </div>

            <div class="mt-4 text-xs text-gray-500">
                <p>"• Check browser console for debug logs"</p>
                <p>"• Progress should go from 0.0 to 1.0 as you scroll"</p>
                <p>"• If nothing changes, there's a bug in the scroll listener"</p>
            </div>
        </div>
    }
}

/// Even simpler test with manual scroll listener
#[component]
pub fn ManualScrollTest() -> impl IntoView {
    let container_ref = NodeRef::<leptos::html::Div>::new();
    let scroll_top = RwSignal::new(0);
    let scroll_height = RwSignal::new(0);
    let client_height = RwSignal::new(0);

    Effect::new(move |_| {
        if let Some(container) = container_ref.get_untracked() {
            web_sys::console::log_1(&"Setting up manual scroll listener...".into());

            let container_clone = container.clone();
            let scroll_callback = Closure::wrap(Box::new(move |_: web_sys::Event| {
                let scroll_top_val = container_clone.scroll_top();
                let scroll_height_val = container_clone.scroll_height();
                let client_height_val = container_clone.client_height();

                web_sys::console::log_1(
                    &format!(
                        "Manual scroll: top={}, height={}, client={}",
                        scroll_top_val, scroll_height_val, client_height_val
                    )
                    .into(),
                );

                scroll_top.set(scroll_top_val);
                scroll_height.set(scroll_height_val);
                client_height.set(client_height_val);
            }) as Box<dyn FnMut(web_sys::Event)>);

            let _ = container.add_event_listener_with_callback(
                "scroll",
                scroll_callback.as_ref().unchecked_ref(),
            );

            scroll_callback.forget();
        }
    });

    let progress = move || {
        let st = scroll_top.get() as f64;
        let sh = scroll_height.get() as f64;
        let ch = client_height.get() as f64;
        let max_scroll = (sh - ch).max(0.0);
        if max_scroll > 0.0 {
            (st / max_scroll).clamp(0.0, 1.0)
        } else {
            0.0
        }
    };

    view! {
        <div class="p-4 mx-auto mt-8 max-w-lg">
            <h2 class="mb-4 text-xl font-bold">"Manual Scroll Test"</h2>

            <div class="p-3 mb-4 bg-yellow-100 rounded">
                <div class="space-y-1 text-sm">
                    <div>"ScrollTop: " {move || scroll_top.get()}</div>
                    <div>"ScrollHeight: " {move || scroll_height.get()}</div>
                    <div>"ClientHeight: " {move || client_height.get()}</div>
                    <div class="font-bold">"Progress: " {move || format!("{:.3}", progress())}</div>
                </div>
            </div>

            <div
                node_ref=container_ref
                class="overflow-y-auto h-40 bg-yellow-50 rounded border-2 border-yellow-400"
            >
                <div class="p-2 h-[400px]">
                    <p class="mb-2 font-bold">"Manual scroll test - simpler approach"</p>
                    {(0..15)
                        .map(|i| {
                            view! {
                                <div class="p-2 mb-2 text-sm bg-white rounded">
                                    "Manual test item " {i + 1}
                                </div>
                            }
                        })
                        .collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}

/// Combined test component
#[component]
pub fn ScrollTestPage() -> impl IntoView {
    view! {
        <div class="py-8 min-h-screen bg-gray-50">
            <div class="mx-auto max-w-2xl">
                <h1 class="mb-8 text-3xl font-bold text-center">
                    "Scroll Storyteller Debug Tests"
                </h1>

                <ScrollTest />
                <ManualScrollTest />

                <div class="p-4 mt-8 bg-blue-50 rounded">
                    <h3 class="mb-2 font-bold">"Debug Instructions:"</h3>
                    <ol class="space-y-1 text-sm list-decimal list-inside">
                        <li>"Open browser dev tools and check console for logs"</li>
                        <li>"Scroll in both test containers above"</li>
                        <li>"Watch for 'SCROLL EVENT!' logs in console"</li>
                        <li>"Progress bars should update as you scroll"</li>
                        <li>
                            "If manual test works but storyteller doesn't, the issue is in ScrollStoryteller"
                        </li>
                        <li>
                            "If neither works, the issue is in Leptos setup or element references"
                        </li>
                    </ol>
                </div>
            </div>
        </div>
    }
}
