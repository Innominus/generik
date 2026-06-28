//! Per-component interactive examples.
//!
//! Each example follows the same pattern: a set of `RwSignal` controls (buttons /
//! range sliders) drive plain props on generik_layout components. Because the props
//! are plain values (not signals), the component itself is wrapped in
//! `move || view! { <Component prop={signal.get()}>...</Component> }` so the signal
//! reads are tracked and the view re-renders when a signal changes.

use leptos::prelude::*;

use generik_layout::{
    AlignItems, Attribution, Body, Card, CardBody, CardFooter, CardHeader, Center, Cluster, Col,
    Figure, FigureCaption, FigureMedia, Grid, Gutter, JustifyContent, Media, MediaObject,
    Masonry, PullQuote, Quote, Side, Sidebar, SidebarSide, Stack,
};

use std::sync::Arc;

// ===== Shared control helpers =====

/// A small labelled button used for discrete cycling controls.
#[component]
fn CycleButton(
    label: &'static str,
    value: Box<dyn Fn() -> String + Send>,
    on_click: Box<dyn Fn()>,
) -> impl IntoView {
    view! {
        <button
            type="button"
            class="px-3 py-1.5 rounded-md border border-neutral-300 bg-white text-sm font-medium text-neutral-700 hover:bg-neutral-100 active:bg-neutral-200 transition-colors"
            on:click=move |_| on_click()
        >
            <span class="text-neutral-500">{label}: </span>
            <span class="text-neutral-900">{move || value()}</span>
        </button>
    }
}

/// A labelled range slider showing the current numeric value.
#[component]
fn RangeControl(
    label: &'static str,
    min: i64,
    max: i64,
    value: Arc<dyn Fn() -> i64 + Send + Sync>,
    on_input: Box<dyn Fn(i64)>,
) -> impl IntoView {
    view! {
        <label class="flex items-center gap-3 text-sm text-neutral-700">
            <span class="w-20 text-neutral-500">{label}</span>
            <input
                type="range"
                min=min
                max=max
                prop:value={
                    let value = value.clone();
                    move || value().to_string()
                }
                class="flex-1 accent-neutral-900"
                on:input=move |ev| {
                    let raw = event_target_value(&ev);
                    if let Ok(n) = raw.parse::<i64>() {
                        on_input(n);
                    }
                }
            />
            <span class="w-10 text-right font-mono text-neutral-900">{move || value().to_string()}</span>
        </label>
    }
}

/// A panel holding a row of controls.
#[component]
fn Controls(children: Children) -> impl IntoView {
    view! {
        <div class="flex flex-wrap items-center gap-3 rounded-md bg-neutral-50 border border-neutral-200 p-3 mb-6">
            {children()}
        </div>
    }
}

fn gutter_name(g: Gutter) -> &'static str {
    match g {
        Gutter::Sm => "sm",
        Gutter::Md => "md",
        Gutter::Lg => "lg",
    }
}

fn cycle_gutter(g: Gutter) -> Gutter {
    match g {
        Gutter::Sm => Gutter::Md,
        Gutter::Md => Gutter::Lg,
        Gutter::Lg => Gutter::Sm,
    }
}

fn side_name(s: Side) -> &'static str {
    match s {
        Side::Left => "left",
        Side::Right => "right",
    }
}

fn cycle_side(s: Side) -> Side {
    match s {
        Side::Left => Side::Right,
        Side::Right => Side::Left,
    }
}

fn align_name(a: AlignItems) -> &'static str {
    match a {
        AlignItems::Start => "start",
        AlignItems::Center => "center",
        AlignItems::End => "end",
        AlignItems::Stretch => "stretch",
        AlignItems::Baseline => "baseline",
    }
}

fn cycle_align(a: AlignItems) -> AlignItems {
    match a {
        AlignItems::Start => AlignItems::Center,
        AlignItems::Center => AlignItems::End,
        AlignItems::End => AlignItems::Stretch,
        AlignItems::Stretch => AlignItems::Baseline,
        AlignItems::Baseline => AlignItems::Start,
    }
}

