//! Above-the-fold editorial sections — big Swiss layouts that make heavy use
//! of the 12-column grid on large screens. A portfolio of cover treatments.

use leptos::prelude::*;

use generik_layout::{
    Attribution, Center, Col, Container, Dropcap, Figure, FigureCaption, FigureMedia, Grid,
    Gutter, IndexItem, Measure, Metric, PullQuote, Quote, Rule, RuleVariant, Stack, Stat,
    StickyHeader, Teaser,
};

// BEGIN AboveTheFold
#[component]
pub fn AboveTheFold() -> impl IntoView {
    view! {
        <StickyHeader wordmark="QUADRA" nav=&["Features", "Archive", "Subscribe"] issue="Issue 14 — Jun 2026"/>
        <MagazineCover/>
        <ContentsPage/>
        <FeatureOpener/>
        <DataPoster/>
        <MoreFromThisIssue/>
        <PullQuoteHero/>
    }
}
// END AboveTheFold

// BEGIN MagazineCover
#[component]
fn MagazineCover() -> impl IntoView {
    view! {
        <section class="min-h-screen flex flex-col justify-between bg-paper py-16 md:py-20">
            <Container>
                <Grid gap={Gutter::Lg}>
                    <Col cols_md={7} start_md={1} class="flex flex-col justify-between">
                        <p class="gl-eyebrow">"Quarterly · No. 14"</p>
                        <div class="mt-auto">
                            <h1 class="gl-masthead-title text-7xl md:text-8xl lg:text-9xl">"QUADRA"</h1>
                            <p class="gl-meta mt-4">"A quarterly on the discipline of the grid"</p>
                        </div>
                    </Col>
                    <Col cols_md={5} start_md={8} class="flex flex-col justify-between gap-10">
                        <p class="gl-eyebrow text-right">"June 2026"</p>
                        <h2 class="gl-display text-3xl md:text-4xl">"The quiet geometry of Swiss design"</h2>
                        <Stack gap={Gutter::Md} class="mt-auto">
                            <Stat number="14" label="Issues published"/>
                            <Stat number="2019" label="Established"/>
                        </Stack>
                    </Col>
                </Grid>
            </Container>
            <Container class="mt-12">
                <Rule variant=RuleVariant::Thick/>
            </Container>
        </section>
    }
}
// END MagazineCover

// BEGIN ContentsPage
#[component]
fn ContentsPage() -> impl IntoView {
    let entries: [(&'static str, &'static str, &'static str, &'static str); 6] = [
        ("01", "Feature", "On the discipline of the grid", "p. 04"),
        ("02", "Essay", "Typography as architecture", "p. 12"),
        ("03", "Note", "The baseline as rhythm", "p. 18"),
        ("04", "Review", "Asymmetry and balance", "p. 22"),
        ("05", "Archive", "Müller-Brockmann at 100", "p. 28"),
        ("06", "Colophon", "Set in Inter, Source Serif, JetBrains Mono", "p. 32"),
    ];

    view! {
        <section class="gl-section">
            <Container>
                <Rule label="In this issue"/>
                <Grid gap={Gutter::Lg} class="mt-10">
                    <Col cols_md={3} start_md={1}>
                        <p class="gl-eyebrow">"Contents"</p>
                        <h2 class="gl-heading text-2xl md:text-3xl mt-3">"In this issue"</h2>
                        <p class="gl-meta mt-4">"Six pieces on the grid as a language for the page."</p>
                    </Col>
                    <Col cols_md={9} start_md={4}>
                        <Measure measure="75ch">
                            <Stack gap={Gutter::Md}>
                                {entries
                                    .into_iter()
                                    .map(|(n, cat, title, meta)| view! {
                                        <IndexItem number=n category=cat title=title meta=meta/>
                                    })
                                    .collect::<Vec<_>>()}
                            </Stack>
                        </Measure>
                    </Col>
                </Grid>
                <Rule variant=RuleVariant::Thin class="mt-12"/>
            </Container>
        </section>
    }
}
// END ContentsPage

