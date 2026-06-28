//! Swiss/editorial layout components for Leptos 0.8.
//!
//! A self-contained layout primitive library: responsive 12-column grid, stack,
//! cluster, sidebar, center, and masonry components. Ships its own CSS in one
//! `gl-`-namespaced string (no Tailwind required) and injects it once via the
//! `LayoutStyles` component.
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

pub mod components;
pub mod shared;
pub mod styles;

pub use components::*;
pub use shared::{merge_class, AlignItems, Gutter, JustifyContent, Side};
pub use styles::LAYOUT_CSS;