//! Combined editorial showcase using every generik_layout component together.

use leptos::prelude::*;

use generik_layout::{
    AlignItems, Center, Cluster, Col, Grid, Gutter, JustifyContent, Masonry, Side, Sidebar,
    SidebarSide, Stack,
};

/// Density presets reshaping the whole showcase at once.
#[derive(Clone, Copy, PartialEq)]
enum Density {
    Airy,
    Standard,
    Tight,
}

impl Density {
    fn gutter(self) -> Gutter {
        match self {
            Density::Airy => Gutter::Lg,
            Density::Standard => Gutter::Md,
            Density::Tight => Gutter::Sm,
        }
    }

    fn max_width(self) -> &'static str {
        match self {
            Density::Airy => "960px",
            Density::Standard => "1140px",
            Density::Tight => "1320px",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Density::Airy => "airy",
            Density::Standard => "standard",
            Density::Tight => "tight",
        }
    }

    fn next(self) -> Self {
        match self {
            Density::Airy => Density::Standard,
            Density::Standard => Density::Tight,
            Density::Tight => Density::Airy,
        }
    }
}

// BEGIN Showcase
#[component]
pub fn Showcase() -> impl IntoView {
    let density = RwSignal::new(Density::Standard);

    // Precomputed (class, style, label) triples — avoids format! in the reactive map.
    let photo_tiles: [(&'static str, &'static str, &'static str); 8] = [
        (
            "rounded-md bg-gradient-to-br from-rose-200 to-rose-400 flex items-end p-3 text-white text-sm font-medium",
            "grid-column: span 2; grid-row: span 2;",
            "Plate 1",
        ),
        (
            "rounded-md bg-gradient-to-br from-sky-200 to-sky-400 flex items-end p-3 text-white text-sm font-medium",
            "grid-column: span 1; grid-row: span 1;",
            "Plate 2",
        ),
        (
            "rounded-md bg-gradient-to-br from-amber-200 to-amber-400 flex items-end p-3 text-white text-sm font-medium",
            "grid-column: span 1; grid-row: span 2;",
            "Plate 3",
        ),
        (
            "rounded-md bg-gradient-to-br from-emerald-200 to-emerald-400 flex items-end p-3 text-white text-sm font-medium",
            "grid-column: span 2; grid-row: span 1;",
            "Plate 4",
        ),
        (
            "rounded-md bg-gradient-to-br from-violet-200 to-violet-400 flex items-end p-3 text-white text-sm font-medium",
            "grid-column: span 1; grid-row: span 1;",
            "Plate 5",
        ),
        (
            "rounded-md bg-gradient-to-br from-fuchsia-200 to-fuchsia-400 flex items-end p-3 text-white text-sm font-medium",
            "grid-column: span 1; grid-row: span 1;",
            "Plate 6",
        ),
        (
            "rounded-md bg-gradient-to-br from-cyan-200 to-cyan-400 flex items-end p-3 text-white text-sm font-medium",
            "grid-column: span 2; grid-row: span 1;",
            "Plate 7",
        ),
        (
            "rounded-md bg-gradient-to-br from-indigo-200 to-indigo-400 flex items-end p-3 text-white text-sm font-medium",
            "grid-column: span 1; grid-row: span 2;",
            "Plate 8",
        ),
    ];

    view! {
        <div class="flex items-center gap-3 mb-8">
            <button
                type="button"
                class="px-4 py-2 rounded-md bg-neutral-900 text-white text-sm font-medium hover:bg-neutral-700 transition-colors"
                on:click=move |_| density.set(density.get().next())
            >
                <span class="opacity-70">"density: "</span>
                {move || density.get().label()}
            </button>
            <span class="text-xs text-neutral-500">
                "cycles gutter + max-width across all sections"
            </span>
        </div>

        <div class="space-y-16">
            // Hero — Center inside a tall parent.
            <div
                class="rounded-lg border border-neutral-200 bg-gradient-to-br from-neutral-900 to-neutral-700 text-white overflow-hidden"
                style="height: 60vh; position: relative;"
            >
                <Center class="px-8">
                    <div class="text-center">
                        <p class="text-xs uppercase tracking-[0.3em] text-neutral-300">
                            "Issue 14 — June 2026"
                        </p>
                        <h2 class="mt-4 text-5xl md:text-7xl font-semibold tracking-tight">
                            "The quiet geometry of Swiss design"
                        </h2>
                        <p class="mt-4 text-neutral-300 max-w-xl mx-auto">
                            "A study in grids, gutters, and the spaces between."
                        </p>
                    </div>
                </Center>
            </div>

            // Single reactive wrapper for all density-driven sections — collapses
            // 5 distinct closure types into 1, reducing monomorphization.
            {move || {
                let d = density.get();
                view! {
                    // Editorial 2-column article layout via Grid + Col (asymmetric at md).
                    <Grid gap={d.gutter()} max_width={d.max_width()}>
                        <Col cols_md={8} start_md={1}>
                            <article class="prose-sm">
                                <p class="text-xs uppercase tracking-[0.2em] text-neutral-500">
                                    "Feature"
                                </p>
                                <h3 class="mt-2 text-3xl font-semibold tracking-tight">
                                    "On the discipline of the grid"
                                </h3>
                                <p class="mt-4 text-neutral-700 leading-relaxed">
                                    "The 12-column grid is less a constraint than a vocabulary. \
                                    Within it, asymmetry becomes legible: an 8-column article with \
                                    a 4-column rail, a 4-column pull-quote offset by a start, a \
                                    full-bleed image spanning all twelve. The grid gives the eye a \
                                    rhythm it can predict, then breaks that rhythm deliberately."
                                </p>
                                <p class="mt-3 text-neutral-700 leading-relaxed">
                                    "Muller-Brockmann's posters treat the grid as a musical staff: \
                                    intervals, repetitions, rests. The same is true here — the \
                                    responsive breakpoints are movements, each restating the theme \
                                    for a new scale."
                                </p>
                            </article>
                        </Col>
                        <Col cols_md={4} start_md={9}>
                            <div class="rounded-md border border-neutral-200 bg-neutral-50 p-5">
                                <p class="text-xs uppercase tracking-[0.2em] text-neutral-500">
                                    "Pull-quote"
                                </p>
                                <blockquote class="mt-3 text-lg font-medium text-neutral-900 leading-snug">
                                    "\"The grid is the structure on which the work is built — \
                                    and like a foundation, it should be felt, not seen.\""
                                </blockquote>
                                <p class="mt-3 text-sm text-neutral-500">"— Josef Müller-Brockmann"</p>
                            </div>
                        </Col>
                    </Grid>

                    // Related articles rail via Sidebar.
                    <Sidebar side={Side::Right} side_width="320px" gap={d.gutter()}>
                        <div>
                            <p class="text-xs uppercase tracking-[0.2em] text-neutral-500">
                                "Continue reading"
                            </p>
                            <h3 class="mt-2 text-2xl font-semibold tracking-tight">
                                "Typography as architecture"
                            </h3>
                            <p class="mt-3 text-neutral-700 leading-relaxed">
                                "Where the grid organises space, type organises meaning. \
                                Swiss typography favours restraint: a single sans-serif family, \
                                a tight scale, generous whitespace. The result reads as quiet \
                                authority — the page does not shout, it states."
                            </p>
                            <p class="mt-3 text-neutral-700 leading-relaxed">
                                "Set body text at a measure of 60–75 characters; let headings \
                                breathe above and below; align captions to the baseline. These \
                                small rules compound into a coherent whole."
                            </p>
                        </div>
                        <SidebarSide slot:side_slot>
                            <div class="rounded-md border border-neutral-200 bg-white p-5">
                                <p class="text-xs uppercase tracking-[0.2em] text-neutral-500">
                                    "Related"
                                </p>
                                <ul class="mt-3 space-y-3 text-sm">
                                    <li class="border-b border-neutral-100 pb-3">
                                        <p class="font-medium text-neutral-900">"Grid systems in print"</p>
                                        <p class="text-neutral-500 text-xs mt-0.5">"8 min read"</p>
                                    </li>
                                    <li class="border-b border-neutral-100 pb-3">
                                        <p class="font-medium text-neutral-900">"The baseline as rhythm"</p>
                                        <p class="text-neutral-500 text-xs mt-0.5">"5 min read"</p>
                                    </li>
                                    <li class="border-b border-neutral-100 pb-3">
                                        <p class="font-medium text-neutral-900">"Whitespace as material"</p>
                                        <p class="text-neutral-500 text-xs mt-0.5">"6 min read"</p>
                                    </li>
                                    <li>
                                        <p class="font-medium text-neutral-900">"Asymmetry and balance"</p>
                                        <p class="text-neutral-500 text-xs mt-0.5">"7 min read"</p>
                                    </li>
                                </ul>
                            </div>
                        </SidebarSide>
                    </Sidebar>

                    // Tags via Cluster.
                    <Cluster gap={d.gutter()} align={AlignItems::Center} justify={JustifyContent::Start}>
                        {["Essay", "Design", "Typography", "Grid", "Swiss", "Editorial", "Layout", "History"]
                            .into_iter()
                            .map(|t| view! {
                                <span class="px-3 py-1.5 rounded-full bg-neutral-900 text-white text-xs font-medium tracking-wide">
                                    {t.to_string()}
                                </span>
                            })
                            .collect::<Vec<_>>()}
                    </Cluster>

                    // Article previews via Stack.
                    <Stack gap={d.gutter()}>
                        {[
                            ("01", "The modular system", "A module is a fixed unit; the grid is its field. Together they let a design scale without losing proportion."),
                            ("02", "Columns and measure", "Twelve columns divide cleanly into halves, thirds, quarters and sixths — enough latitude for almost any layout."),
                            ("03", "The role of the baseline", "Vertical rhythm is set by the baseline grid. Headings, body and captions all snap to it; the page feels composed rather than assembled."),
                        ]
                            .into_iter()
                            .map(|(n, title, body)| view! {
                                <article class="grid grid-cols-[auto_1fr] gap-6 border-b border-neutral-200 pb-6">
                                    <span class="text-3xl font-semibold text-neutral-300 tabular-nums">{n.to_string()}</span>
                                    <div>
                                        <h4 class="text-xl font-semibold tracking-tight">{title.to_string()}</h4>
                                        <p class="mt-1 text-sm text-neutral-600 leading-relaxed">{body.to_string()}</p>
                                    </div>
                                </article>
                            })
                            .collect::<Vec<_>>()}
                    </Stack>

                    // Photo grid via Masonry.
                    <div>
                        <p class="text-xs uppercase tracking-[0.2em] text-neutral-500 mb-4">
                            "Gallery"
                        </p>
                        <Masonry cols={4} gap={d.gutter()}>
                            {photo_tiles
                                .into_iter()
                                .map(|(class_str, style_str, label)| {
                                    view! {
                                        <div
                                            class=class_str
                                            style=style_str
                                        >
                                            {label}
                                        </div>
                                    }
                                })
                                .collect::<Vec<_>>()}
                        </Masonry>
                    </div>
                }
            }}
        </div>
    }
}
// END Showcase