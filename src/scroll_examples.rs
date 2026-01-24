use crate::scroll_storyteller::{
    create_element_storyteller_with_config, EasingFunction, ScrollStorytellerConfig,
};
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

/// Example component demonstrating parallax scrolling within an element
#[component]
pub fn ParallaxExample() -> impl IntoView {
    let container_ref = NodeRef::<leptos::html::Div>::new();
    let parallax_ref = NodeRef::<leptos::html::Div>::new();

    Effect::new(move |_| {
        if let (Some(container), Some(element)) =
            (container_ref.get_untracked(), parallax_ref.get_untracked())
        {
            let config = ScrollStorytellerConfig {
                throttle_ms: 8,
                smooth_scroll: true,
                offset_top: 0.0,
                offset_bottom: 0.0,
                run_straight_away: false,
                resize_debounce_ms: 250,
            };

            if let Ok(storyteller) = create_element_storyteller_with_config(
                container.clone().unchecked_into::<HtmlElement>(),
                config,
            ) {
                // Create parallax effect that moves element slower than container scroll
                storyteller.on_scroll(move |progress| {
                    let parallax_offset = progress.progress * -100.0; // Move up 100px over full scroll
                    if let Ok(element_html) = element.clone().dyn_into::<HtmlElement>() {
                        let _ = element_html.style().set_property(
                            "transform",
                            &format!("translateY({}px)", parallax_offset),
                        );
                    }
                });
            }
        }
    });

    view! {
        <div class="mx-auto w-full max-w-2xl">
            <div
                node_ref=container_ref
                class="overflow-y-auto relative h-96 bg-gradient-to-b to-green-200 rounded-lg border-2 border-gray-300 from-sky-200"
                style="scroll-behavior: smooth;"
            >
                <div class="p-8 h-[800px]">
                    <div
                        node_ref=parallax_ref
                        class="absolute top-1/2 left-1/2 p-6 bg-white rounded-lg shadow-lg transform -translate-x-1/2 -translate-y-1/2"
                    >
                        <h3 class="text-2xl font-bold text-center">"Parallax Element"</h3>
                        <p class="mt-2 text-center text-gray-600">
                            "I move slower than the scroll"
                        </p>
                    </div>
                    <div class="p-4 mt-96 rounded bg-white/80">
                        <p>
                            "Scroll within this container to see the parallax effect on the floating element above."
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Example component demonstrating scroll-triggered animations within an element
#[component]
pub fn ScrollAnimationExample() -> impl IntoView {
    let container_ref = NodeRef::<leptos::html::Div>::new();
    let opacity_signal = RwSignal::new(0.0);
    let scale_signal = RwSignal::new(0.5);

    Effect::new(move |_| {
        if let Some(container) = container_ref.get_untracked() {
            if let Ok(storyteller) = create_element_storyteller_with_config(
                container.clone().unchecked_into::<HtmlElement>(),
                ScrollStorytellerConfig::default(),
            ) {
                // Fade in and scale up between 10% and 50% of container scroll
                storyteller.on_progress_range(0.1, 0.5, move |_progress, range_progress| {
                    let eased_progress = if range_progress < 0.5 {
                        2.0 * range_progress * range_progress // ease in
                    } else {
                        1.0 - (-2.0 * range_progress + 2.0).powi(2) / 2.0 // ease out
                    };

                    opacity_signal.set(eased_progress);
                    scale_signal.set(0.5 + (eased_progress * 0.5));
                });
            }
        }
    });

    view! {
        <div class="mx-auto w-full max-w-2xl">
            <div
                node_ref=container_ref
                class="overflow-y-auto h-96 bg-gray-50 rounded-lg border-2 border-gray-300"
            >
                <div class="p-8 h-[1000px]">
                    <div class="flex justify-center items-center h-32">
                        <p class="text-gray-600">"Scroll down to see animation"</p>
                    </div>

                    <div class="flex justify-center items-center h-64">
                        <div
                            class="flex justify-center items-center w-32 h-32 bg-blue-500 rounded-lg"
                            style=move || {
                                format!(
                                    "opacity: {}; transform: scale({}); transition: opacity 0.15s ease-out, transform 0.15s ease-out",
                                    opacity_signal.get(),
                                    scale_signal.get(),
                                )
                            }
                        >
                            <span class="font-bold text-white">"Animate"</span>
                        </div>
                    </div>

                    <div class="p-4 mt-64 bg-white rounded">
                        <p class="text-gray-600">
                            "Animation triggers between 10% and 50% of scroll progress."
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Example component demonstrating scroll progress indicator for element scrolling
#[component]
pub fn ScrollProgressIndicator() -> impl IntoView {
    let container_ref = NodeRef::<leptos::html::Div>::new();
    let progress_signal = RwSignal::new(0.0);

    Effect::new(move |_| {
        if let Some(container) = container_ref.get_untracked() {
            if let Ok(storyteller) = create_element_storyteller_with_config(
                container.clone().unchecked_into::<HtmlElement>(),
                ScrollStorytellerConfig::default(),
            ) {
                storyteller.on_scroll(move |progress| {
                    progress_signal.set(progress.progress * 100.0);
                });
            }
        }
    });

    view! {
        <div class="mx-auto w-full max-w-2xl">
            <div class="p-3 mb-4 bg-gray-100 rounded">
                <div class="flex justify-between items-center mb-2">
                    <span class="text-sm font-medium">"Scroll Progress:"</span>
                    <span class="font-mono text-sm">
                        {move || format!("{:.1}%", progress_signal.get())}
                    </span>
                </div>
                <div class="w-full h-2 bg-gray-300 rounded-full">
                    <div
                        class="h-2 bg-blue-500 rounded-full transition-all duration-200 ease-out"
                        style=move || format!("width: {}%", progress_signal.get())
                    />
                </div>

            </div>

            <div
                node_ref=container_ref
                class="overflow-y-auto h-64 bg-white rounded-lg border-2 border-gray-300"
            >
                <div class="p-6 space-y-4 h-[800px]">
                    {(1..=20)
                        .map(|i| {
                            view! {
                                <div class="p-4 bg-gray-100 rounded">
                                    <h4 class="font-semibold">{format!("Section {}", i)}</h4>
                                    <p class="text-sm text-gray-600">
                                        "This is content section " {i}
                                        ". Watch the progress bar above as you scroll."
                                    </p>
                                </div>
                            }
                        })
                        .collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}

/// Example component demonstrating scroll-triggered section changes within an element
#[component]
pub fn SectionScrollExample() -> impl IntoView {
    let container_ref = NodeRef::<leptos::html::Div>::new();
    let current_section = RwSignal::new(1);

    Effect::new(move |_| {
        if let Some(container) = container_ref.get_untracked() {
            if let Ok(storyteller) = create_element_storyteller_with_config(
                container.clone().unchecked_into::<HtmlElement>(),
                ScrollStorytellerConfig::default(),
            ) {
                // Define section triggers with better boundaries
                storyteller.on_progress_range(0.0, 0.3, move |_progress, _range| {
                    current_section.set(1);
                });

                storyteller.on_progress_range(0.3, 0.7, move |_progress, _range| {
                    current_section.set(2);
                });

                storyteller.on_progress_range(0.7, 1.0, move |_progress, _range| {
                    current_section.set(3);
                });
            }
        }
    });

    view! {
        <div class="mx-auto w-full max-w-2xl">
            <div class="p-3 mb-4 text-center bg-blue-100 rounded">
                <span class="font-semibold text-blue-800">
                    "Current Section: " {move || current_section.get()}
                </span>
            </div>

            <div
                node_ref=container_ref
                class="overflow-y-auto h-80 rounded-lg border-2 border-gray-300"
                style="scroll-behavior: smooth;"
            >
                <div class="h-[900px]">
                    <div
                        class="flex justify-center items-center bg-red-100 border-b-2 border-white"
                        style="height: 30%;"
                    >
                        <div class="text-center">
                            <h2 class="text-3xl font-bold text-red-800">"Section 1"</h2>
                            <p class="text-red-600">"0% - 30% of scrollable area"</p>
                        </div>
                    </div>
                    <div
                        class="flex justify-center items-center bg-green-100 border-b-2 border-white"
                        style="height: 40%;"
                    >
                        <div class="text-center">
                            <h2 class="text-3xl font-bold text-green-800">"Section 2"</h2>
                            <p class="text-green-600">"30% - 70% of scrollable area"</p>
                        </div>
                    </div>
                    <div class="flex justify-center items-center bg-blue-100" style="height: 30%;">
                        <div class="text-center">
                            <h2 class="text-3xl font-bold text-blue-800">"Section 3"</h2>
                            <p class="text-blue-600">"70% - 100% of scrollable area"</p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Example component demonstrating scroll-to functionality for elements
#[component]
pub fn ScrollToExample() -> impl IntoView {
    let container_ref = NodeRef::<leptos::html::Div>::new();

    let scroll_to_section = move |progress: f64| {
        if let Some(container) = container_ref.get_untracked() {
            if let Ok(storyteller) = create_element_storyteller_with_config(
                container.clone().unchecked_into::<HtmlElement>(),
                ScrollStorytellerConfig::default(),
            ) {
                let _ = storyteller.scroll_to_progress(progress);
            }
        }
    };

    view! {
        <div class="mx-auto w-full max-w-2xl">
            <div class="p-3 mb-4 bg-gray-100 rounded">
                <h3 class="mb-2 font-semibold">"Quick Navigation"</h3>
                <div class="flex flex-wrap gap-2">
                    <button
                        class="py-1 px-3 text-white bg-blue-500 rounded transition-colors hover:bg-blue-600"
                        on:click=move |_| scroll_to_section(0.0)
                    >
                        "Top"
                    </button>
                    <button
                        class="py-1 px-3 text-white bg-blue-500 rounded transition-colors hover:bg-blue-600"
                        on:click=move |_| scroll_to_section(0.25)
                    >
                        "25%"
                    </button>
                    <button
                        class="py-1 px-3 text-white bg-blue-500 rounded transition-colors hover:bg-blue-600"
                        on:click=move |_| scroll_to_section(0.5)
                    >
                        "Middle"
                    </button>
                    <button
                        class="py-1 px-3 text-white bg-blue-500 rounded transition-colors hover:bg-blue-600"
                        on:click=move |_| scroll_to_section(0.75)
                    >
                        "75%"
                    </button>
                    <button
                        class="py-1 px-3 text-white bg-blue-500 rounded transition-colors hover:bg-blue-600"
                        on:click=move |_| scroll_to_section(1.0)
                    >
                        "Bottom"
                    </button>
                </div>
            </div>

            <div
                node_ref=container_ref
                class="overflow-y-auto h-80 bg-gradient-to-b from-purple-100 via-pink-100 to-yellow-100 rounded-lg border-2 border-gray-300"
                style="scroll-behavior: smooth;"
            >
                <div class="p-6 h-[1200px]">
                    <div class="space-y-8">
                        <div class="p-6 rounded-lg bg-white/80">
                            <h3 class="mb-2 text-xl font-bold">"Navigation Example"</h3>
                            <p class="text-gray-600">
                                "Use the buttons above to smoothly scroll to different positions within this container."
                            </p>
                        </div>

                        {(1..=10)
                            .map(|i| {
                                let bg_color = match i % 4 {
                                    0 => "bg-red-100",
                                    1 => "bg-blue-100",
                                    2 => "bg-green-100",
                                    _ => "bg-yellow-100",
                                };

                                view! {
                                    <div class=format!("p-4 {} rounded", bg_color)>
                                        <h4 class="font-semibold">
                                            {format!("Content Block {}", i)}
                                        </h4>
                                        <p class="text-sm text-gray-600">
                                            "This is content block number " {i}
                                            ". The smooth scrolling will position this content appropriately."
                                        </p>
                                    </div>
                                }
                            })
                            .collect::<Vec<_>>()}
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Example component combining multiple scroll effects within an element
#[component]
pub fn AdvancedScrollExample() -> impl IntoView {
    let container_ref = NodeRef::<leptos::html::Div>::new();
    let background_hue = RwSignal::new(200.0);
    let text_scale = RwSignal::new(1.0);
    let rotation = RwSignal::new(0.0);

    Effect::new(move |_| {
        if let Some(container) = container_ref.get_untracked() {
            if let Ok(storyteller) = create_element_storyteller_with_config(
                container.clone().unchecked_into::<HtmlElement>(),
                ScrollStorytellerConfig::default(),
            ) {
                storyteller.on_scroll(move |progress| {
                    // Change background hue through the scroll
                    background_hue.set(200.0 + (progress.progress * 160.0)); // 200-360 degrees

                    // Scale text with easing
                    let eased_progress = progress.eased(EasingFunction::EaseInOutCubic);
                    text_scale.set(1.0 + (eased_progress * 1.5));

                    // Rotate based on scroll
                    rotation.set(progress.progress * 180.0);
                });
            }
        }
    });

    view! {
        <div class="mx-auto w-full max-w-2xl">
            <div
                node_ref=container_ref
                class="overflow-y-auto h-96 rounded-lg border-2 border-gray-300"
                style=move || {
                    format!(
                        "background: linear-gradient(135deg, hsl({}, 60%, 85%), hsl({}, 60%, 95%))",
                        background_hue.get(),
                        background_hue.get() + 30.0,
                    )
                }
            >
                <div class="flex justify-center items-center h-[1000px]">
                    <div
                        class="p-8 text-center rounded-lg shadow-lg bg-white/80"
                        style=move || {
                            format!(
                                "transform: scale({}) rotate({}deg); transition: transform 0.1s ease-out",
                                text_scale.get(),
                                rotation.get(),
                            )
                        }
                    >
                        <h1 class="mb-4 text-4xl font-bold text-gray-800">"Advanced Effects"</h1>
                        <p class="text-lg text-gray-600">"Background + Scale + Rotation"</p>
                        <div class="mt-4 text-sm text-gray-500">
                            <p>"Hue: " {move || format!("{:.0}°", background_hue.get())}</p>
                            <p>"Scale: " {move || format!("{:.2}x", text_scale.get())}</p>
                            <p>"Rotation: " {move || format!("{:.0}°", rotation.get())}</p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Simple usage example showing the basic pattern for element scrolling
#[component]
pub fn BasicUsageExample() -> impl IntoView {
    let container_ref = NodeRef::<leptos::html::Div>::new();
    let scroll_progress = RwSignal::new(0.0);
    let debug_msg = RwSignal::new("Initializing...".to_string());

    // Manual scroll listener for comparison
    let manual_progress = RwSignal::new(0.0);

    Effect::new(move |_| {
        web_sys::console::log_1(&"BasicUsageExample: Effect starting...".into());

        if let Some(container) = container_ref.get_untracked() {
            web_sys::console::log_1(&"Container element found!".into());
            debug_msg.set("Container found, setting up listeners...".to_string());

            // Manual scroll listener first (to verify basic functionality)
            let container_for_manual = container.clone();
            let manual_callback =
                wasm_bindgen::closure::Closure::wrap(Box::new(move |_: web_sys::Event| {
                    let scroll_top = container_for_manual.scroll_top() as f64;
                    let scroll_height = container_for_manual.scroll_height() as f64;
                    let client_height = container_for_manual.client_height() as f64;
                    let max_scroll = (scroll_height - client_height).max(0.0);
                    let progress = if max_scroll > 0.0 {
                        scroll_top / max_scroll
                    } else {
                        0.0
                    };

                    web_sys::console::log_1(
                        &format!("Manual scroll: progress = {:.3}", progress).into(),
                    );
                    manual_progress.set(progress);
                })
                    as Box<dyn FnMut(web_sys::Event)>);

            let element: Element = container.clone().into();
            let _ = element.add_event_listener_with_callback(
                "scroll",
                manual_callback.as_ref().unchecked_ref(),
            );
            manual_callback.forget();

            // Now try the storyteller
            if let Ok(storyteller) = create_element_storyteller_with_config(
                container.clone().unchecked_into::<HtmlElement>(),
                ScrollStorytellerConfig::default(),
            ) {
                web_sys::console::log_1(&"Storyteller created successfully!".into());
                debug_msg.set("Storyteller active - scroll to test".to_string());

                storyteller.on_scroll(move |progress| {
                    web_sys::console::log_1(
                        &format!("Storyteller scroll event: {:.3}", progress.progress).into(),
                    );
                    scroll_progress.set(progress.progress);
                });
            } else {
                web_sys::console::error_1(&"Failed to create storyteller!".into());
                debug_msg.set("Error: Failed to create storyteller".to_string());
            }
        }
    });

    view! {
        <div class="mx-auto w-full max-w-2xl">
            <div class="p-4 mb-4 bg-blue-50 rounded-lg">
                <h3 class="mb-2 font-semibold">"Basic Element Scrolling Debug"</h3>
                <div class="mb-2 text-sm text-gray-600">{move || debug_msg.get()}</div>

                <div class="grid gap-4 md:grid-cols-2">
                    <div>
                        <h4 class="text-sm font-semibold">"Manual Listener:"</h4>
                        <div class="font-mono text-lg text-green-600">
                            {move || format!("{:.3}", manual_progress.get())}
                        </div>
                        <div class="w-full h-2 bg-gray-200 rounded-full">
                            <div
                                class="h-2 bg-green-500 rounded-full transition-all duration-200 ease-out"
                                style=move || format!("width: {}%", manual_progress.get() * 100.0)
                            />
                        </div>
                    </div>

                    <div>
                        <h4 class="text-sm font-semibold">"Storyteller:"</h4>
                        <div class="font-mono text-lg text-blue-600">
                            {move || format!("{:.3}", scroll_progress.get())}
                        </div>
                        <div class="w-full h-2 bg-gray-200 rounded-full">
                            <div
                                class="h-2 bg-blue-600 rounded-full transition-all duration-200 ease-out"
                                style=move || format!("width: {}%", scroll_progress.get() * 100.0)
                            />
                        </div>
                    </div>
                </div>
            </div>

            <div
                node_ref=container_ref
                class="overflow-y-auto h-64 bg-white rounded-lg border-2 border-gray-300"
                style="scroll-behavior: smooth;"
            >
                <div class="p-6 h-[600px]">
                    <div class="space-y-4">
                        <div class="p-4 bg-gray-100 rounded">
                            <h4 class="font-semibold">"Element Scroll Storytelling"</h4>
                            <p class="mt-2 text-sm text-gray-600">
                                "Unlike window scrolling, this tracks scroll progress within this specific container element."
                            </p>
                        </div>

                        <div class="p-4 bg-blue-50 rounded">
                            <h4 class="font-semibold">"Benefits"</h4>
                            <ul class="mt-2 text-sm list-disc list-inside text-gray-600">
                                <li>"More precise control over scroll areas"</li>
                                <li>"Multiple independent scroll storytellers"</li>
                                <li>"Better for component-based animations"</li>
                                <li>"Easier to contain scroll effects"</li>
                            </ul>
                        </div>

                        <div class="p-4 bg-green-50 rounded">
                            <h4 class="font-semibold">"Use Cases"</h4>
                            <ul class="mt-2 text-sm list-disc list-inside text-gray-600">
                                <li>"Card or panel animations"</li>
                                <li>"Timeline components"</li>
                                <li>"Gallery or portfolio sections"</li>
                                <li>"Dashboard widgets"</li>
                            </ul>
                        </div>

                        <div class="p-4 bg-yellow-50 rounded">
                            <p class="text-sm text-gray-600">
                                "Scroll within this container to see the progress indicator update above.
                                The value is always normalized between 0.0 (top) and 1.0 (bottom)."
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Complete demo page showing all element scrolling examples
#[component]
pub fn ScrollStorytellerDemo() -> impl IntoView {
    view! {
        <div class="py-8 min-h-screen bg-gray-50">
            <div class="px-4 mx-auto max-w-4xl">
                <div class="mb-12 text-center">
                    <h1 class="mb-4 text-4xl font-bold text-gray-900">
                        "Element Scroll Storyteller Demo"
                    </h1>
                    <p class="text-lg text-gray-600">
                        "Interactive examples showing scroll-based animations within container elements"
                    </p>
                </div>

                <div class="space-y-12">
                    <div>
                        <h2 class="mb-4 text-2xl font-bold text-gray-900">"1. Basic Usage"</h2>
                        <BasicUsageExample />
                    </div>

                    <div>
                        <h2 class="mb-4 text-2xl font-bold text-gray-900">
                            "2. Scroll Progress Tracking"
                        </h2>
                        <ScrollProgressIndicator />
                    </div>

                    <div>
                        <h2 class="mb-4 text-2xl font-bold text-gray-900">"3. Parallax Effect"</h2>
                        <ParallaxExample />
                    </div>

                    <div>
                        <h2 class="mb-4 text-2xl font-bold text-gray-900">
                            "4. Scroll Animations"
                        </h2>
                        <ScrollAnimationExample />
                    </div>

                    <div>
                        <h2 class="mb-4 text-2xl font-bold text-gray-900">"5. Section Tracking"</h2>
                        <SectionScrollExample />
                    </div>

                    <div>
                        <h2 class="mb-4 text-2xl font-bold text-gray-900">
                            "6. Programmatic Scrolling"
                        </h2>
                        <ScrollToExample />
                    </div>

                    <div>
                        <h2 class="mb-4 text-2xl font-bold text-gray-900">
                            "7. Advanced Combined Effects"
                        </h2>
                        <AdvancedScrollExample />
                    </div>
                </div>

                <div class="p-6 mt-16 bg-white rounded-lg border shadow-sm">
                    <h3 class="mb-3 text-xl font-semibold">
                        "Element Scrolling vs Window Scrolling"
                    </h3>
                    <div class="grid gap-6 md:grid-cols-2">
                        <div>
                            <h4 class="mb-2 font-semibold text-green-700">
                                "✅ Element Scrolling Benefits:"
                            </h4>
                            <ul class="space-y-1 text-sm list-disc list-inside text-gray-600">
                                <li>"Multiple independent scroll areas"</li>
                                <li>"Better component encapsulation"</li>
                                <li>"More precise scroll control"</li>
                                <li>"Easier responsive design"</li>
                                <li>"Better for complex layouts"</li>
                            </ul>
                        </div>
                        <div>
                            <h4 class="mb-2 font-semibold text-blue-700">"💡 Perfect for:"</h4>
                            <ul class="space-y-1 text-sm list-disc list-inside text-gray-600">
                                <li>"Timeline components"</li>
                                <li>"Product galleries"</li>
                                <li>"Dashboard widgets"</li>
                                <li>"Card-based layouts"</li>
                                <li>"Modal content"</li>
                            </ul>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
