//! Silkenweb adapters for `generik_layout`.
//!
//! Framework-neutral CSS, tokens, value types, and responsive grid-column rule
//! generation live at the crate root (`col_css`, `shared`, `styles`) and are
//! available regardless of which framework feature is selected. This module
//! hosts the Silkenweb component adapters.
//!
//! The adapters cover the stylesheet injector, the box-style primitives
//! ([`container`], [`center`], [`measure`], [`stack`], [`cluster`]), the grid
//! family ([`grid`], [`col`], [`masonry`], [`sidebar`]), and the editorial
//! components ([`card`], [`figure`], [`media_object`], [`rule`], [`dropcap`],
//! [`metric`], [`stat`]). Each is parameterised over a generic
//! [`Dom`](silkenweb::dom::Dom) and reproduces the Leptos components' `gl-*`
//! class and inline-style semantics; most accept typed direct children and
//! most return a [`Div<D>`](silkenweb::elements::html::Div), while [`figure`]
//! returns a semantic [`Figure<D>`](silkenweb::elements::html::Figure).
//!
//! Call [`mount_styles`] once during app setup so the `gl-*` classes resolve:
//!
//! ```ignore
//! use generik_layout::shared::Gutter;
//! use generik_layout::silkenweb::{col, grid, mount_styles};
//! use silkenweb::elements::html::div;
//!
//! // during app setup:
//! mount_styles();
//! let half = col(6, None, None, None, None, None, None, None, None, None, None, None, "", "", [div()]);
//! let page = grid(Gutter::Md, "", "", "", [half]);
//! ```

mod card;
mod dropcap;
mod figure;
mod grid;
mod helpers;
mod index_item;
mod masonry;
mod masthead;
mod media_object;
mod metric;
mod primitives;
mod pull_quote;
mod rule;
mod sidebar;
mod stat;
mod sticky_header;
mod stylesheet;
mod teaser;

pub use card::card;
pub use dropcap::dropcap;
pub use figure::figure;
pub use grid::{col, grid};
pub use index_item::index_item;
pub use masonry::masonry;
pub use masthead::masthead;
pub use media_object::media_object;
pub use metric::metric;
pub use primitives::{center, cluster, container, measure, stack};
pub use pull_quote::pull_quote;
pub use rule::rule;
pub use sidebar::sidebar;
pub use stat::stat;
pub use sticky_header::sticky_header;
pub use stylesheet::mount_styles;
pub use teaser::teaser;