// BEGIN FeatureOpener
#[component]
fn FeatureOpener() -> impl IntoView {
    view! {
        <section class="gl-section">
            <Container>
                <Rule variant=RuleVariant::Thin label="Feature 01"/>
                <Grid gap={Gutter::Lg} class="mt-12">
                    <Col cols_md={8} start_md={1}>
                        <p class="gl-eyebrow">"Feature"</p>
                        <h2 class="gl-heading text-4xl md:text-6xl mt-3">"On the discipline of the grid"</h2>
                        <p class="gl-lede mt-6">
                            "The grid is not a cage. It is the staff on which the music of the \
                            page is written — intervals, repetitions, rests."
                        </p>
                        <p class="gl-meta mt-6">"By the editors · 8 min read"</p>
                    </Col>
                    <Col cols_md={4} start_md={9}>
                        <Figure>
                            <FigureMedia slot:media_slot>
                                <div class="h-96 bg-gradient-to-br from-ink to-ink-soft"></div>
                            </FigureMedia>
                            <FigureCaption slot:caption_slot>
                                "Plate I — Beethoven poster, Josef Müller-Brockmann, 1955."
                            </FigureCaption>
                        </Figure>
                    </Col>
                </Grid>
                <Measure measure="65ch" class="mt-12">
                    <Dropcap>
                        <p class="gl-prose">
                            "The 12-column grid is less a constraint than a vocabulary. Within \
                            it, asymmetry becomes legible: an 8-column article beside a 4-column \
                            rail, a pull-quote offset by a start, a full-bleed image spanning all \
                            twelve. The grid gives the eye a rhythm it can predict, then breaks \
                            that rhythm deliberately."
                        </p>
                    </Dropcap>
                </Measure>
            </Container>
        </section>
    }
}
// END FeatureOpener

// BEGIN DataPoster
#[component]
fn DataPoster() -> impl IntoView {
    let metrics: [(&'static str, &'static str, Option<&'static str>); 4] = [
        ("14", "Issues published", Some("+1")),
        ("60+", "Essays archived", Some("+8")),
        ("1.2k", "Subscribers", Some("+12%")),
        ("2019", "Established", None),
    ];

    view! {
        <section class="gl-section">
            <Container>
                <Rule label="By the numbers"/>
                <Grid gap={Gutter::Lg} class="mt-16 py-12">
                    {metrics
                        .into_iter()
                        .map(|(value, label, delta)| {
                            let metric = match delta {
                                Some(d) => view! { <Metric value=value label=label delta=d/> }.into_any(),
                                None => view! { <Metric value=value label=label/> }.into_any(),
                            };
                            view! {
                                <Col cols_md={3}>{metric}</Col>
                            }
                        })
                        .collect::<Vec<_>>()}
                </Grid>
                <Rule variant=RuleVariant::Thin class="mt-16"/>
            </Container>
        </section>
    }
}
// END DataPoster

// BEGIN MoreFromThisIssue
#[component]
fn MoreFromThisIssue() -> impl IntoView {
    let teasers: [(&'static str, &'static str, &'static str, &'static str); 3] = [
        ("Archive", "Grid systems in print", "8 min · Essay", "bg-gradient-to-br from-ink to-ink-soft"),
        ("Note", "The baseline as rhythm", "5 min · Note", "bg-gradient-to-br from-paper-alt to-rule"),
        ("Review", "Asymmetry and balance", "7 min · Review", "bg-gradient-to-br from-rule to-ink-muted"),
    ];

    view! {
        <section class="gl-section">
            <Container>
                <Rule label="More from this issue"/>
                <Grid gap={Gutter::Lg} class="mt-10">
                    {teasers
                        .into_iter()
                        .map(|(eyebrow, title, meta, media)| view! {
                            <Col cols_md={4}>
                                <Teaser eyebrow=eyebrow title=title meta=meta media=media/>
                            </Col>
                        })
                        .collect::<Vec<_>>()}
                </Grid>
                <Rule variant=RuleVariant::Thin class="mt-12"/>
            </Container>
        </section>
    }
}
// END MoreFromThisIssue

// BEGIN PullQuoteHero
#[component]
fn PullQuoteHero() -> impl IntoView {
    view! {
        <section class="bg-ink text-paper py-32">
            <Center>
                <Container width="800px">
                    <PullQuote>
                        <Quote slot:quote_slot>
                            "The grid is the structure on which the work is built — and like a \
                            foundation, it should be felt, not seen. The empty spaces are not \
                            waste; they are the breath between phrases, the silence that lets \
                            the type sing."
                        </Quote>
                        <Attribution slot:attribution_slot>
                            "— Josef Müller-Brockmann, Grid Systems in Graphic Design"
                        </Attribution>
                    </PullQuote>
                </Container>
            </Center>
        </section>
        <Rule variant=RuleVariant::Thick/>
    }
}
// END PullQuoteHero