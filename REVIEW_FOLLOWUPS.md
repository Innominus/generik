# Review followups — 2026-06-28 (Round 3: 5-reviewer pass)

## Reviewer 1 (proc-macro + build config) — 10 findings

### Accepted (fix now)
- P1-F1 | medium | docs | `lib.rs:32-37` | The `cfg!(debug_assertions)` comment mis-describes the mechanism (says "consumer cfg context"; actually baked into proc-macro at its build time, tracks consumer profile only because cargo builds proc-macros with same profile). Reword. (Self-fix.)
- P1-F3 | medium | build | `[profile.release]` applies size-opts to proc-macro host build (pure DX cost). Add `[profile.release.package.generik_css_macros] opt-level=0 lto=false codegen-units=16`. (Self-fix.)
- P1-F4 | low | docs | `tests/macro_expansion.rs:6` module doc says "branch on cfg!(debug_assertions)" but tests branch on observed output. Fix the line. (Self-fix.)
- P1-F5 | low | correctness | Silent fall-back on lightningcss error. Add `eprintln!` when helper returns None in release. (Self-fix.)

### Accepted (defer)
- P1-F2 | medium | build | `generik_layout` configured for publication but depends on `publish=false` `generik_css_macros` (path dep, no version) → unpublishable. Decide intent when publishing; not blocking now.
- P1-F6 | low | lightningcss alpha pinning (`=1.0.0-alpha.71`). Lockfile mitigates; defer.
- P1-F7 | low | `proc-macro2` explicit dep unused in source. Harmless; defer.
- P1-F8 | low | No `/*! license */` / `calc()` / nesting test. Defer.
- P1-F9 | low | Helper doc contract incomplete. Defer.
- P1-F10 | low | Hard dep on `generik_css_macros` no opt-out. Defer until external consumers need it.

## Reviewer 2 (library core: grid/styles/shared/lib) — 11 findings

### Accepted (fix now)
- P2-F1 | HIGH | correctness | `metric.rs:24-29` `style=color` where `color="var(--gl-accent)"` is NOT a valid CSS declaration (no `property: value` pair) → browser discards → delta color never renders. **Two reviewers independently found this (P4-F1 below is the same).** Fix: `style=("color", color)` tuple form. (Self-fix.)
- P2-F4 | medium | correctness | `all_default` predicate duplicated verbatim between `Col` (108-119) and `compute_col_css` (194-197) → drift risk → panic. Extract `ColConfig::is_all_default()`. (Builder — grid.rs.)
- P2-F8 | low | correctness | `COL_CSS_CACHE.lock().expect("poisoned")` panics on poison. Recover: `.unwrap_or_else(|e| e.into_inner())`. (Self-fix.)

### Accepted (defer)
- P2-F2 | medium | test | No direct unit test for ColConfig cache. Defer — would need extracting cache logic.
- P2-F3 | medium | test | No test for clamping (cols=0→1). Defer — clamp is in Col, not pure fn.
- P2-F5 | low | correctness | `Col` clamps lower (>=1) but not upper (cols=20 overflows 12-col grid). Defer — document, don't clamp (Tailwind doesn't either).
- P2-F6 | low | idiomatic | 4 manual `impl Default` on enums (clippy derivable_impls). Re-flag of prior F14. Defer.
- P2-F7 | low | idiomatic | `compute_col_css` 13 params (clippy too_many_arguments). Accepted tradeoff; defer.
- P2-F9 | low | perf | `format!("gl-col-{id}")` on every cache hit. Negligible; defer.
- P2-F10 | low | test | No direct BREAKPOINTS value test. Defer.
- P2-F11 | low | test | `merge_class("", "")` untested. Defer.

## Reviewer 3 (original 10 components) — 7 findings

### Accepted (fix now)
- P3-F2 | medium | design | `PullQuote` `border-left: 3px solid currentColor` relies on consumer's ambient color, undocumented. Change to `border-left: 3px solid var(--gl-accent, currentColor)` for sensible default + still overridable. (Self-fix — styles.rs.)

### Accepted (defer)
- P3-F1 | medium | correctness | `LayoutStylesMarker` `Send+Sync` bound implicit. Auto-satisfied by ZST today. Defer — add compile-time assertion only if marker gains fields.
- P3-F3 | low | perf | `Sidebar`/`MediaObject` `base = "gl-...".to_string()` intermediate allocations. Negligible; defer.
- P3-F4 | low | perf | `format!("gl-gap{}", gap.class_suffix())` allocates per render. Negligible; defer.
- P3-F5 | low | docs | `Center` doc grammar ("both axes" → "on both axes"). Defer.
- P3-F6 | low | idiomatic | Duplicated `FnOnce`-slot comment across 3 files. Defer.
- P3-F7 | low | idiomatic | `RuleVariant` double re-export (rule.rs pub use + shared.rs). Already addressed in lib.rs (R2-F4); defer further cleanup.

## Reviewer 4 (editorial + 4 new components) — 8 findings