fn justify_name(j: JustifyContent) -> &'static str {
    match j {
        JustifyContent::Start => "start",
        JustifyContent::Center => "center",
        JustifyContent::End => "end",
        JustifyContent::Between => "between",
        JustifyContent::Around => "around",
        JustifyContent::Evenly => "evenly",
    }
}

fn cycle_justify(j: JustifyContent) -> JustifyContent {
    match j {
        JustifyContent::Start => JustifyContent::Center,
        JustifyContent::Center => JustifyContent::Between,
        JustifyContent::Between => JustifyContent::Around,
        JustifyContent::Around => JustifyContent::Evenly,
        JustifyContent::Evenly => JustifyContent::End,
        JustifyContent::End => JustifyContent::Start,
    }
}

// ===== GridExample =====

// BEGIN GridExample
#[component]
pub fn GridExample() -> impl IntoView {
    let cols_md = RwSignal::new(6usize);
    // start_md cycles None -> Some(1) -> Some(3) -> Some(7) -> None.
    let start_md = RwSignal::new(None::<usize>);
    let gutter = RwSignal::new(Gutter::Md);

    view! {
        <Controls>
            <RangeControl
                label="cols_md"
                min=1
                max=12
                value=Arc::new(move || cols_md.get() as i64)
                on_input=Box::new(move |n| cols_md.set(n as usize))
            />
            <CycleButton
                label="start_md"
                value=Box::new(move || start_md.get().map(|s| s.to_string()).unwrap_or_else(|| "None".to_owned()))
                on_click=Box::new(move || {
                    start_md.set(match start_md.get() {
                        None => Some(1),
                        Some(1) => Some(3),
                        Some(3) => Some(7),
                        Some(7) => None,
                        _ => None,
                    });
                })
            />
            <CycleButton
                label="gutter"
                value=Box::new(move || gutter_name(gutter.get()).to_owned())
                on_click=Box::new(move || gutter.set(cycle_gutter(gutter.get())))
            />
        </Controls>

        {move || {
            // Read all signals up front so the closure depends on each one and
            // re-runs (reconstructing the Cols with fresh props) when any changes.
            // Reading cols_md only inside the `view!` match arms left it in a
            // child reactive scope that didn't reliably propagate slider changes.
            let cm = cols_md.get();
            let sm = start_md.get();
            let g = gutter.get();
            view! {
                <Grid gap={g}>
                    {match sm {
                        Some(s) => view! {
                            <Col
                                cols_md={cm}
                                start_md={s}
                                class="demo-col"
                            >
                                <div class="h-24 rounded-md bg-sky-100 border border-sky-300 flex items-center justify-center text-sky-900 font-medium">
                                    "Col A"
                                </div>
                            </Col>
                        }.into_any(),
                        None => view! {
                            <Col cols_md={cm} class="demo-col">
                                <div class="h-24 rounded-md bg-sky-100 border border-sky-300 flex items-center justify-center text-sky-900 font-medium">
                                    "Col A"
                                </div>
                            </Col>
                        }.into_any(),
                    }}
                    <Col cols_md={cm} class="demo-col">
                        <div class="h-24 rounded-md bg-emerald-100 border border-emerald-300 flex items-center justify-center text-emerald-900 font-medium">
                            "Col B"
                        </div>
                    </Col>
                </Grid>
            }
        }}

        <p class="mt-4 text-xs text-neutral-500">
            {move || {
                match start_md.get() {
                    None => "start_md is None — Col A inherits the default (no explicit start) at md.".to_string(),
                    Some(s) => format!("start_md = Some({s}) — Col A begins at grid line {s} at md+."),
                }
            }}
            " Resize the viewport past 768px to see the md breakpoint take effect."
        </p>
    }
}
// END GridExample

// ===== StackExample =====

