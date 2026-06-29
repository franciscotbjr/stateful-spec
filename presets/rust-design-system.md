# Project Definition — Rust Design System (toolkit-agnostic)

> Pre-filled preset for a **native Rust design-system crate** — the single source of tokens, themes, components, motion, and accessibility for a native UI. Distinct from `rust-library` (generic logic crate) and from `rust-gpui-app` (the application/composition layer that *consumes* this crate). It **inherits all generic crate hygiene from `rust-library`** (modules, errors, lints, toolchain, tracing, identity, CHANGELOG) — only the design-system deltas are restated below. Customize for your project; delete optional blocks that do not apply. **Toolkit-specific** mechanics are quarantined in `> **Toolkit-only:**` callouts — adapt or drop them for your framework.

---

## Project Identity

- **Project Name:** [your-design-system-name]
- **Description:** [One-sentence description, e.g., the token + component design system for the [app-name] app]
- **Project Type:** software
- **Category:** design system
- **Repository URL:** [https://github.com/user/repo]
- **License:** MIT OR Apache-2.0

## Technology Stack

### Language(s)

| Language | Version | Role |
|----------|---------|------|
| Rust | [edition, e.g., 2024] | Primary |

- **MSRV / toolchain pinning:** same as `rust-library` — declare `rust-version` and commit a root `rust-toolchain.toml`. The toolchain file is **required here**, not optional: a design system is consumed by apps, so divergent toolchains producing false-green `fmt`/`clippy` would propagate downstream.

### UI Framework

| Framework | Version | Purpose |
|-----------|---------|---------|
| [e.g., egui \| GPUI] | [git rev / e.g., 0.29] | Native UI toolkit the components render through |

> **Toolkit-only:** components render against one toolkit's primitives (retained-mode `GPUI` vs. immediate-mode `egui`). Tokens, theme model, icon catalog, contrast math, and the motion table stay **toolkit-agnostic**; only component `render` bodies bind to the toolkit. With GPUI this is a git dependency pinned by exact `rev`; with `egui` it is a normal versioned crate.

### Key Dependencies

| Dependency | Version | Purpose |
|------------|---------|---------|
| [e.g., palette \| custom] | [e.g., 0.7] | OKLCH color math for the semantic-state ramp |
| [e.g., serde] | [e.g., 1.0.228] | (De)serialize themes / persisted token overrides |
| [e.g., thiserror] | [e.g., 2.0.18] | Error derive (inherited convention) |
| [e.g., embedded font bytes] | — | Two font families embedded via `include_bytes!` |

### Build System & Package Manager

- **Package Manager / Build Tool:** cargo
- **Task Runner:** cargo (aliases / `just` / `make`, optional)

## Repository Structure

```
design-system/
├── Cargo.toml
├── rust-toolchain.toml         # committed at root (required)
├── clippy.toml                 # the anti-facade boundary (disallowed-methods)
├── assets/
│   └── fonts/                  # the two embedded font families
├── src/
│   ├── lib.rs                  # //! docs, prelude, apply_theme + tokens() accessor re-exports
│   ├── tokens/
│   │   ├── mod.rs              # facade
│   │   ├── canon.rs            # THE token canon: ramp → aliases → OKLCH semantic states
│   │   ├── density.rs          # Density enum → ~5 derived vars
│   │   └── color_mode.rs       # ColorMode enum → ~5 derived vars
│   ├── theme.rs                # Theme data model + apply_theme() + context-stashed tokens()
│   ├── typography.rs           # the two embedded families → type scale
│   ├── motion.rs               # centralized timing/easing table
│   ├── icon.rs                 # Icon: closed enum/catalog of inline shape paths (icon-as-shape)
│   ├── a11y/
│   │   ├── mod.rs
│   │   └── contrast.rs         # WCAG contrast math SHIPPED in the crate
│   ├── raw.rs                  # the ONE sanctioned escape hatch (#![allow(clippy::disallowed_methods)])
│   ├── widget.rs               # DsWidget helper trait + Widget / DsStatefulWidget<State> contracts
│   └── components/             # ONE FILE PER COMPONENT
│       ├── button.rs
│       ├── checkbox.rs
│       └── ...
├── tests/                      # render-without-panic + differential-geometry tests
├── examples/                   # optional gallery / storybook (gallery feature)
├── README.md
└── CHANGELOG.md
```

### Key Directories

| Directory | Purpose |
|-----------|---------|
| src/tokens/ | The token canon, `Density`/`ColorMode` enums and their derived vars |
| src/components/ | One file per component; each implements a `DsWidget` contract |
| src/a11y/ | Contrast math + accessibility helpers shipped in the crate |
| assets/fonts/ | The two embedded font families |
| examples/ | Optional companion gallery/storybook (feature `gallery`) |

## Code Conventions

### Naming

Standard Rust naming (same as `rust-library`): `snake_case` files/functions/modules, `PascalCase` types, `SCREAMING_SNAKE_CASE` constants. **One file per component**, named after the component type it holds (`button.rs` → `Button`).

### Code Style

- **Formatter:** rustfmt (default), 100-col, 4-space, import order std → external → crate → super/self.

#### Lints

Generic crate lints (`#![deny(missing_docs)]`, broken-intra-doc-links) are inherited from `rust-library`. The DS-specific lint is the **anti-facade boundary**:

- Components must consume style **only** through the design-system API (`tokens()` / the active `Theme`) — never reach around it to raw colors, hard-coded sizes, ad-hoc font handles, or toolkit color/font constructors. Enforce it via clippy `disallowed-methods` in `clippy.toml`:

  > **Toolkit-only:** the exact paths depend on the framework (GPUI builds color via `rgb()`/`Hsla`; egui uses `Color32` + `FontId`). Use placeholders for the real constructor names.

  ```toml
  # clippy.toml — components resolve every style through tokens()/Theme, never around it
  disallowed-methods = [
    { path = "[toolkit]::[raw-color-ctor]", reason = "use a semantic token, not a raw color" },
    { path = "[toolkit]::[raw-font-handle ctor]", reason = "use the typography scale, not a raw font" },
  ]
  ```

  **Exactly one sanctioned escape hatch:** the `raw::` module (`src/raw.rs`) is the **only** place that calls toolkit color/font constructors, and it carries `#![allow(clippy::disallowed_methods)]`. `tokens/canon.rs` builds the ramp by calling `raw::` helpers, so canon needs **no** allow. Anything else reaching around the API is a build failure.

### Patterns & Conventions

Generic crate patterns (module facades, prelude-as-public-API, central `Error`/`thiserror`, visibility discipline, `tracing`, identity, no-panic-on-input) are **inherited from `rust-library`** — not restated. Design-system deltas:

- **One token canon.** A single source resolves in one direction: **raw color ramp → theme aliases → OKLCH semantic states** (`default`/`hover`/`active`/`disabled`/…). Dark and light themes **share one primary**; a **runtime primary override** re-derives the semantic states from the same canon. No component defines its own colors.
- **Density & color mode as typed enums.** `Density` and `ColorMode` are closed `enum`s (never strings); each maps to roughly **five derived variables** (spacing unit, control height, font step, border width, focus-ring inset). The DS **owns** these enum types; the consuming app persists a chosen value (see `rust-gpui-app` settings) and passes it into `apply_theme` — it does not define a parallel density/mode enum.
- **Embedded two-font typography.** Exactly **two families** — [e.g., a UI/sans family + a monospace/code family] — embedded via `include_bytes!`, exposed as a fixed type scale; components reference scale steps, not font handles.
- **Icon-as-shape.** Icons are **shape paths in a closed `enum`/catalog** (`Icon::Check`, `Icon::Close`, …) authored inline in `icon.rs`, not arbitrary asset files — each variant returns vector geometry.
- **Component contracts via the builder pattern.** Two component kinds, both built with chainable `with_*(mut self, …) -> Self` builders that never panic:
  - a **stateless `Widget`** — pure render-from-props;
  - a **`DsStatefulWidget<State>`** — owns view-local *presentational* `State`, still no app/domain I/O;
  - both implement a shared **`DsWidget`** helper trait (token/theme access, focus & hit-target helpers, the standard render entry).

  A widget's view-local `State` (e.g. `hover`) *selects* which semantic token *state* (`hover`) it reads — the token ramp knows the colors; the widget knows which one applies.

  > **Boundary:** components are **presentational only** — no engine, `UiCommand`/`EngineEvent`, or domain state. That wiring belongs to the app (`rust-gpui-app`). This does not violate the app's render-from-state discipline: view-local `State` (hover/expanded/scroll-offset) is ephemera, not the domain snapshot the app reduces.
- **Centralized motion.** All timings/easings live in one `motion.rs` table; components reference named entries (`motion::PRESS`, `motion::FADE`), never inline durations. Honors a reduced-motion flag from the active theme.
- **Single theming entry point.** `apply_theme(theme, &mut ctx)` is the **one** entry that installs the active theme + derived tokens into the toolkit context; components read them back through a **context-stashed `tokens()`** accessor. `Theme` carries `ColorMode` + `Density` + an optional primary override (constructed via its builder); `apply_theme` derives the token set from those and stashes it. No component is handed a `Theme` directly.
  > **Toolkit-only:** the stash mechanism is `cx.set_global` / `cx.global` (GPUI) vs. a value threaded through the frame `Context` (egui). The `tokens()` signature abstracts over it.
- **Concurrency (override).** A DS crate is typically **UI-thread-bound and not `Send + Sync`** (it touches toolkit context). This **overrides** the `rust-library` `Send + Sync` rule — do **not** add `assert_send_sync` tests for context-bound types; pure data types (tokens, `Theme`, `Icon`) may still be `Send + Sync` and serializable.

### Feature Flags

| Feature | Default | Activates | Purpose |
|---------|---------|-----------|---------|
| [e.g., serde] | no | `dep:serde` | (De)serialize `Theme` / persisted token overrides |
| [e.g., gallery] | no | — | Build the companion gallery/storybook example |

## Testing

### Strategy

- **Render-without-panic tests** — one per component: build it via its builder against a default theme and **drive a render pass to completion without panicking**, across each `ColorMode` × `Density` combination, using [e.g., the toolkit's headless/test context] so a unit test needs no live window.
- **Differential-geometry tests** — assert computed geometry (control sizes, padding, hit-target boxes, icon path bounds) **changes as expected when `Density`/`ColorMode` change**, and stays stable otherwise. Catches a token that silently stopped feeding a component.
- **Accessibility tests** — assert every semantic foreground/background pairing meets the WCAG threshold using the **shipped** contrast math (including a deliberately-failing pairing asserted rejected); a focused state produces a visible focus-ring token; the reduced-motion flag collapses motion-token durations.
- **Test Framework:** cargo test [or `cargo nextest` in CI].
- **Coverage Target:** No formal target; cover every component (render-without-panic) and every token pairing (contrast).

### Conventions

- Assert resolved geometry / contrast ratios (behavior), not just that a value exists; token/`Theme` data types assert serde round-trips when `serde` is enabled (inherited test hygiene).

### Test Naming Convention

`test_{what}_{scenario}` — e.g., `test_button_renders_no_panic_compact_dark`, `test_checkbox_geometry_grows_with_density`, `test_contrast_primary_on_surface_passes_aa`.

## Quality Gates

> Inherits the `rust-library` gate floor. The **accessibility gate is a hard, automated addition** — contrast is shipped math, so it runs as a normal test, not a manual check.

```bash
# Formatter check
cargo fmt --all --check

# Linter — warnings are errors; includes the anti-facade disallowed-methods boundary
cargo clippy --all-features --all-targets -- -D warnings

# Tests — render-without-panic + differential-geometry + WCAG contrast
cargo test --all-features

# Build (default features only must also pass, per inherited rule)
cargo build --all-features

# Doc build
cargo doc --all-features --no-deps
```

## Documentation

### Required Documentation Files

| File | Purpose |
|------|---------|
| README.md | Overview, token model, how to `apply_theme` + read `tokens()`, component list |
| CHANGELOG.md | Version history (Keep a Changelog + SemVer) — icon/token additions are SemVer surface |
| ARCHITECTURE.md | (Recommended) The token-canon diagram: ramp → aliases → OKLCH states; the anti-facade boundary |
| LICENSE | License file |

### Documentation Style

- **Code Comments:** rustdoc (`///` public items, `//!` module headers); document each `Icon` variant and each semantic token.
- **Doc Examples:** prefer `no_run` for examples that need a toolkit context.
- **CHANGELOG:** Keep a Changelog; record new icons, tokens, and components as additive entries.

## Deployment / Distribution

> Ships as a **library crate**, but defaults to **in-repo path consumption** by its companion app rather than crates.io.

- **Packaging.** Default to `publish = false` with **path dependencies** from the consuming app — the design system co-evolves with the app and is not (yet) a public crate. To make it publishable: flip `publish = false → true`, then follow the `rust-library` publishing flow (crates.io metadata, internal path deps given both `path` and `version`, dependency-before-dependent order).
- **CI/CD:** [e.g., GitHub Actions] — the inherited gate job (`fmt` → `clippy` → `test` incl. contrast → `doc`), cross-OS. No binary-artifact workflow.
- **Companion gallery/storybook (optional).** A feature-gated example target (`[[example]] name = "gallery"`, `required-features = ["gallery"]`) — or a separate workspace member crate if it grows — renders every component across `ColorMode` × `Density` for visual review. A dev aid, never a shipped dependency.
- **Branch Strategy:** main + feature branches.

## Constraints & Non-Negotiables

- Components resolve **every** style through `tokens()` / the active `Theme` — never raw colors, sizes, fonts, or toolkit constructors (enforced by the anti-facade clippy boundary; **one** sanctioned `raw::` escape hatch).
- There is **one token canon** (ramp → aliases → OKLCH states); dark/light share one primary; the runtime primary override re-derives from it. No per-component colors.
- `Density` and `ColorMode` are typed **enums** owned by the DS, not strings; each drives the derived-variable set; the app passes a chosen value into `apply_theme`.
- Icons are **shapes in a closed catalog** authored inline, not asset files; adding one is a SemVer surface change.
- Components are **presentational only** — no engine, protocol, domain state, or I/O (that is the app's job; see `rust-gpui-app`).
- Theming flows through the single `apply_theme` entry + context-stashed `tokens()` accessor — components are never handed a `Theme` directly.
- WCAG contrast + accessibility (focus-ring, reduced-motion, geometry-covered hit-target) are **automated gates**; the contrast math is **shipped in the crate**.
- Every component has a **render-without-panic** test and **differential-geometry** coverage.
- This crate is typically **not `Send + Sync`** (UI-thread-bound) — this overrides the `rust-library` Send+Sync rule; pure data types may still be both.
- A committed root `rust-toolchain.toml` is **required**.
- No unsafe code without justification; never panic on external/user-controlled input (inherited).
