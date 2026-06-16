use leptos::prelude::*;
use leptos_meta::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SeoConfig {
    pub title: &'static str,
    pub description: &'static str,
    pub canonical: &'static str,
    pub og_site_name: &'static str,
    pub og_title: &'static str,
    pub og_description: &'static str,
    pub og_type: &'static str,
    pub og_url: &'static str,
    pub og_image: &'static str,
    pub og_image_alt: &'static str,
    pub twitter_card: &'static str,
    pub twitter_title: &'static str,
    pub twitter_description: &'static str,
    pub twitter_image: &'static str,
}

#[component]
pub fn RouteMetaTags() -> impl IntoView {
    let seo = expect_context::<RwSignal<SeoConfig>>();

    view! {
        {move || {
            let seo = seo.get();

            view! {
                <Title text=seo.title />
                <Meta name="description" content=seo.description />
                <Meta name="robots" content="index,follow" />
                <Link rel="canonical" href=seo.canonical />
                <Meta property="og:site_name" content=seo.og_site_name />
                <Meta property="og:title" content=seo.og_title />
                <Meta property="og:description" content=seo.og_description />
                <Meta property="og:type" content=seo.og_type />
                <Meta property="og:url" content=seo.og_url />
                <Meta property="og:image" content=seo.og_image />
                <Meta property="og:image:alt" content=seo.og_image_alt />
                <Meta name="twitter:card" content=seo.twitter_card />
                <Meta name="twitter:title" content=seo.twitter_title />
                <Meta name="twitter:description" content=seo.twitter_description />
                <Meta name="twitter:image" content=seo.twitter_image />
            }
        }}
    }
}
