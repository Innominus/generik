//! Combined editorial showcase using every generik_layout component together.

use leptos::prelude::*;

use generik_layout::{
    AlignItems, Attribution, Body, Card, CardBody, CardFooter, CardHeader, Center, Cluster, Col,
    Container, Dropcap, Figure, FigureCaption, FigureMedia, Grid, Gutter, JustifyContent,
    Masonry, Masthead, Media, MediaObject, Measure, PullQuote, Quote, Rule, RuleVariant, Side,
    Sidebar, SidebarSide, Stack, Stat,
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
            "rounded-md bg-gradient-to-br from-ink to-ink-soft flex items-end p-3 text-paper text-sm font-medium",
            "grid-column: span 2; grid-row: span 2;",
            "Plate 1",
        ),
        (
            "rounded-md bg-gradient-to-br from-paper-alt to-rule flex items-end p-3 text-ink text-sm font-medium",
            "grid-column: span 1; grid-row: span 1;",
            "Plate 2",
        ),
        (
            "rounded-md bg-gradient-to-br from-accent to-ink flex items-end p-3 text-paper text-sm font-medium",
            "grid-column: span 1; grid-row: span 2;",
            "Plate 3",
        ),
        (
            "rounded-md bg-gradient-to-br from-paper to-paper-alt flex items-end p-3 text-ink text-sm font-medium",
            "grid-column: span 2; grid-row: span 1;",
            "Plate 4",
        ),
        (
            "rounded-md bg-gradient-to-br from-rule to-ink-muted flex items-end p-3 text-paper text-sm font-medium",
            "grid-column: span 1; grid-row: span 1;",
            "Plate 5",
        ),
        (
            "rounded-md bg-gradient-to-br from-ink-soft to-ink-muted flex items-end p-3 text-paper text-sm font-medium",
            "grid-column: span 1; grid-row: span 1;",
            "Plate 6",
        ),
        (
            "rounded-md bg-gradient-to-br from-paper-alt to-rule flex items-end p-3 text-ink text-sm font-medium",
            "grid-column: span 2; grid-row: span 1;",
            "Plate 7",
        ),
        (
            "rounded-md bg-gradient-to-br from-ink to-ink-soft flex items-end p-3 text-paper text-sm font-medium",
            "grid-column: span 1; grid-row: span 2;",
            "Plate 8",
        ),
    ];

    // Stat row data: (number, label) — Swiss poster data strip.
    let stats: [(&'static str, &'static str); 4] = [
        ("14", "Issues published"),
        ("60+", "Essays archived"),
        ("1.2k", "Subscribers"),
        ("2019", "Established"),
    ];

    // Related-reading rail items: (meta, title).
    let related: [(&'static str, &'static str); 4] = [
        ("8 min · Essay", "Grid systems in print"),
        ("5 min · Note", "The baseline as rhythm"),
        ("6 min · Essay", "Whitespace as material"),
        ("7 min · Review", "Asymmetry and balance"),
    ];

    // Numbered preview list: (num, title, body).
    let previews: [(&'static str, &'static str, &'static str); 3] = [
        (
            "01",
            "The modular system",
            "A module is a fixed unit; the grid is its field. Together they let a design \
            scale without losing proportion — the same module that frames a poster can \
            tile a page, a screen, a signage system.",
        ),
        (
            "02",
            "Columns and measure",
            "Twelve columns divide cleanly into halves, thirds, quarters and sixths — \
            enough latitude for almost any layout, while remaining legible at a glance \
            to the designer setting type within it.",
        ),
        (
            "03",
            "The role of the baseline",
            "Vertical rhythm is set by the baseline grid. Headings, body and captions all \
            snap to it; the page feels composed rather than assembled, the eye never \
            trips on a stray line.",
        ),
    ];

    // Voices pull-quotes: (quote, attribution). Length must stay 3 — indexed by
    // [0]/[1]/[2] in the asymmetric grid below; adding/removing requires
    // updating all three indices.
    let voices: [(&'static str, &'static str); 3] = [
        (
            "The grid is the structure on which the work is built — and like a foundation, \
            it should be felt, not seen.",
            "— Josef Müller-Brockmann",
        ),
        (
            "Typography is the visual equivalent of speech — and silence. The white space \
            is not waste; it is the breath between phrases.",
            "— Helmut Schmid",
        ),
        (
            "Restraint is not the absence of expression. It is expression, chosen.",
            "— Emil Ruder",
        ),
    ];

    view! {
        // ===== Density control — editorially restyled. =====
        <div class="flex items-center gap-4 mb-12">
            <button
                type="button"
                class="font-mono text-xs uppercase tracking-widest text-ink border border-rule px-4 py-2 hover:border-ink hover:text-accent transition-colors"
                on:click=move |_| density.set(density.get().next())
            >
                <span class="text-ink-muted">"density: "</span>
                {move || density.get().label()}
            </button>
            <span class="gl-meta">
                "cycles gutter + max-width across every section"
            </span>
        </div>

        // ===== Masthead — static, sits above the density-driven body. =====
        <Masthead name="QUADRA" issue="Issue 14 — June 2026" tagline="A quarterly on the discipline of the grid"/>

        // ===== Stat row — Swiss poster data strip, spread across the width. =====
        <Cluster gap={Gutter::Lg} align={AlignItems::Baseline} justify={JustifyContent::Between} class="py-8 px-4">
            {stats
                .into_iter()
                .map(|(n, l)| view! {
                    <Stat number=n label=l />
                })
                .collect::<Vec<_>>()}
        </Cluster>

        <Rule variant=RuleVariant::Thin/>

        // ===== Hero — full-bleed, warm ink gradient, capped content width. =====
        <div
            class="bg-gradient-to-br from-ink to-ink-soft text-paper overflow-hidden my-12"
            style="min-height: 60vh;"
        >
            <Center class="px-6 py-24">
                <Container width="760px" class="text-center">
                    <p class="gl-eyebrow text-paper-alt">"Cover story"</p>
                    <h2 class="gl-display text-paper mt-6">"The quiet geometry of Swiss design"</h2>
                    <p class="gl-lede text-paper-alt mt-6">
                        "A study in grids, gutters, and the spaces between — where the \
                        discipline of the page becomes a vocabulary for everything placed on it."
                    </p>
                </Container>
            </Center>
        </div>

        // ===== Single reactive wrapper for all density-driven sections. =====
        // Collapses the monomorphization of 5+ distinct closure types into one.
        {move || {
            let d = density.get();
            view! {
                // ----- Feature 01 — asymmetric 8/4 grid split with dropcap prose. -----
                <Grid gap={d.gutter()} max_width={d.max_width()}>
                    <Col cols_md={8} start_md={1}>
                        <article>
                            <p class="gl-eyebrow">"Feature 01"</p>
                            <h3 class="gl-heading text-3xl md:text-4xl mt-3">"On the discipline of the grid"</h3>
                            <p class="gl-lede mt-4">"The grid is not a cage. It is the staff on which the music of the page is written."</p>
                            <div class="gl-prose mt-6">
                                <Dropcap>
                                    <p>"The 12-column grid is less a constraint than a vocabulary. \
                                    Within it, asymmetry becomes legible: an 8-column article with a \
                                    4-column rail, a 4-column pull-quote offset by a start, a full-bleed \
                                    image spanning all twelve. The grid gives the eye a rhythm it can \
                                    predict, then breaks that rhythm deliberately."</p>
                                </Dropcap>
                                <p>"Müller-Brockmann's posters treat the grid as a musical staff: \
                                intervals, repetitions, rests. The same is true here — the responsive \
                                breakpoints are movements, each restating the theme for a new scale. \
                                What looks like restraint is in fact a great deal of decisions, made \
                                once and held to."</p>
                                <p>"Whitespace is the material the grid shapes. A page that breathes \
                                between its columns reads as confident; a page packed edge to edge \
                                reads as anxious. The discipline is to leave the margin alone, to let \
                                the type find its measure, to resist the urge to fill."</p>
                            </div>
                        </article>
                    </Col>
                    <Col cols_md={4} start_md={9}>
                        <PullQuote class="text-ink">
                            <Quote slot:quote_slot>
                                "The grid is the structure on which the work is built — and like a \
                                foundation, it should be felt, not seen."
                            </Quote>
                            <Attribution slot:attribution_slot>"— Josef Müller-Brockmann"</Attribution>
                        </PullQuote>
                    </Col>
                </Grid>

                <Rule variant=RuleVariant::Thin label="Further reading"/>

                // ----- Feature 02 — Sidebar with a related-reading rail of Cards. -----
                <Sidebar side={Side::Right} side_width="320px" gap={d.gutter()}>
                    <Measure measure="60ch">
                        <p class="gl-eyebrow">"Feature 02"</p>
                        <h3 class="gl-heading text-2xl md:text-3xl mt-3">"Typography as architecture"</h3>
                        <div class="gl-prose mt-4">
                            <p>"Where the grid organises space, type organises meaning. Swiss \
                            typography favours restraint: a single sans-serif family, a tight scale, \
                            generous whitespace. The result reads as quiet authority — the page does \
                            not shout, it states."</p>
                            <p>"Set body text at a measure of 60–75 characters; let headings breathe \
                            above and below; align captions to the baseline. These small rules \
                            compound into a coherent whole, the way a building's proportions are \
                            felt long before they are measured."</p>
                        </div>
                    </Measure>
                    <SidebarSide slot:side_slot>
                        <Stack gap={d.gutter()}>
                            <p class="gl-eyebrow">"Related"</p>
                            {related
                                .into_iter()
                                .map(|(meta, title)| view! {
                                    <Card class="bg-paper border-rule">
                                        <CardBody slot:body_slot>
                                            <p class="gl-meta">{meta}</p>
                                            <p class="font-display font-semibold text-ink mt-1">{title}</p>
                                        </CardBody>
                                    </Card>
                                })
                                .collect::<Vec<_>>()}
                        </Stack>
                    </SidebarSide>
                </Sidebar>

                // ----- Tags — editorial pills, mono on a rule border. -----
                <Cluster gap={d.gutter()} align={AlignItems::Center} justify={JustifyContent::Start} class="py-8">
                    {["Essay", "Design", "Typography", "Grid", "Swiss", "Editorial", "Layout", "History"]
                        .into_iter()
                        .map(|t| view! {
                            <span class="font-mono text-xs text-ink border border-rule px-3 py-1 tracking-wide uppercase">
                                {t}
                            </span>
                        })
                        .collect::<Vec<_>>()}
                </Cluster>

                // ----- Numbered previews — accent numerals, measured bodies. -----
                <Stack gap={d.gutter()}>
                    {previews
                        .into_iter()
                        .map(|(n, title, body)| view! {
                            <article class="grid grid-cols-[auto_1fr] gap-6 border-b border-rule pb-6">
                                <span class="font-display text-4xl font-bold text-accent tabular-nums leading-none">{n}</span>
                                <Measure measure="60ch">
                                    <h4 class="gl-heading text-xl">{title}</h4>
                                    <p class="gl-prose mt-2">{body}</p>
                                </Measure>
                            </article>
                        })
                        .collect::<Vec<_>>()}
                </Stack>

                <Rule variant=RuleVariant::Thin label="Plates"/>

                // ----- Figures gallery — captioned editorial figures. -----
                <Grid gap={d.gutter()} max_width={d.max_width()}>
                    <Col cols_md={6}>
                        <Figure>
                            <FigureMedia slot:media_slot>
                                <div class="h-64 rounded-md bg-gradient-to-br from-ink to-ink-soft"></div>
                            </FigureMedia>
                            <FigureCaption slot:caption_slot>
                                "Plate I — Beethoven poster, Josef Müller-Brockmann, 1955. The grid as staff."
                            </FigureCaption>
                        </Figure>
                    </Col>
                    <Col cols_md={6}>
                        <Figure>
                            <FigureMedia slot:media_slot>
                                <div class="h-64 rounded-md bg-gradient-to-br from-paper-alt to-rule"></div>
                            </FigureMedia>
                            <FigureCaption slot:caption_slot>
                                "Plate II — Typographische Monatsblätter, a study in column measure."
                            </FigureCaption>
                        </Figure>
                    </Col>
                </Grid>

                // ----- Masonry photo grid — dense mosaic of plates. -----
                <Masonry cols={4} gap={d.gutter()}>
                    {photo_tiles
                        .into_iter()
                        .map(|(class_str, style_str, label)| view! {
                            <div class=class_str style=style_str>{label}</div>
                        })
                        .collect::<Vec<_>>()}
                </Masonry>

                <Rule variant=RuleVariant::Thin label="Voices"/>

                // ----- Voices — three pull-quotes from designers, asymmetric 7/5 grid. -----
                <Grid gap={d.gutter()} max_width={d.max_width()}>
                    <Col cols_md={7}>
                        <PullQuote class="text-ink">
                            <Quote slot:quote_slot>{voices[0].0}</Quote>
                            <Attribution slot:attribution_slot>{voices[0].1}</Attribution>
                        </PullQuote>
                    </Col>
                    <Col cols_md={5} start_md={8}>
                        <Stack gap={d.gutter()}>
                            <PullQuote class="text-ink">
                                <Quote slot:quote_slot>{voices[1].0}</Quote>
                                <Attribution slot:attribution_slot>{voices[1].1}</Attribution>
                            </PullQuote>
                            <PullQuote class="text-ink">
                                <Quote slot:quote_slot>{voices[2].0}</Quote>
                                <Attribution slot:attribution_slot>{voices[2].1}</Attribution>
                            </PullQuote>
                        </Stack>
                    </Col>
                </Grid>

                // ----- From the editor — MediaObject credit box. -----
                <MediaObject side={Side::Left} media_width="120px" gap={d.gutter()} class="py-8">
                    <Media slot:media_slot>
                        <div class="h-28 w-28 rounded-md bg-ink text-paper flex items-center justify-center font-display text-5xl font-bold">
                            "E"
                        </div>
                    </Media>
                    <Body slot:body_slot>
                        <p class="gl-eyebrow">"From the editor"</p>
                        <div class="gl-prose mt-2">
                            <p>"This issue began with a question: what survives when a page is \
                            reduced to its bones? The answer, again and again, is the grid. The \
                            essays collected here treat it not as a system to obey but as a \
                            language to write in."</p>
                        </div>
                        <p class="gl-meta mt-3">"E. R., Editor"</p>
                    </Body>
                </MediaObject>
            }
        }}

        // ----- Subscribe card — magazine subscription card near the end. -----
        <Card class="bg-paper-alt border-rule max-w-xl mx-auto">
            <CardHeader slot:header_slot>
                <p class="gl-eyebrow">"Subscribe"</p>
                <h3 class="gl-heading text-2xl mt-1">"Quadra, Issue 15 — September 2026"</h3>
            </CardHeader>
            <CardBody slot:body_slot>
                <p class="gl-prose">
                    "Four issues a year, printed in the browser. Each one a single subject, \
                    set in Inter, Source Serif 4, and JetBrains Mono. No advertising, no \
                    scroll-jacking — just pages that hold their shape."
                </p>
            </CardBody>
            <CardFooter slot:footer_slot>
                <div class="flex items-center justify-between">
                    <span class="gl-meta">"£32 / year"</span>
                    <span class="font-mono text-xs text-ink border border-rule px-3 py-1 uppercase tracking-wide hover:border-ink hover:text-accent transition-colors">
                        "Subscribe →"
                    </span>
                </div>
            </CardFooter>
        </Card>

        // ===== Closing colophon — thick rule, two meta lines spread across the width. =====
        <Rule variant=RuleVariant::Thick/>
        <Cluster gap={Gutter::Md} align={AlignItems::Baseline} justify={JustifyContent::Between} class="pb-8 px-4">
            <p class="gl-meta">"Set in Inter, Source Serif 4, and JetBrains Mono."</p>
            <p class="gl-meta">"© 2026 QUADRA · Printed in the browser."</p>
        </Cluster>
    }
}
// END Showcase