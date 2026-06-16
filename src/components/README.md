# generik::components

This module provides opinionated, reusable Leptos components extracted from real-world projects. They assume a Tailwind CSS + DaisyUI stylesheet is available (some components use DaisyUI classes such as `drawer`, `btn`, `card`, etc.).

Enable the module with the `components` feature:

```toml
[dependencies]
generik = { git = "https://github.com/Innominus/generik", features = ["components"] }
```

Then import from `generik::components::*` or individual submodules:

```rust
use generik::components::{scrollable::Scrollable, toast::{ToastProvider, use_toast}};
```

---

## `Scrollable`

A full-viewport-height custom scroll container that compensates for the width of the native scrollbar and restores hash-based anchor scrolling on load.

### Why it exists

In many SPA layouts the document body is not the scrolling root. Instead, an inner element is fixed to `100vh` and scrolls its own content. `Scrollable` wraps that element, measures the scrollbar width, exposes it as a CSS custom property, and calls you back once the scroll node is ready so you can attach scroll storytellers, observers, etc.

### Basic usage

```rust
use generik::components::scrollable::Scrollable;
use leptos::prelude::*;

#[component]
pub fn MyPage() -> impl IntoView {
    view! {
        <Scrollable>
            <div class="p-8">
                <h1>"Content goes here"</h1>
            </div>
        </Scrollable>
    }
}
```

### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `children` | `Children` | required | The scrollable content. It is wrapped in an inner `div`. |
| `with_scroll_node` | `Callback<NodeRef<Section>>` | no-op | Called once the outer `<section>` has mounted. Use it to set up scroll listeners or storytellers. |
| `scroll_offset` | `f64` | `80.0` | Vertical offset used when scrolling to a hash fragment on load. |
| `pad_offset_var` | `&'static str` | `"--pad-offset"` | Name of the CSS custom property that receives the scrollbar width in pixels. |
| `scroll_parent_id` | `&'static str` | `"scroll-parent"` | `id` applied to the outer `<section>`. Other code can scroll this element by id. |

### CSS contract

`Scrollable` does not ship its own CSS, but it uses Tailwind classes and a few CSS variables you must define in your stylesheet:

```css
:root {
  --header-height: 4rem;   /* height of your fixed header */
}

.scroll-bar {
  scrollbar-gutter: stable; /* optional, prevents layout shift */
}
```

The component applies `scroll-pt-[var(--header-height)]` so hash anchors clear the fixed header.

### Scrollbar width compensation

On mount, `Scrollable` computes:

```
scrollbar_width = outer_width - inner_width
```

and sets the CSS variable `--pad-offset` (or whatever you named it) to that value. You can then write right-padding utilities that keep content visually aligned when the scrollbar appears:

```rust
use generik::components::page::BASE_X_PADDING_WITH_OFFSET;

view! {
    <div class=BASE_X_PADDING_WITH_OFFSET>
        "This content's right edge aligns with non-scrollable pages."
    </div>
}
```

### Attaching a storyteller

Use the `with_scroll_node` callback to receive the outer `NodeRef<Section>` once it exists:

```rust
use generik::{
    components::{overlay::FooterController, scrollable::Scrollable},
    scroll_storyteller::{create_element_storyteller_with_config, ScrollStorytellerConfig},
};
use leptos::{html::Section, prelude::*};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

#[component]
pub fn Home() -> impl IntoView {
    let footer_controller = expect_context::<RwSignal<FooterController>>();

    let on_scroll = Callback::new(move |node: NodeRef<Section>| {
        let el = node
            .get_untracked()
            .unwrap()
            .unchecked_into::<HtmlElement>();

        let teller = create_element_storyteller_with_config(
            el,
            ScrollStorytellerConfig::default(),
        )
        .unwrap();

        teller.on_enter_range(0.98, 1.0, move |_| {
            footer_controller.update(|f| f.toggle(true));
        });
        teller.on_exit_range(0.98, 1.0, move |_| {
            footer_controller.update(|f| f.toggle(false));
        });
    });

    view! {
        <Scrollable with_scroll_node=on_scroll>
            <section id="hero">"Hero"</section>
            <section id="details">"Details"</section>
        </Scrollable>
    }
}
```

### Hash anchor restoration

If the user lands on a URL such as `/page#details`, `Scrollable` automatically finds the element with id `details` and scrolls it into view with `scroll_offset` pixels of clearance. This works because the component is itself the scroll root.

### Full example with fixed header + footer

```rust
use generik::components::{
    overlay::FooterController,
    page::PageSection,
    scrollable::Scrollable,
    toast::{ToastProvider, use_toast},
};
use leptos::prelude::*;

#[component]
pub fn AppShell() -> impl IntoView {
    provide_context(RwSignal::new(FooterController::new()));

    view! {
        <ToastProvider>
            <MyHeader />
            <Scrollable scroll_offset=80.0 scroll_parent_id="main-scroll">
                <PageSection>
                    "First section"
                </PageSection>
                <PageSection>
                    "Second section"
                </PageSection>
            </Scrollable>
            <MyFixedFooter />
        </ToastProvider>
    }
}
```

---

## `Drawer`

A DaisyUI drawer wrapper. Requires DaisyUI in your stylesheet.

### Usage