// BEGIN StackExample
#[component]
pub fn StackExample() -> impl IntoView {
    let gap = RwSignal::new(Gutter::Md);

    view! {
        <Controls>
            <CycleButton
                label="gap"
                value=Box::new(move || gutter_name(gap.get()).to_owned())
                on_click=Box::new(move || gap.set(cycle_gutter(gap.get())))
            />
        </Controls>

        {move || view! {
            <Stack gap={gap.get()}>
                <div class="h-16 rounded-md bg-indigo-100 border border-indigo-300 px-4 flex items-center text-indigo-900 font-medium">
                    "Item 1"
                </div>
                <div class="h-16 rounded-md bg-indigo-100 border border-indigo-300 px-4 flex items-center text-indigo-900 font-medium">
                    "Item 2"
                </div>
                <div class="h-16 rounded-md bg-indigo-100 border border-indigo-300 px-4 flex items-center text-indigo-900 font-medium">
                    "Item 3"
                </div>
            </Stack>
        }}
    }
}
// END StackExample

// ===== ClusterExample =====

// BEGIN ClusterExample
#[component]
pub fn ClusterExample() -> impl IntoView {
    let gap = RwSignal::new(Gutter::Md);
    let align = RwSignal::new(AlignItems::Center);
    let justify = RwSignal::new(JustifyContent::Start);

    view! {
        <Controls>
            <CycleButton
                label="gap"
                value=Box::new(move || gutter_name(gap.get()).to_owned())
                on_click=Box::new(move || gap.set(cycle_gutter(gap.get())))
            />
            <CycleButton
                label="align"
                value=Box::new(move || align_name(align.get()).to_owned())
                on_click=Box::new(move || align.set(cycle_align(align.get())))
            />
            <CycleButton
                label="justify"
                value=Box::new(move || justify_name(justify.get()).to_owned())
                on_click=Box::new(move || justify.set(cycle_justify(justify.get())))
            />
        </Controls>

        {move || view! {
            <Cluster gap={gap.get()} align={align.get()} justify={justify.get()}>
                <div class="h-10 px-4 rounded-full bg-rose-100 border border-rose-300 text-rose-900 flex items-center text-sm font-medium">
                    "Tag A"
                </div>
                <div class="h-16 px-4 rounded-full bg-rose-100 border border-rose-300 text-rose-900 flex items-center text-sm font-medium">
                    "Tag B (tall)"
                </div>
                <div class="h-8 px-4 rounded-full bg-rose-100 border border-rose-300 text-rose-900 flex items-center text-sm font-medium">
                    "Tag C (short)"
                </div>
                <div class="h-12 px-4 rounded-full bg-rose-100 border border-rose-300 text-rose-900 flex items-center text-sm font-medium">
                    "Tag D"
                </div>
                <div class="h-10 px-4 rounded-full bg-rose-100 border border-rose-300 text-rose-900 flex items-center text-sm font-medium">
                    "Tag E"
                </div>
                <div class="h-14 px-4 rounded-full bg-rose-100 border border-rose-300 text-rose-900 flex items-center text-sm font-medium">
                    "Tag F"
                </div>
            </Cluster>
        }}
    }
}
// END ClusterExample

// ===== SidebarExample =====

