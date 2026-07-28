//! Swiss/editorial layout components for Leptos 0.8 and Silkenweb.
//!
//! A self-contained layout primitive library: responsive 12-column grid, stack,
//! cluster, sidebar, center, and masonry components. It ships one global
//! `gl-`-namespaced CSS contract (no Tailwind required), injected with Leptos
//! [`LayoutStyles`](components::LayoutStyles) or Silkenweb [`silkenweb::mount_styles`].
//!
//! # Usage
//!
//! ```ignore
//! use generik_layout::{LayoutStyles, Grid, Col, Stack};
//! use leptos::prelude::*;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     view! {
//!         <LayoutStyles/>
//!         <Grid>
//!             <Col cols_md=6>"Main"</Col>
//!             <Col cols_md=6>"Aside"</Col>
//!         </Grid>
//!     }
//! }
//! ```

mod col_css;
pub mod shared;
pub mod styles;

#[cfg(feature = "leptos")]
pub mod components;

#[cfg(feature = "silkenweb")]
pub mod silkenweb;

#[cfg(feature = "leptos")]
pub use components::*;
pub use shared::{merge_class, AlignItems, Gutter, JustifyContent, RuleVariant, Side};
pub use styles::LAYOUT_CSS;
