# Scroll Storyteller Utility

A comprehensive scroll-based animation and storytelling utility for Rust WASM Leptos applications. This utility provides normalized scroll progress values (0.0 - 1.0) instead of raw pixel values, making it easier to create responsive scroll-triggered animations and effects.

**Focus on Element Scrolling**: While this utility supports both window and element scrolling, the examples below focus on element scrolling as it provides better component encapsulation and more precise control for modern web applications.

## Features

- **Normalized Progress**: Always returns values between 0.0 and 1.0 regardless of content height
- **Multiple Easing Functions**: Built-in easing functions for smooth animations
- **Range-based Callbacks**: Trigger animations within specific scroll ranges
- **Element & Window Scrolling**: Works with any scrollable element or the window
- **Throttled Events**: Configurable throttling for performance
- **Parallax Effects**: Easy parallax value generation
- **Smooth Scrolling**: Programmatic scrolling with smooth animations
- **Reactive Signals**: Integrates seamlessly with Leptos reactivity

## Basic Usage

### Element Scrolling (Recommended)

```rust
use crate::utils::scroll_storyteller::{
    create_element_storyteller_with_config, ScrollProgress, ScrollStorytellerConfig
};
use leptos::prelude::*;
use web_sys::Element;

// Get reference to your scrollable container
let container_ref = NodeRef::<leptos::html::Div>::new();

Effect::new(move |_| {
    if let Some(container) = container_ref.get_untracked() {
        let config = ScrollStorytellerConfig::default();
        
        if let Ok(storyteller) = create_element_storyteller_with_config(
            container.clone().unchecked_into::<Element>(),
            config,
        ) {
            // Listen to scroll events with normalized progress
            storyteller.on_scroll(|progress: ScrollProgress| {
                logging::log!("Scroll progress: {}", progress.progress); // 0.0 to 1.0
            });
        }
    }
});
```

### Window Scrolling (Alternative)

```rust
use crate::utils::scroll_storyteller::{create_window_storyteller, ScrollProgress};

Effect::new(move |_| {
    if let Ok(storyteller) = create_window_storyteller() {
        storyteller.on_scroll(|progress: ScrollProgress| {
            logging::log!("Window scroll progress: {}", progress.progress);
        });
    }
});
```

## ScrollProgress Structure

The `ScrollProgress` struct provides comprehensive scroll information:

```rust
pub struct ScrollProgress {
    pub progress: f64,        // Normalized 0.0 - 1.0
    pub scroll_y: f64,        // Raw scroll position in pixels
    pub scroll_height: f64,   // Total scrollable height
    pub viewport_height: f64, // Visible area height
}
```

## Animation Examples

### Parallax Effect

```rust
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

#[component]
pub fn ParallaxComponent() -> impl IntoView {
    let element_ref = NodeRef::<leptos::html::Div>::new();
    
    Effect::new(move |_| {
        if let Some(element) = element_ref.get_untracked() {
            if let Ok(storyteller) = create_window_storyteller() {
                storyteller.on_scroll(move |progress| {
                    let offset = progress.progress * -100.0; // Move slower than scroll
                    if let Ok(element_html) = element.clone().dyn_into::<HtmlElement>() {
                        let _ = element_html.style().set_property(
                            "transform", 
                            &format!("translateY({}px)", offset)
                        );
                    }
                });
            }
        }
    });
    
    view! {
        <div node_ref=element_ref class="parallax-element">
            "I move with parallax!"
        </div>
    }
}
```

### Fade In Animation (Element Scrolling)

```rust
#[component] 
pub fn FadeInComponent() -> impl IntoView {
    let container_ref = NodeRef::<leptos::html::Div>::new();
    let opacity = RwSignal::new(0.0);
    
    Effect::new(move |_| {
        if let Some(container) = container_ref.get_untracked() {
            if let Ok(storyteller) = create_element_storyteller_with_config(
                container.clone().unchecked_into::<Element>(),
                ScrollStorytellerConfig::default(),
            ) {
                // Fade in between 20% and 60% of container scroll
                storyteller.on_progress_range(0.2, 0.6, move |_progress, range_progress| {
                    opacity.set(range_progress); // 0.0 to 1.0 within the range
                });
            }
        }
    });
    
    view! {
        <div node_ref=container_ref class="overflow-y-auto h-64">
            <div class="h-[400px] flex items-center justify-center">
                <div style=move || format!("opacity: {}", opacity.get())>
                    "I fade in as you scroll!"
                </div>
            </div>
        </div>
    }
}
```