```rust
use generik::components::drawer::{Drawer, DrawerSideItem, DrawerSlot};
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let open = RwSignal::new(false);

    view! {
        <Drawer open=open>
            <DrawerSlot slot:drawer_content>
                <main>"Main content"</main>
            </DrawerSlot>
            <DrawerSlot slot:drawer_side>
                <DrawerSideItem>
                    <a href="/">"Home"</a>
                </DrawerSideItem>
                <DrawerSideItem>
                    <a href="/contact">"Contact"</a>
                </DrawerSideItem>
            </DrawerSlot>
        </Drawer>
    }
}
```

### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `open` | `RwSignal<bool>` | required | Controls drawer visibility from outside. |
| `id` | `&'static str` | `"main-drawer"` | The checkbox `id` and label `for`. Change if you have multiple drawers. |
| `drawer_content` | `DrawerSlot` | required | Main page content. |
| `drawer_side` | `DrawerSlot` | required | Drawer panel content, typically nav links. |

### Notes

- The drawer uses a hidden checkbox (`drawer-toggle`) bound to `open`, which is how DaisyUI toggles the panel.
- Clicking the overlay or the arrow button flips the signal.
- `DrawerSideItem` is a convenience `<li>` wrapper for the side panel list.

---

## `ToastProvider` / `ToastManager`

A self-contained toast stack with success, warning, and error variants. Includes an inline stylesheet so it works out of the box.

### Usage

```rust
use generik::components::toast::{ToastProvider, use_toast};
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <ToastProvider>
            <MyForm />
        </ToastProvider>
    }
}

#[component]
pub fn MyForm() -> impl IntoView {
    let toasts = use_toast();

    let submit = move |_| {
        toasts.success("Saved!");
    };

    view! {
        <button on:click=submit>"Submit"</button>
    }
}
```

### API

```rust
let toasts = use_toast();

toasts.push("Default message");
toasts.push_with("Custom kind", ToastKind::Warning);
toasts.success("It worked!");
toasts.warning("Heads up");
toasts.error("Something went wrong");
```

Toasts auto-dismiss after 4.5 seconds with a 300 ms exit animation. Clicking a toast dismisses it immediately.

---

## `SeoConfig` / `RouteMetaTags`

Reactive `<head>` tags driven by a context signal.

### Usage

```rust
use generik::components::seo::{RouteMetaTags, SeoConfig};
use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_context(RwSignal::new(SeoConfig {
        title: "My Site",
        description: "...",
        canonical: "https://example.com/",
        og_site_name: "My Site",
        og_title: "My Site",
        og_description: "...",
        og_type: "website",
        og_url: "https://example.com/",
        og_image: "https://example.com/image.jpg",
        og_image_alt: "...",
        twitter_card: "summary_large_image",
        twitter_title: "My Site",
        twitter_description: "...",
        twitter_image: "https://example.com/image.jpg",
    }));

    view! {
        <RouteMetaTags />
        <Router>
            <Routes>
                <Route path=StaticSegment("") view=Home />
            </Routes>
        </Router>
    }
}
```

On each page, update the signal:

```rust
#[component]
pub fn Home() -> impl IntoView {
    let seo = expect_context::<RwSignal<SeoConfig>>();
    seo.set(HOME_SEO);
    // ...
}
```

---

## Form primitives

### `FormStatus`

Tracks whether a form is currently submitting.

```rust
use generik::components::form::FormStatus;

let status = FormStatus::new();
status.submitting.set(true);
```

### `FieldErrors`

Per-field validation errors, provided via Leptos context.

```rust
use generik::components::form::{FieldErrors, FormCard, FormInput};
use leptos::prelude::*;

let errors = FieldErrors::new();
errors.set("email", "Please enter a valid email.");

view! {
    <FormCard title="Sign in" field_errors=errors.clone()>
        <FormInput id="email" name="email" label="Email" value=email />
    </FormCard>
}
```

### `FormInput` / `FormTextarea`

Two-way-bound input helpers. Pass a `RwSignal<String>` as `value`.

```rust
use generik::components::form::{FormInput, FormTextarea};

let name = RwSignal::new(String::new());
let message = RwSignal::new(String::new());

view! {
    <FormInput id="name" name="name" label="Name" value=name max_length=100 />
    <FormTextarea id="message" name="message" label="Message" value=message max_length=400 />
}
```

Both components read optional `FieldErrors` from context and render errors under the field.

---

## `FooterController`

A tiny state struct for showing/hiding a fixed footer. Used together with a storyteller.

```rust
use generik::components::overlay::FooterController;
use leptos::prelude::*;

let controller = RwSignal::new(FooterController::new());
controller.update(|c| c.toggle(true));
```

---

## `PageShell` / `PageSection`

Tailwind padding wrappers.

```rust
use generik::components::page::{PageSection, PageShell};

view! {
    <PageSection>
        "A full-height section"
    </PageSection>

    <PageSection full_page=false>
        "A regular section"
    </PageSection>
}
```

---

## `common`

Small utility components.

```rust
use generik::components::common::{LoadingSpinner, Spacer};

view! {
    <LoadingSpinner class="loading-sm" />
    <Spacer class="h-8" />
}
```

---

## `icons`

Heroicons-style SVGs.

```rust
use generik::components::icons::{Lightning, Mobile};

view! {
    <Lightning class="size-8 text-yellow-500" />
    <Mobile class="size-6" />
}
```
