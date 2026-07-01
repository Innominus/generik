use leptos::prelude::*;

use generik_layout::{LayoutStyles, Rule, RuleVariant};

use crate::atf::AboveTheFold;
use crate::code_viewer::{example_source, CodeBlock};
use crate::examples::*;
use crate::showcase::Showcase;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <LayoutStyles/>
        <div class="min-h-screen bg-paper text-ink antialiased font-body">
            <header class="border-b border-rule bg-paper">
                <div class="mx-auto max-w-6xl px-6 py-16">
                    <p class="gl-eyebrow">
                        "generik_layout"
                    </p>
                    <h1 class="gl-heading mt-2 text-4xl md:text-5xl">
                        "Swiss / editorial layout primitives for Leptos"
                    </h1>
                    <p class="mt-3 max-w-2xl text-ink-soft font-body">
                        "Interactive examples for every layout component. Each card below \
                        exposes live controls bound to reactive signals — resize your \
                        window and toggle the props to see the responsive behaviour."
                    </p>
                </div>
            </header>

            // Above-the-fold editorial sections, full-width (outside the constrained main).
            <AboveTheFold/>

            // A thin rule separating the ATF hero from the per-component reference.
            <Rule variant=RuleVariant::Thin/>

            <main class="mx-auto max-w-6xl px-6 py-20 space-y-20">
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
            </main>

            // Combined showcase — full-width (outside the constrained main) so its
            // full-bleed hero/ink panels and edge-to-edge rules read as intended.
            <ShowcaseSection/>

            <footer class="border-t border-rule bg-paper-alt">
                <div class="mx-auto max-w-6xl px-6 py-8 text-sm text-ink-muted font-mono">
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
        <section class="rounded-lg border border-rule bg-paper-alt p-8 shadow-sm">
            <div class="flex flex-col gap-1 border-b border-rule pb-6 mb-8">
                <div class="flex items-start justify-between gap-4">
                    <div>
                        <h2 class="gl-heading text-2xl">{title}</h2>
                        <p class="text-sm text-ink-soft font-body mt-1">{description}</p>
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
        <div class="flex rounded-md border border-rule overflow-hidden text-xs font-mono uppercase tracking-wider">
            <button
                type="button"
                class=move || if show_code.get() {
                    "px-3 py-1.5 text-ink-muted hover:bg-paper-alt hover:text-ink"
                } else {
                    "px-3 py-1.5 bg-ink text-paper"
                }
                on:click=move |_| show_code.set(false)
            >
                "Preview"
            </button>
            <button
                type="button"
                class=move || if show_code.get() {
                    "px-3 py-1.5 bg-ink text-paper"
                } else {
                    "px-3 py-1.5 text-ink-muted hover:bg-paper-alt hover:text-ink"
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
        <section class="pt-16 mt-16 border-t border-rule">
            <div class="mx-auto max-w-6xl">
                <div class="flex items-start justify-between gap-4">
                    <div>
                        <p class="gl-eyebrow">
                            "Combined showcase"
                        </p>
                        <h2 class="gl-heading mt-2 text-3xl md:text-4xl">
                            "Editorial homepage"
                        </h2>
                        <p class="mt-2 max-w-2xl text-ink-soft font-body">
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