### Scale Animation with Easing

```rust
let scale = RwSignal::new(0.5);

if let Ok(storyteller) = create_window_storyteller() {
    storyteller.on_scroll(move |progress| {
        let eased = progress.eased(EasingFunction::EaseInOutCubic);
        scale.set(0.5 + (eased * 1.5)); // Scale from 0.5 to 2.0
    });
}
```

## Range-Based Callbacks

### Trigger on Enter/Exit Range

```rust
// Trigger when entering a range
storyteller.on_enter_range(0.3, 0.7, |progress| {
    logging::log!("Entered the range at {}", progress.progress);
});

// Trigger when exiting a range
storyteller.on_exit_range(0.3, 0.7, |progress| {
    logging::log!("Exited the range at {}", progress.progress);
});

// Continuous updates within range
storyteller.on_progress_range(0.3, 0.7, |progress, range_progress| {
    logging::log!("Within range: {} (normalized: {})", 
                   progress.progress, range_progress);
});
```

## Helper Functions

### Parallax Values

```rust
if let Ok(storyteller) = create_window_storyteller() {
    let parallax_fn = storyteller.parallax_value(50.0); // 50px range

    // Use in your component
    view! {
        <div style=move || format!("transform: translateY({}px)", parallax_fn())>
            "Parallax content"
        </div>
    }
}
```

### Range Values

```rust
if let Ok(storyteller) = create_window_storyteller() {
    // Value that changes from 10.0 to 100.0 between 25% and 75% scroll
    let range_fn = storyteller.range_value(0.25, 0.75, 10.0, 100.0);

    view! {
        <div style=move || format!("font-size: {}px", range_fn())>
            "Dynamic font size"
        </div>
    }
}
```

### Eased Values

```rust
let eased_fn = storyteller.eased_value(EasingFunction::EaseInOut, 360.0);

view! {
    <div style=move || format!("transform: rotate({}deg)", eased_fn())>
        "Rotating element"
    </div>
}
```

## Programmatic Scrolling

### Scroll to Progress

```rust
// Scroll to 50% of the page
storyteller.scroll_to_progress(0.5).unwrap();

// Scroll to specific pixel position
storyteller.scroll_to_pixels(1000.0).unwrap();
```

## Configuration Options

```rust
pub struct ScrollStorytellerConfig {
    pub throttle_ms: u32,     // Event throttling (default: 16ms ≈ 60fps)
    pub smooth_scroll: bool,  // Enable smooth programmatic scrolling
    pub offset_top: f64,      // Top offset for fixed headers
    pub offset_bottom: f64,   // Bottom offset
}
```

## Easing Functions

Available easing functions:

- `EasingFunction::Linear` - No easing
- `EasingFunction::EaseIn` - Quadratic ease in
- `EasingFunction::EaseOut` - Quadratic ease out  
- `EasingFunction::EaseInOut` - Quadratic ease in/out
- `EasingFunction::EaseInCubic` - Cubic ease in
- `EasingFunction::EaseOutCubic` - Cubic ease out
- `EasingFunction::EaseInOutCubic` - Cubic ease in/out

## Performance Tips

1. **Use appropriate throttling**: Default 16ms provides ~60fps. Increase for better performance if not needed.

2. **Limit expensive operations**: Avoid heavy computations in scroll callbacks.

3. **Use CSS transforms**: For animations, prefer CSS transforms over changing layout properties.

4. **Debounce DOM updates**: Consider batching DOM updates when possible.

## Real-World Examples

### Scroll Progress Indicator (Element)

```rust
#[component]
pub fn ScrollProgressBar() -> impl IntoView {
    let container_ref = NodeRef::<leptos::html::Div>::new();
    let progress = RwSignal::new(0.0);
    
    Effect::new(move |_| {
        if let Some(container) = container_ref.get_untracked() {
            if let Ok(storyteller) = create_element_storyteller_with_config(
                container.clone().unchecked_into::<Element>(),
                ScrollStorytellerConfig::default(),
            ) {
                storyteller.on_scroll(move |scroll_progress| {
                    progress.set(scroll_progress.progress * 100.0);
                });
            }
        }
    });
    
    view! {
        <div>
            <div class="w-full h-2 bg-gray-200 rounded mb-4">
                <div 
                    class="h-full bg-blue-500 transition-all duration-100 rounded"
                    style=move || format!("width: {}%", progress.get())
                />
            </div>
            <div node_ref=container_ref class="overflow-y-auto h-64 border">
                <div class="h-[600px] p-4">
                    "Scrollable content here..."
                </div>
            </div>
        </div>
    }
}
```

