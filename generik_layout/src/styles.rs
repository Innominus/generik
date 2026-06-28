/// The single source of CSS truth for the crate.
///
/// All classes are namespaced with `gl-`. Components reference class names from
/// this string; they do not inline their own CSS (the per-`Col` responsive
/// rules in `grid.rs` are the only exception, because they are per-instance).
///
/// Override the design tokens on `:root` to theme the layout.
pub const LAYOUT_CSS: &str = r#"
:root {
  --gl-max-width: 1200px;
  --gl-gap-sm: 0.5rem;
  --gl-gap-md: 1.5rem;
  --gl-gap-lg: 2.5rem;
  --gl-pad: 1rem;
  --gl-sidebar-width: 280px;
  --gl-media-width: 160px;
}

/* ===== Scoped box-sizing reset (no global side effects) ===== */
[class^="gl-"], [class*=" gl-"] { box-sizing: border-box; }

/* ===== Gap utility classes (shared by grid/stack/cluster/sidebar/masonry) ===== */
.gl-gap-sm { gap: var(--gl-gap-sm); }
.gl-gap-md { gap: var(--gl-gap-md); }
.gl-gap-lg { gap: var(--gl-gap-lg); }

/* ===== Grid ===== */
.gl-grid {
  display: grid;
  grid-template-columns: repeat(12, 1fr);
  width: 100%;
  max-width: var(--gl-max-width);
  margin-inline: auto;
  padding-inline: var(--gl-pad);
}

/* ===== Col ===== */
.gl-col {
  grid-column: span 12;
}

/* ===== Stack ===== */
.gl-stack {
  display: flex;
  flex-direction: column;
}

/* ===== Cluster ===== */
.gl-cluster {
  display: flex;
  flex-wrap: wrap;
}

/* ===== Sidebar ===== */
/* Mobile-first: single column on small screens. */
.gl-sidebar {
  display: grid;
  grid-template-columns: 1fr;
  padding-inline: var(--gl-pad);
  margin-inline: auto;
  width: 100%;
  max-width: var(--gl-max-width);
}
@media (min-width: 768px) {
  .gl-sidebar {
    grid-template-columns: var(--gl-sidebar-width) 1fr;
  }
  .gl-sidebar-right {
    grid-template-columns: 1fr var(--gl-sidebar-width);
  }
}
/* gl-sidebar-main / gl-sidebar-side: positional grid children; intentionally
   unstyled. Exposed as consumer styling hooks (e.g. for padding, backgrounds). */

/* ===== Center ===== */
.gl-center {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 100%;
}

/* ===== Masonry ===== */
.gl-masonry {
  display: grid;
  grid-auto-flow: dense;
  grid-auto-rows: minmax(80px, auto);
  width: 100%;
  max-width: var(--gl-max-width);
  margin-inline: auto;
  padding-inline: var(--gl-pad);
}

/* ===== Figure ===== */
.gl-figure {
  margin: 0;
  display: flex;
  flex-direction: column;
  gap: var(--gl-gap-sm);
}
.gl-figure-media { margin: 0; }
.gl-figure-caption {
  font-size: 0.875rem;
  color: var(--gl-caption-color, #555);
  line-height: 1.4;
}

/* ===== MediaObject ===== */
.gl-media-object {
  display: grid;
  grid-template-columns: 1fr;
  width: 100%;
}
@media (min-width: 768px) {
  .gl-media-object {
    grid-template-columns: var(--gl-media-width, 160px) 1fr;
  }
  .gl-media-object-right {
    grid-template-columns: 1fr var(--gl-media-width, 160px);
  }
}
/* gl-media-media / gl-media-body: positional grid children; intentionally
   unstyled. Exposed as consumer styling hooks (e.g. for padding, backgrounds). */

/* ===== PullQuote ===== */
.gl-pullquote {
  margin: 0;
  padding: 0;
  border-left: 3px solid currentColor;
  padding-left: var(--gl-gap-md);
}
.gl-pullquote-text {
  font-size: 1.25rem;
  font-weight: 500;
  line-height: 1.4;
}
.gl-pullquote-attribution {
  margin-top: var(--gl-gap-sm);
  font-size: 0.875rem;
  color: var(--gl-caption-color, #555);
}

/* ===== Card ===== */
.gl-card {
  display: flex;
  flex-direction: column;
  border: 1px solid var(--gl-card-border, #e5e7eb);
  border-radius: 0.5rem;
  background: var(--gl-card-bg, #fff);
  overflow: hidden;
}
.gl-card-header {
  padding: var(--gl-gap-md);
  border-bottom: 1px solid var(--gl-card-border, #e5e7eb);
}
.gl-card-body {
  padding: var(--gl-gap-md);
  flex: 1 1 auto;
}
.gl-card-footer {
  padding: var(--gl-gap-md);
  border-top: 1px solid var(--gl-card-border, #e5e7eb);
}
"#;