// BEGIN SidebarExample
#[component]
pub fn SidebarExample() -> impl IntoView {
    let sidebar = RwSignal::new(Side::Left);
    let side_width = RwSignal::new("280px");

    let cycle_width = move || match side_width.get() {
        "200px" => "280px",
        "280px" => "400px",
        _ => "200px",
    };

    view! {
        <Controls>
            <CycleButton
                label="side"
                value=Box::new(move || side_name(sidebar.get()).to_owned())
                on_click=Box::new(move || sidebar.set(cycle_side(sidebar.get())))
            />
            <CycleButton
                label="side_width"
                value=Box::new(move || side_width.get().to_owned())
                on_click=Box::new(move || side_width.set(cycle_width()))
            />
        </Controls>

        {move || view! {
            <Sidebar side={sidebar.get()} side_width={side_width.get()}>
                <div class="rounded-md bg-amber-50 border border-amber-200 p-5">
                    <h3 class="text-lg font-semibold text-amber-900">"Main content"</h3>
                    <p class="mt-2 text-sm text-amber-800">
                        "This is the wide main column. On viewports below 768px the layout \
                        collapses to a single column (mobile-first). The narrow rail appears \
                        on the chosen side on wider screens."
                    </p>
                    <p class="mt-3 text-sm text-amber-800">
                        "The side column keeps its own width while this column flexes to fill \
                        the remaining space."
                    </p>
                </div>
                <SidebarSide slot:side_slot>
                    <nav class="rounded-md bg-amber-100 border border-amber-200 p-4">
                        <h4 class="text-xs uppercase tracking-wider text-amber-900 mb-3">"Navigation"</h4>
                        <ul class="space-y-2 text-sm text-amber-900">
                            <li>"→ Introduction"</li>
                            <li>"→ Sections"</li>
                            <li>"→ Gallery"</li>
                            <li>"→ Related"</li>
                            <li>"→ Footer"</li>
                        </ul>
                    </nav>
                </SidebarSide>
            </Sidebar>
        }}
    }
}
// END SidebarExample

// ===== CenterExample =====

// BEGIN CenterExample
#[component]
pub fn CenterExample() -> impl IntoView {
    let min_h = RwSignal::new("200px");
    let cycle_min_h = move || match min_h.get() {
        "200px" => "400px",
        "400px" => "80vh",
        _ => "200px",
    };

    view! {
        <Controls>
            <CycleButton
                label="parent height"
                value=Box::new(move || min_h.get().to_owned())
                on_click=Box::new(move || min_h.set(cycle_min_h()))
            />
        </Controls>

        {move || view! {
            <div
                class="rounded-md border border-neutral-300 overflow-hidden"
                style=format!("height: {}; position: relative;", min_h.get())
            >
                <Center>
                    <div class="rounded-md bg-violet-100 border border-violet-300 px-6 py-4 text-violet-900 font-medium">
                        "Centered content"
                    </div>
                </Center>
            </div>
        }}
    }
}
// END CenterExample

// ===== MasonryExample =====