### Accepted (fix now)
- P4-F1 | critical | correctness | `metric.rs:29` `style=color` invalid CSS → delta color never renders. SAME AS P2-F1. Fix: `style=("color", color)`. (Self-fix.)
- P4-F2 | medium | idiomatic | `rule.rs:26` `let line = move || view!{...}` needless closure, invoked once. Inline. (Self-fix.)
- P4-F3 | medium | api-misuse | `Metric` delta `+`-detection is a heuristic; docstring shows `−` (U+2212) but code only checks ASCII `+`. Document precisely OR extract a `delta_kind` helper. Defer to a polish pass — the `+`-only behavior is the intended one; just fix the docstring to say ASCII `+`. (Self-fix — docstring.)

### Accepted (defer)
- P4-F4 | medium | design | `Stat.number` vs `Metric.value` naming inconsistency. Defer — pre-1.0 but breaking; decide in an API pass.
- P4-F5 | low | correctness | `Container width="0"` footgun. Defer — document, don't validate.
- P4-F6 | low | idiomatic | `merge_style` logic duplicated 3x (Container/Measure/Grid). Defer — extract when 4th consumer appears.
- P4-F7 | low | design | `Rule` renders `<div>` not `<hr>` for a11y. Defer — a11y pass.
- P4-F8 | low | test | No test for `delta_color` logic. Defer.

## Reviewer 5 (examples + showcase + ATF) — 14 findings

### Accepted (fix now)
- P5-F1 | critical | correctness | `Teaser` component never exercised anywhere in the demo. Add a "More from this issue" 3-up Teaser row to the ATF. (Builder — atf.rs.)
- P5-F2 | high | correctness | `Showcase` full-bleed hero/ink sections are constrained inside `<main class="max-w-6xl">` — the Swiss-poster edge-to-edge effect is lost. Move `<ShowcaseSection/>` OUTSIDE `<main>` (full-width, like `<AboveTheFold/>`). (Self-fix — app.rs.)
- P5-F3 | high | correctness | `voices[0]`/`[1]`/`[2]` direct indexing inside reactive closure — theoretical panic, real inconsistency. Add a comment noting the length constraint. (Self-fix — showcase.rs.)
- P5-F9 | medium | correctness | `code_viewer.rs` `on_load` highlight no-ops if highlight.js CDN hasn't loaded; no retry. Add a queue in index.html. (Self-fix — index.html.)
- P5-F10 | medium | correctness | `extract_region` `str::find` false-match risk. Newline-bound the markers (`\n// BEGIN {name}\n`). (Self-fix — code_viewer.rs.)

### Accepted (defer)
- P5-F4 | medium | correctness | `StickyHeader` `nav=&[...]` static promotion — confirmed correct, no fix.
- P5-F5 | medium | correctness | `MagazineCover` `min-h-screen` + sticky header visual — confirmed OK.
- P5-F6 | medium | visual | `MagazineCover` wordmark anchoring (mt-auto in Col, not viewport). Defer — design review.
- P5-F7 | medium | perf | `.to_owned()` allocations in control renders. Re-flag of prior F14. Defer.
- P5-F8 | medium | visual | `ClusterExample` `rounded-full` pills vs showcase square pills. Defer — could be intentional.
- P5-F11 | low | visual | `Section` card `shadow-sm` vs flat editorial. Defer.
- P5-F12 | low | visual | Control helpers use `neutral-*` palette vs warm `ink/paper/rule`. Defer.
- P5-F13 | low | correctness | FOUC `<style>` only covers body. Confirmed adequate.
- P5-F14 | low | docs | `atf.rs` module doc counts StickyHeader as a section. Trivial; defer.

## Summary of fixes to apply
**Self-fixes (trivial, I'll do them):**
1. metric.rs — `style=("color", color)` tuple form (P2-F1/P4-F1, critical)
2. app.rs — move `<ShowcaseSection/>` outside `<main>` (P5-F2, high)
3. showcase.rs — comment on `voices` length constraint (P5-F3)
4. rule.rs — inline the `let line = move ||` closure (P4-F2)
5. styles.rs — PullQuote border `var(--gl-accent, currentColor)` (P3-F2)
6. code_viewer.rs — newline-bound markers (P5-F10)
7. index.html — highlight.js queue/retry (P5-F9)
8. grid.rs — cache poison recovery `.unwrap_or_else(|e| e.into_inner())` (P2-F8)
9. Cargo.toml — `[profile.release.package.generik_css_macros]` override (P1-F3)
10. generik_css_macros/src/lib.rs — reword cfg comment + add eprintln fall-back (P1-F1, P1-F5)
11. generik_css_macros/tests/macro_expansion.rs — fix module doc line (P1-F4)
12. metric.rs — fix docstring to say ASCII `+` (P4-F3)

**Builder dispatches (substantive):**
13. atf.rs — add Teaser "More from this issue" 3-up row (P5-F1, critical)
14. grid.rs — extract `ColConfig::is_all_default()` to fix duplication (P2-F4, medium)