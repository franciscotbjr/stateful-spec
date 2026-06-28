# Backlog

> Triaged **opportunities** — work beyond the current milestone's contract. The canonical format is
> defined in `methodology/backlog.md` (at the repo root — this repo IS the methodology source). This
> file (`.stateful-spec/backlog.md`) is the project's instance: one row per opportunity, `O-NNN`
> numbering owned here (sequential, stable, never reused).
>
> A real defect against the current milestone's spec does **not** belong here — it follows the
> correction path (`request-changes` / reopen). Only durable opportunities are recorded.

<!-- O-NNN allocated at triage. States: new → triaged → promoted | discarded. -->

| ID | Description | Origin | Status | Destination |
|------|-------------|--------|--------|-------------|
| O-001 | The repo is the Stateful Spec methodology source yet had not adopted the structures it ships — its own `intake/` inbox + `backlog.md`, documented `RAW_HISTORY`, and an up-to-date `project-definition.md` structure. Make the repo practice its own methodology. | iteration 012/013 review (self-adoption audit) | promoted | Iteration 014 — self-adoption |
| O-002 | Publish the reference crate `stateful-spec-flow` (`packages/flow-rs/`) to crates.io: update `Cargo.toml` with the metadata required for publishing (description, license, repository, keywords, categories, etc.) and prepare the release. | intake/Backlog/prd.md (idea) | promoted | Iteration 015 — publish-flow-rs-crates-io |
| O-003 | Publish the Node/TS reference package `@stateful-spec/flow` (`packages/flow-ts/`) to npm — the npm counterpart to O-002's crates.io release: update `package.json` with the publishing metadata (description, license, repository, keywords, files, exports, etc.) and prepare the release. (Intake item titled "flow-rs to npm"; flow-rs → crates.io is done, so npm targets the TS twin.) | intake/Backlog/prd.md (idea) | promoted | Iteration 016 — publish-flow-ts-npm |
| O-004 | Validate the published references now live in their registries — confirm `stateful-spec-flow` v0.1.0 on crates.io (O-002) and `@stateful-spec/flow` v0.1.0 on npm (O-003) install, resolve, and that every place the repo points at them (docs, `packages/` READMEs, `project-definition.md`) cites the correct published name/version and works as advertised. | intake/Backlog/prd.md (idea) | promoted | Iteration 017 — validate-published-references |
| O-005 | Apply the external "new - 03 - preset rust" product-requirements document (located at an external path in the developer's working area) — a Rust preset for the methodology. The intake item carries a curator persona + exhaustive-questioning protocol; the concrete requirements live in the external PRD and must be read and shaped during planning before implementation. | intake/Backlog/prd.md (idea) | promoted | Iteration 018 — apply-preset-rust |
| O-006 | Add a `rust-gpui-app` preset (`presets/rust-gpui-app.md`) — a pre-filled Project Definition for a Rust retained-mode/GPU desktop/UI application (GPUI or egui/eframe), distinct from `rust-library`: render-from-state architecture (GUI-agnostic async engine + `UiCommand`/`EngineEvent` channels + a pure headless-testable reducer), the "UI-executor-is-not-tokio" async boundary enforced via clippy `disallowed-methods`, a 3-gate verification model (technical/visual/functional) with smoke + headless-geometry tests, theming-via-design-system composition, serde-enum settings + mandatory-`Lang` i18n, virtualized lists / `Arc<[T]>` snapshot caching, committed-`Cargo.lock` pinning of pre-1.0 git deps, a release-tuned profile, and a per-OS human-gated binaries CI. Authoring: mirror `templates/project/project-definition.md` as a terse preset (not an architecture essay); generalize stand-in/MCP/GPUI evidence to framework-neutral guidance with clearly-marked GPUI-only callouts; reduce the `--capture` CLI spec to a one-line "fixtures gated, never ship mocks" constraint; add a row to the presets index. Documentation-only. | iteration 018 (stand-in analysis, O-005/PRD) | new | — |
| O-007 | Add a toolkit-agnostic `rust-design-system` preset (`presets/rust-design-system.md`; Project Type: software, Category: design system) — a pre-filled Project Definition for a native Rust design-system crate, distinct from `rust-library` (logic crate) and from `rust-gpui-app` (composition layer): a single token canon (raw ramp → theme aliases → OKLCH semantic states, dark/light sharing one primary, runtime primary override), density + color-mode as typed enums driving five variables, embedded two-font typography, an icon-as-shape closed catalog, builder component contracts (stateless `Widget` vs `DsStatefulWidget<State>` + a shared `DsWidget` helper), one-file-per-component structure, centralized motion timings, a single `apply_theme` + context-stashed tokens accessor, the anti-facade clippy boundary with one sanctioned escape hatch, WCAG contrast + accessibility gates with shipped contrast math, per-component render-without-panic + differential geometry tests, `publish = false` path-dep packaging (with a documented publishable switch), a committed root `rust-toolchain.toml`, and an optional companion gallery/storybook. Authoring: mirror `templates/project/project-definition.md`; use bracketed `[e.g., egui \| GPUI]` placeholders; add "design system" as a free-text Category example; reuse the standard software operations (no DS-specific ops); add a presets-index row. Documentation-only. | iteration 018 (stand-in analysis, O-005/PRD) | new | — |

<!--
States:
- new       — promoted into the backlog, not yet evaluated.
- triaged   — evaluated; awaiting a destination (roadmap slot / iteration) or a discard.
- promoted  — turned into roadmap/iteration work (Destination says which).
- discarded — closed without doing it; Destination records why.
-->