// BEGIN MasonryExample
#[component]
pub fn MasonryExample() -> impl IntoView {
    let cols = RwSignal::new(4usize);
    let gap = RwSignal::new(Gutter::Md);

    let cycle_cols = move || match cols.get() {
        2 => 3,
        3 => 4,
        4 => 6,
        _ => 2,
    };

    // 8 items of varying row spans to show dense packing.
    // Precomputed (class, style, label) triples — avoids format! in the reactive map.
    let items: [(&'static str, &'static str, &'static str); 8] = [
        (
            "rounded-md border p-4 flex items-center justify-center font-medium bg-teal-100 border-teal-300 text-teal-900",
            "grid-column: span 1; grid-row: span 2;",
            "#0",
        ),
        (
            "rounded-md border p-4 flex items-center justify-center font-medium bg-teal-200 border-teal-300 text-teal-900",
            "grid-column: span 2; grid-row: span 1;",
            "#1",
        ),
        (
            "rounded-md border p-4 flex items-center justify-center font-medium bg-teal-100 border-teal-300 text-teal-900",
            "grid-column: span 1; grid-row: span 3;",
            "#2",
        ),
        (
            "rounded-md border p-4 flex items-center justify-center font-medium bg-teal-200 border-teal-300 text-teal-900",
            "grid-column: span 2; grid-row: span 2;",
            "#3",
        ),
        (
            "rounded-md border p-4 flex items-center justify-center font-medium bg-teal-100 border-teal-300 text-teal-900",
            "grid-column: span 1; grid-row: span 1;",
            "#4",
        ),
        (
            "rounded-md border p-4 flex items-center justify-center font-medium bg-teal-200 border-teal-300 text-teal-900",
            "grid-column: span 3; grid-row: span 1;",
            "#5",
        ),
        (
            "rounded-md border p-4 flex items-center justify-center font-medium bg-teal-100 border-teal-300 text-teal-900",
            "grid-column: span 1; grid-row: span 2;",
            "#6",
        ),
        (
            "rounded-md border p-4 flex items-center justify-center font-medium bg-teal-200 border-teal-300 text-teal-900",
            "grid-column: span 2; grid-row: span 1;",
            "#7",
        ),
    ];

    view! {
        <Controls>
            <CycleButton
                label="cols"
                value=Box::new(move || cols.get().to_string())
                on_click=Box::new(move || cols.set(cycle_cols()))
            />
            <CycleButton
                label="gap"
                value=Box::new(move || gutter_name(gap.get()).to_owned())
                on_click=Box::new(move || gap.set(cycle_gutter(gap.get())))
            />
        </Controls>

        {move || view! {
            <Masonry cols={cols.get()} gap={gap.get()}>
                {items
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
        }}
    }
}
// END MasonryExample

// ===== FigureExample =====

// BEGIN FigureExample
#[component]
pub fn FigureExample() -> impl IntoView {
    view! {
        <Figure class="max-w-md">
            <FigureMedia slot:media_slot>
                <div
                    class="h-48 rounded-md bg-gradient-to-br from-sky-200 to-sky-500 flex items-center justify-center"
                >
                    <span class="text-white text-sm font-medium tracking-wide">
                        "16:9 placeholder image"
                    </span>
                </div>
            </FigureMedia>
            <FigureCaption slot:caption_slot>
                "Plate 1 — A composition of sky-blue blocks demonstrating the Figure component. \
                The caption sits below the media and uses a smaller, muted type style."
            </FigureCaption>
        </Figure>
    }
}
// END FigureExample

// ===== MediaObjectExample =====

// BEGIN MediaObjectExample
#[component]
pub fn MediaObjectExample() -> impl IntoView {
    view! {
        <MediaObject media_width="96px">
            <Media slot:media_slot>
                <div
                    class="h-24 w-24 rounded-md bg-gradient-to-br from-rose-300 to-rose-500 flex items-center justify-center text-white font-bold text-2xl"
                >
                    "G"
                </div>
            </Media>
            <Body slot:body_slot>
                <h4 class="text-xl font-semibold tracking-tight text-neutral-900">
                    "Grid systems in print"
                </h4>
                <p class="mt-2 text-sm text-neutral-600 leading-relaxed">
                    "The media column holds a fixed-width avatar while the body column flexes \
                    to fill the remaining space. On narrow viewports the layout collapses to a \
                    stacked single column."
                </p>
            </Body>
        </MediaObject>
    }
}
// END MediaObjectExample

// ===== PullQuoteExample =====

// BEGIN PullQuoteExample
#[component]
pub fn PullQuoteExample() -> impl IntoView {
    view! {
        <PullQuote class="max-w-2xl">
            <Quote slot:quote_slot>
                "The grid is the structure on which the work is built — and like a foundation, \
                it should be felt, not seen."
            </Quote>
            <Attribution slot:attribution_slot>
                "— Josef Müller-Brockmann"
            </Attribution>
        </PullQuote>
    }
}
// END PullQuoteExample

// ===== CardExample =====

// BEGIN CardExample
#[component]
pub fn CardExample() -> impl IntoView {
    view! {
        <Card class="max-w-md">
            <CardHeader slot:header_slot>
                <h4 class="text-lg font-semibold tracking-tight text-neutral-900">
                    "Editorial card"
                </h4>
                <p class="text-xs uppercase tracking-[0.2em] text-neutral-500 mt-1">
                    "Header slot"
                </p>
            </CardHeader>
            <CardBody slot:body_slot>
                <p class="text-sm text-neutral-700 leading-relaxed">
                    "Cards compose three optional regions — header, body, and footer — rendered \
                    in source order. Borders separate each region; the body flexes to fill \
                    remaining space."
                </p>
            </CardBody>
            <CardFooter slot:footer_slot>
                <div class="flex items-center justify-between text-sm text-neutral-500">
                    <span>"Footer slot"</span>
                    <span class="font-medium text-neutral-900">"Read more →"</span>
                </div>
            </CardFooter>
        </Card>
    }
}
// END CardExample