### Section Navigation (Element)

```rust
#[component]
pub fn SectionNav() -> impl IntoView {
    let container_ref = NodeRef::<leptos::html::Div>::new();
    let current_section = RwSignal::new(1);
    
    Effect::new(move |_| {
        if let Some(container) = container_ref.get_untracked() {
            if let Ok(storyteller) = create_element_storyteller_with_config(
                container.clone().unchecked_into::<Element>(),
                ScrollStorytellerConfig::default(),
            ) {
                storyteller.on_progress_range(0.0, 0.33, move |_, _| current_section.set(1));
                storyteller.on_progress_range(0.33, 0.66, move |_, _| current_section.set(2)); 
                storyteller.on_progress_range(0.66, 1.0, move |_, _| current_section.set(3));
            }
        }
    });
    
    view! {
        <div>
            <div class="mb-4 text-center">
                <span>"Current Section: " {move || current_section.get()}</span>
            </div>
            <div node_ref=container_ref class="overflow-y-auto h-80 border">
                <div class="h-[900px]">
                    <div class="h-1/3 bg-red-100 flex items-center justify-center">
                        "Section 1"
                    </div>
                    <div class="h-1/3 bg-blue-100 flex items-center justify-center">
                        "Section 2"
                    </div>
                    <div class="h-1/3 bg-green-100 flex items-center justify-center">
                        "Section 3"
                    </div>
                </div>
            </div>
        </div>
    }
}
```

## Integration with Leptos

This utility is designed to work seamlessly with Leptos 0.8:

- Uses `RwSignal::new()` for reactive state
- Integrates with `Effect::new()` for setup
- Works with `NodeRef::<leptos::html::ElementType>::new()` for element references
- Use `get_untracked()` on NodeRefs to access elements without tracking
- Compatible with Leptos component lifecycle
- **Recommended**: Use element scrolling for better component encapsulation

### Element Scrolling Benefits:

- **Component Isolation**: Each scroll area is independent
- **Better Performance**: Only tracks relevant scroll areas  
- **Responsive Design**: Easier to handle different screen sizes
- **Reusability**: Components can be used anywhere without conflicts
- **Precise Control**: Better control over scroll boundaries and behavior

## Error Handling

Most functions return `Result<T, JsValue>` for proper error handling:

```rust
// Recommended pattern for element scrolling with error handling
Effect::new(move |_| {
    if let Some(container) = container_ref.get_untracked() {
        if let Ok(storyteller) = create_element_storyteller_with_config(
            container.clone().unchecked_into::<Element>(),
            ScrollStorytellerConfig::default(),
        ) {
            storyteller.on_scroll(move |progress| {
                // Your scroll logic here
            });
        } else {
            logging::error!("Failed to create element storyteller");
        }
    }
});
```

## Quick Start Template

Here's a complete example to get you started with element scrolling:

```rust
use crate::utils::scroll_storyteller::{
    create_element_storyteller_with_config, ScrollStorytellerConfig
};
use leptos::prelude::*;
use web_sys::Element;

#[component]
pub fn MyScrollComponent() -> impl IntoView {
    let container_ref = NodeRef::<leptos::html::Div>::new();
    let opacity = RwSignal::new(0.0);
    
    Effect::new(move |_| {
        if let Some(container) = container_ref.get_untracked() {
            if let Ok(storyteller) = create_element_storyteller_with_config(
                container.clone().unchecked_into::<Element>(),
                ScrollStorytellerConfig::default(),
            ) {
                storyteller.on_progress_range(0.2, 0.8, move |_, range_progress| {
                    opacity.set(range_progress);
                });
            }
        }
    });
    
    view! {
        <div node_ref=container_ref class="overflow-y-auto h-64 border">
            <div class="h-[400px] flex items-center justify-center">
                <div style=move || format!("opacity: {}", opacity.get())>
                    "I fade in as you scroll!"
                </div>
            </div>
        </div>
    }
}
```

This utility provides everything you need for creating engaging scroll-driven experiences in your Leptos applications with excellent component encapsulation!