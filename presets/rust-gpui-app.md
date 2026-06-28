# Project Definition — Rust GPU UI Application (GPUI / egui)

> Pre-filled preset for a Rust desktop/UI **application** built on a GPU-accelerated native UI framework — retained-mode (e.g. GPUI) or immediate-mode (e.g. egui/eframe). Distinct from `rust-library` (a published crate): this is a shipped **binary** with a render-from-state architecture (the app's own discipline, layered on either UI mode). Customize for your project; delete optional blocks that do not apply. **GPUI-only** callouts mark mechanics specific to GPUI — adapt or drop them for another framework. For the underlying crate-authoring conventions (modules, errors, secrets, identity), the `rust-library` preset still applies.

---

## Project Identity

- **Project Name:** [your-app-name]
- **Description:** [One-sentence description]
- **Project Type:** application (desktop GUI)
- **Repository URL:** [https://github.com/user/repo]
- **License:** MIT OR Apache-2.0

## Technology Stack

### Language(s)

| Language | Version | Role |
|----------|---------|------|
| Rust | [edition, e.g., 2024] | Primary |

- **MSRV (rust-version):** [e.g., 1.96] — declare it in `Cargo.toml`.
- **Toolchain pinning:** commit a `rust-toolchain.toml` at the repo root pinning the channel + the components every gate needs, so divergent local toolchains can't produce false-green `fmt`/`clippy`:

  ```toml
  [toolchain]
  channel = "[e.g., 1.96.0]"
  components = ["rustfmt", "clippy"]
  ```

### UI Framework

| Framework | Version | Purpose |
|-----------|---------|---------|
| [e.g., GPUI \| egui/eframe] | [git rev / e.g., 0.29] | GPU-accelerated UI toolkit (retained- or immediate-mode) |

> **GPUI-only:** GPUI has no crates.io release — it is a **git dependency** pinned by exact `rev`/`tag`. With `egui/eframe` it is a normal versioned crate.

### Key Dependencies

| Dependency | Version | Purpose |
|------------|---------|---------|
| [e.g., tokio] | [e.g., 1.52.3] | Async runtime for the engine (not the UI executor) |
| [e.g., serde] | [e.g., 1.0.228] | Settings (de)serialization |
| [e.g., thiserror] | [e.g., 2.0.18] | Error derive macros |
| [e.g., tracing] | [e.g., 0.1.44] | Structured instrumentation |
| [e.g., your-design-system] | [path/git] | Theming tokens → theme (see `rust-design-system` preset) |

### Build System & Package Manager

- **Package Manager:** cargo
- **Build Tool:** cargo
- **Task Runner:** cargo (via aliases / `just` / `make`, optional)

### Cargo.lock & pre-1.0 git dependencies

- **Commit `Cargo.lock`** — this is a binary/app, so the lockfile is part of the source of truth and makes builds reproducible across machines and CI.
- Pin pre-1.0 / git dependencies (notably the UI framework) by an **exact `rev`/`tag`**, never a floating branch — a retained-mode UI toolkit moves fast and a silent bump breaks rendering.

## Repository Structure

```
project/
├── Cargo.toml
├── Cargo.lock              # committed (binary/app)
├── rust-toolchain.toml
├── clippy.toml             # the "async off the UI executor" boundary (disallowed-methods)
├── src/
│   ├── main.rs             # entry: build runtime, spawn engine, open window
│   ├── app/                # UI layer: views/components — render-from-state only
│   ├── engine/             # GUI-agnostic async engine: owns state + all side effects (I/O, net)
│   ├── protocol.rs         # UiCommand (UI → engine) / EngineEvent (engine → UI) message types
│   ├── state.rs            # the snapshot the UI renders + the pure `reduce()` function
│   ├── settings.rs         # serde-enum persisted settings
│   └── i18n/               # Lang enum + message catalogs
├── tests/                  # headless engine/reducer + geometry tests
├── assets/                 # fonts, icons, themes (if embedded)
├── README.md
└── CHANGELOG.md
```

### Key Directories

| Directory | Purpose |
|-----------|---------|
| src/app/ | UI layer — views/components, render-from-state only |
| src/engine/ | GUI-agnostic async engine — owns state and side effects |
| tests/ | Headless engine/reducer and geometry tests (no window) |
| assets/ | Embedded fonts, icons, theme files |

## Code Conventions

### Naming

Standard Rust naming (same as the `rust-library` preset): `snake_case` files/functions/modules, `PascalCase` types, `SCREAMING_SNAKE_CASE` constants.

### Code Style

- **Formatter:** rustfmt (default settings)
- **Max Line Length:** 100 (rustfmt default)
- **Indentation:** 4 spaces
- **Import Order:** std → external crates → crate internal → super/self

#### Lints

- Crate root carries `#![deny(rustdoc::broken_intra_doc_links)]`; an app may relax `missing_docs` relative to a library, but keep `-D warnings` on clippy.
- **Keep async/blocking work off the UI executor.** Enforce it as a lint, not a convention, via clippy `disallowed-methods` in `clippy.toml`:

  ```toml
  # clippy.toml — async/blocking work belongs to the engine runtime, not the UI executor
  disallowed-methods = [
    { path = "tokio::spawn", reason = "hand work to the engine; do not spawn on the UI executor" },
    { path = "tokio::task::spawn_blocking", reason = "run blocking work in the engine" },
    { path = "std::thread::sleep", reason = "never block the UI thread" },
  ]
  ```

  The engine still spawns freely — through an explicit runtime `Handle` (`handle.spawn(…)`), which the lint does **not** touch. Only the **ambient free-function** forms (`tokio::spawn` and friends) are blocked, and those are exactly what careless UI code reaches for.

  > **GPUI-only:** GPUI's foreground executor is **not** tokio, and a view has no ambient tokio runtime. Calling `tokio::spawn` from a view runs on the wrong runtime (or panics); route the intent to the engine instead. The lint above makes the mistake a build failure.

### Patterns & Conventions

- **Render-from-state architecture.** The UI is a pure function of an immutable **state snapshot** — views never mutate domain state or perform I/O. A **GUI-agnostic async engine** owns the canonical state and every side effect (I/O, network, long-running work). The two layers talk over channels: the UI sends **`UiCommand`** intents to the engine; the engine emits **`EngineEvent`** deltas/notifications back. The UI applies events to its snapshot and re-renders.
- **Pure reducer.** State transitions live in a single pure, synchronous `reduce(state, event) -> state` with no I/O and no framework types, so the entire core is **headless-testable** without opening a window.
- **The async boundary.** All async/blocking work runs on the engine's runtime (e.g. tokio); the UI executor only renders. (See the clippy boundary above.)
- **Cheap snapshots / large lists.** Render from clone-cheap snapshots — share immutable collections as `Arc<[T]>` so a re-render clones a pointer, not the data. **Virtualize** long lists (render only the visible row window).
- **Theming via a design system.** Never hard-code colors/spacing; consume a design-system crate (tokens → active theme) and resolve every style from the theme. (Companion preset: `rust-design-system`.)
- **Settings as serde enums.** Persist user settings as serde structs; model closed choices (theme mode, density, language) as `#[serde(rename_all = "…")]` **enums**, not strings — an invalid persisted value then fails to deserialize instead of silently mis-rendering.
- **i18n through a catalog.** All user-facing strings resolve through a message catalog keyed by a **mandatory `Lang` enum** — no bare string literals in views. A missing translation falls back to a typed default, never panics.
- **Shared Rust hygiene** (same as `rust-library`): instrument with `tracing` (never `println!`); never panic on external/user input; derive self-reported name/version from `env!("CARGO_PKG_*")`; hand-write masking `Debug` for any secret-bearing type.

### Feature Flags

> Optional — applies if the app has optional functionality (e.g. a dev-only inspector).

| Feature | Default | Purpose |
|---------|---------|---------|
| [e.g., dev-tools] | no | [Dev-only inspector / capture UI] |

## Testing

> An application is **not** fully verified by `cargo test` alone — it has a rendered surface and an end-to-end behavior `cargo test` can't see. Use a **3-gate verification model**.

### Verification gates

1. **Technical gate** — the standard quality gates below all green (`fmt`, `clippy` incl. the `disallowed-methods` boundary, `test`, `build --release`).
2. **Visual gate** — a human (or a screenshot/snapshot check) confirms the rendered UI is correct: layout, theme, density, and localized strings across the key screens.
3. **Functional gate** — the app does the thing end-to-end against the **real** engine (not fixtures), exercised by a manual smoke run before shipping.

### Strategy

- **Headless engine/reducer tests** — the bulk of coverage: drive `UiCommand → reduce → EngineEvent` with no window and assert the state transitions.
- **Headless geometry tests** — assert computed layout/measurements (sizes, positions, the virtualized row window) without rendering pixels.
- **Smoke test** — launch the app, render the main window, exit cleanly; catches init/wiring panics the unit tests miss.
- **Test Framework:** cargo test [or `cargo nextest` in CI]
- **Coverage Target:** No formal target; concentrate coverage in the headless core.

### Conventions

- **Assert behavior, not existence** — assert the state change or emitted `EngineEvent`, not just `is_ok()`.
- **Fixtures gated, never shipped.** Capture/replay fixtures may exist behind a dev-only flag (e.g. a `--capture` switch); mock/fixture data paths must be **unreachable in a release build**.

### Test Naming Convention

`test_{what_is_being_tested}_{scenario}` — e.g., `test_reduce_select_item_updates_focus`, `test_list_geometry_virtualizes_offscreen_rows`.

## Quality Gates

> The technical gate (Gate 1). The visual + functional gates are manual and run before shipping. Run on each target OS.

```bash
# Formatter check
cargo fmt --all --check

# Linter — warnings are errors; includes the disallowed-methods UI-executor boundary
cargo clippy --all-targets -- -D warnings

# Tests (headless engine/reducer + geometry + smoke)
cargo test

# Release build (the shipped artifact)
cargo build --release
```

## Documentation

### Required Documentation Files

| File | Purpose |
|------|---------|
| README.md | Overview, build/run, supported platforms |
| CHANGELOG.md | Version history (Keep a Changelog + SemVer) |
| ARCHITECTURE.md | (Recommended) The render-from-state map: engine ↔ UI channels, `UiCommand`/`EngineEvent`, reducer |
| LICENSE | License file |

### Documentation Style

- **Code Comments:** rustdoc (`///` public items, `//` internal, `//!` module headers).
- **CHANGELOG:** Keep a Changelog format with a top `[Unreleased]` section and dated `## [x.y.z] — YYYY-MM-DD` headers.

## Distribution / Deployment

> Ships as a **binary**, not a crate — there is no crates.io publish.

- **Release profile.** Tune `[profile.release]` for a shipped binary:

  ```toml
  [profile.release]
  lto = "thin"          # or true for max optimization
  codegen-units = 1
  strip = true
  # panic = "abort"     # optional — only if no code path relies on unwinding
  ```

- **Reproducible builds.** `Cargo.lock` is committed and pre-1.0 git deps are pinned by exact `rev` (see Technology Stack).
- **Per-OS, human-gated binaries CI.** A release workflow builds the binary on each target OS (Linux / macOS / Windows), typically on a version tag (`v*`); the technical gate (`fmt`/`clippy`/`test`) runs first on a cheap matrix. The actual artifact **publish/release is a human-gated action**, not automatic on green.
- **CI/CD:** [e.g., GitHub Actions]
- **Branch Strategy:** main + feature branches.

## Constraints & Non-Negotiables

- The UI is **render-from-state only** — views never mutate domain state or perform I/O; all intent flows to the engine as `UiCommand`, all change flows back as `EngineEvent`.
- **No async/blocking work on the UI executor** (GPUI-only: never `tokio::spawn` from a view) — enforced by clippy `disallowed-methods`.
- The engine core (reducer + protocol) is **headless-testable** with no window dependency.
- All user-facing strings go through the i18n catalog (mandatory `Lang`); closed settings are serde **enums**, not strings.
- **Never ship mock/fixture data in a release build** — capture/replay is dev/test-only.
- `Cargo.lock` is committed; pre-1.0 / git deps are pinned by exact `rev`.
- Colors and spacing come from the design-system tokens — never hard-coded.
- No unsafe code without justification and documentation; never panic on external/user-controlled input.
