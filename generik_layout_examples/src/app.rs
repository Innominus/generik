use leptos::prelude::*;

use generik_layout::LayoutStyles;

use crate::code_viewer::{example_source, CodeBlock};
use crate::examples::*;
use crate::showcase::Showcase;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <LayoutStyles/>
        <div class="min-h-screen bg-neutral-50 text-neutral-900 antialiased">
            <header class="border-b border-neutral-200 bg-white">
                <div class="mx-auto max-w-6xl px-6 py-10">
                    <p class="text-xs uppercase tracking-[0.2em] text-neutral-500">
                        "generik_layout"
                    </p>
                    <h1 class="mt-2 text-4xl font-semibold tracking-tight">
                        "Swiss / editorial layout primitives for Leptos"
                    </h1>
                    <p class="mt-3 max-w-2xl text-neutral-600">
                        "Interactive examples for every layout component. Each card below \
                        exposes live controls bound to reactive signals — resize your \
                        window and toggle the props to see the responsive behaviour."
                    </p>
                </div>
            </header>

            <main class="mx-auto max-w-6xl px-6 py-12 space-y-16">
                <Section title="Grid + Col" description="A 12-column responsive grid. Adjust the md breakpoint span and start, and the gap." code_name="GridExample">
                    <GridExample/>
                </Section>

                <Section title="Stack" description="A vertical flex with a configurable gap." code_name="StackExample">
                    <StackExample/>
                </Section>

                <Section title="Cluster" description="A wrapping flex row with alignment + justification controls." code_name="ClusterExample">
                    <ClusterExample/>
                </Section>

                <Section title="Sidebar" description="A two-column main + side layout. Toggle side and width." code_name="SidebarExample">
                    <SidebarExample/>
                </Section>

                <Section title="Center" description="Centers content on both axes within a parent. Change the parent height." code_name="CenterExample">
                    <CenterExample/>
                </Section>

                <Section title="Masonry" description="A dense CSS-grid mosaic. Change the column count and gap." code_name="MasonryExample">
                    <MasonryExample/>
                </Section>

                <Section title="Figure" description="An editorial figure: media with an optional caption below." code_name="FigureExample">
                    <FigureExample/>
                </Section>

                <Section title="MediaObject" description="Image-on-one-side, text-on-the-other. Collapses to stacked on mobile." code_name="MediaObjectExample">
                    <MediaObjectExample/>
                </Section>

                <Section title="PullQuote" description="A short extracted quote with optional attribution. Pure typography." code_name="PullQuoteExample">
                    <PullQuoteExample/>
                </Section>

                <Section title="Card" description="A general-purpose card with optional header, body, and footer slots." code_name="CardExample">
                    <CardExample/>
                </Section>

                <ShowcaseSection/>
            </main>

            <footer class="border-t border-neutral-200 bg-white">
                <div class="mx-auto max-w-6xl px-6 py-8 text-sm text-neutral-500">
                    "generik_layout examples — built with Leptos 0.8 + Tailwind Play CDN."
                </div>
            </footer>
        </div>
    }
}

/// A bordered card with a title + description, a Preview/Code tab toggle,
/// and the example body. The Code tab shows the exact Rust source of the
/// example (extracted from `examples.rs` / `showcase.rs` by marker name).
#[component]
fn Section(
    children: Children,
    title: &'static str,
    description: &'static str,
    code_name: &'static str,
) -> impl IntoView {
    let show_code = RwSignal::new(false);
    let source = example_source(code_name);

    view! {
        <section class="rounded-lg border border-neutral-200 bg-white p-6 shadow-sm">
            <div class="flex flex-col gap-1 border-b border-neutral-100 pb-4 mb-6">
                <div class="flex items-start justify-between gap-4">
                    <div>
                        <h2 class="text-2xl font-semibold tracking-tight">{title}</h2>
                        <p class="text-sm text-neutral-600">{description}</p>
                    </div>
                    <TabToggle show_code/>
                </div>
            </div>
            <div class=move || if show_code.get() { "hidden" } else { "" }>
                {children()}
            </div>
            <div class=move || if show_code.get() { "" } else { "hidden" }>
                <CodeBlock code=source.clone()/>
            </div>
        </section>
    }
}

/// The Preview/Code tab toggle buttons. `show_code` is the shared signal:
/// `false` = Preview (default), `true` = Code.
#[component]
fn TabToggle(show_code: RwSignal<bool>) -> impl IntoView {
    view! {
        <div class="flex rounded-md border border-neutral-200 overflow-hidden text-xs font-medium">
            <button
                type="button"
                class=move || if show_code.get() {
                    "px-3 py-1.5 text-neutral-500 hover:bg-neutral-50"
                } else {
                    "px-3 py-1.5 bg-neutral-900 text-white"
                }
                on:click=move |_| show_code.set(false)
            >
                "Preview"
            </button>
            <button
                type="button"
                class=move || if show_code.get() {
                    "px-3 py-1.5 bg-neutral-900 text-white"
                } else {
                    "px-3 py-1.5 text-neutral-500 hover:bg-neutral-50"
                }
                on:click=move |_| show_code.set(true)
            >
                "Code"
            </button>
        </div>
    }
}

/// The Showcase section: owns the Preview/Code tab state locally and passes
/// it down to `ShowcaseTabs` and `ShowcaseTabBody`, mirroring the `Section`
/// pattern instead of relying on module-level state.
#[component]
fn ShowcaseSection() -> impl IntoView {
    let show_code = RwSignal::new(false);

    view! {
        <section class="pt-8 border-t border-neutral-200">
            <div class="mx-auto max-w-6xl">
                <div class="flex items-start justify-between gap-4">
                    <div>
                        <p class="text-xs uppercase tracking-[0.2em] text-neutral-500">
                            "Combined showcase"
                        </p>
                        <h2 class="mt-2 text-3xl font-semibold tracking-tight">
                            "Editorial homepage"
                        </h2>
                        <p class="mt-2 max-w-2xl text-neutral-600">
                            "Every layout component composed into one Swiss-magazine page. \
                            Use the density toggle to reshape the whole layout at once."
                        </p>
                    </div>
                    <ShowcaseTabs show_code/>
                </div>
                <div class="mt-8">
                    <ShowcaseTabBody show_code/>
                </div>
            </div>
        </section>
    }
}

/// The Preview/Code tab toggle for the Showcase section.
#[component]
fn ShowcaseTabs(show_code: RwSignal<bool>) -> impl IntoView {
    view! { <TabToggle show_code/> }
}

/// The Showcase body: renders the live `Showcase` (Preview) and the
/// `CodeBlock` (Code) simultaneously, toggling visibility via the `hidden`
/// class so `Showcase`'s children are only constructed once.
#[component]
fn ShowcaseTabBody(show_code: RwSignal<bool>) -> impl IntoView {
    let source = example_source("Showcase");

    view! {
        <div class=move || if show_code.get() { "hidden" } else { "" }>
            <Showcase/>
        </div>
        <div class=move || if show_code.get() { "" } else { "hidden" }>
            <CodeBlock code=source.clone()/>
        </div>
    }
}