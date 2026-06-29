# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added

- **`presets/rust-design-system.md`** — a new preset for a native Rust **design-system crate** (Project Type software, Category "design system"), distinct from `rust-library` (a generic logic crate whose hygiene it inherits) and from `rust-gpui-app` (the application/composition layer that *consumes* it): one token canon (raw ramp → theme aliases → OKLCH semantic states, dark/light sharing one primary, runtime primary override), `Density`/`ColorMode` as typed enums driving the derived variables, embedded two-font typography, an icon-as-shape closed catalog, builder component contracts (stateless `Widget` vs `DsStatefulWidget<State>` + a shared `DsWidget` trait), one-file-per-component layout, centralized motion timings, a single `apply_theme` + context-stashed `tokens()` accessor, the anti-facade clippy `disallowed-methods` boundary with exactly one sanctioned `raw::` escape hatch, shipped WCAG contrast math + accessibility gates, per-component render-without-panic + differential-geometry tests, `publish = false` path-dependency packaging (with a documented publishable switch), a committed root `rust-toolchain.toml`, and an optional companion gallery/storybook. Toolkit-agnostic with `[e.g., egui | GPUI]` placeholders and marked Toolkit-only callouts; terse, mirrors `templates/project/project-definition.md`. Filed as backlog opportunity **O-007** (from iteration 018's stand-in analysis). `presets/README.md` index updated and `templates/project/project-definition.md` gains "design system" as a Category example.
- **`presets/rust-gpui-app.md`** — a new preset for a Rust desktop/UI **application** built on a GPU-accelerated UI framework (retained-mode GPUI, or immediate-mode egui/eframe), distinct from `rust-library` (a published crate): a render-from-state architecture (GUI-agnostic async engine + `UiCommand`/`EngineEvent` channels + a pure headless-testable reducer), the "async-off-the-UI-executor" boundary enforced via clippy `disallowed-methods`, a 3-gate verification model (technical/visual/functional) with headless engine/reducer + geometry + smoke tests, theming via a design-system crate, serde-enum settings + mandatory-`Lang` i18n, `Arc<[T]>` snapshot caching + list virtualization, committed `Cargo.lock` with exact-`rev`-pinned pre-1.0 git deps, a release-tuned `[profile.release]`, and a per-OS human-gated binaries CI. Framework-neutral with marked GPUI-only callouts; terse, mirrors `templates/project/project-definition.md`. Filed as backlog opportunity **O-006** (from iteration 018's stand-in analysis). `presets/README.md` index updated.
- **Multi-agent flow (optional, software)** — a two-agent autonomous milestone loop where a PM/Architect agent plans, specs, and reviews while a Senior Engineer agent implements, fixes, and commits, coordinating through a shared state file.
  - `methodology/multi-agent-flow.md` — the canonical protocol (roles, state machine, signal file, milestone file layout, session-per-milestone, stall cap, branch/commit, completion). Marked "Applies to: software".
  - New operation `start-multi-agent-flow` (arg `pm|engineer`) reusing the existing ops (`create-technical-spec`, `review-changes`, Phase 4, `write-commit-message`), with `.claude/`, `.cursor/`, `.opencode/` ports.
  - New templates `templates/implementation/flow-state.md` (the `.stateful-spec/flow-state.md` signal scaffold) and `templates/implementation/review-handoff.md` (per-milestone code-review handoff).
  - `methodology/roles.md` gains an optional-mode subsection; `start-session`/`end-session` (and ports) gain a milestone-session note; `project-types.md` (software), root `AGENTS.md`/`CLAUDE.md`, `README.md`, and `templates/project/agents-md.md` list the operation.
- **Engramas (compiled memory)** — `memory.md` gains an `Engramas` section: compiled per-iteration summaries (`Summary` / `Key Decisions` / `Learnings`) so `resume-session` gets enough context without reading every `history/` file, with drill-down to the full iteration file only when needed.
  - Map-reduce compaction over each iteration's Session Log; **two-tier compaction** (recent `N` active rows + a single `0-archived` Archive row) keeps the table bounded regardless of project age.
  - `Engramas` section added to `templates/project/memory.md`; central maintenance rule added to `AGENTS.md`; `methodology/overview.md` documents it (Key Files + Why This Matters).
- **Multi-project-type support** — `Project Type` is now a first-class, behavior-driving dimension. The methodology natively supports non-software projects.
  - `methodology/project-types.md` — central registry defining each type (detection signals, terminology mapping, applicable sections/templates/operations).
  - New project types: **skills** (repositories of Agent Skills; zero-code) and **studies** (research/study projects). `software` remains the default.
  - New spec templates: `templates/specification/skill-spec.md`, `study-spec.md`.
  - New implementation template: `templates/implementation/verification-plan.md` (type-aware).
  - New skills operations: `create-skill-spec`, `write-examples`, `diagnose-skill`, `revise-skill` (with `.claude/`, `.opencode/`, `.cursor/` ports).
  - New studies operations: `create-study-spec`, `verify-sources`, `resolve-inconsistency`, `restructure-argument` (with ports).
  - New presets: `presets/skills-repo.md`, `presets/studies-project.md`.
- **Intake & Backlog pipeline (all project types)** — a disciplined home for **opportunities** discovered outside the current milestone.
  - `methodology/backlog.md` — the canon: the `O-NNN` backlog (states `new → triaged → promoted | discarded`, stable/never-reused numbering) fed by a raw `intake/{Backlog,Discovery,QA}/` inbox behind a **READY gate**, triaged manually at session kickoff and close. A defect against the current spec reopens the milestone — it is never a backlog opportunity.
  - New templates: `templates/intake/` (the project-side `Backlog/`/`Discovery/`/`QA/` scaffold with READMEs), `templates/intake-item.md`, `templates/backlog.md`.
- **History archiving & context budget (all types)** — `methodology/history-archiving.md`: a two-tier bound (the Engramas `N` plus a raw-`history/` `RAW_HISTORY` knob, default 3) moves older iterations into an indexed `history/.archived/` that stays resolvable from the History Index but is **never bulk-read**; the next `NNN` scans **both** directories; a folded Engrama's full content is preserved to `history/.archived/memory.md` before it collapses.
- **Post-delivery QA loop (all types)** — `methodology/qa-phase.md`: a loop entered *after* Verify when a delivered artifact fails human/real-world testing — register → route by type (single- vs multi-agent) → categorize **E/P/H** → root-cause → fix+gate+commit per item → turn each into a **lesson → skill**; durable opportunities promote to the backlog. Distinct from spec correction and from the in-flow `[INCIDENT]` log. New template `templates/qa-log.md`.
- **Three-agent flow variant (software)** — `methodology/multi-agent-flow.md` now describes **two modes** in one doc: the existing two-agent flow plus a more rigorous **three-agent** flow that adds a **read-only Architect spec reviewer** (a *design gate* before implementation), per-profile engineer personas, a **hand-back** safety valve, and two independently-bounded loops. It also specifies an implementation-agnostic **tooling contract** (validated verbs / blocking poll / monotonic `seq` / atomic writes) — no code is shipped (documentation-only). `start-multi-agent-flow` gains the `architect` role; new template `templates/implementation/arch-verdict.md`; `templates/implementation/flow-state.md` gains the three-agent fields + `seq`.
- **Optional flow tooling (`packages/`)** — separately-published reference implementations of the multi-agent flow's tooling contract. They are **not required** to use the methodology; an agent must ask the user's permission before using one.
  - `stateful-spec-flow` (Rust → crates.io, zero dependencies) and `@stateful-spec/flow` (Node/TypeScript → npm, zero runtime deps): a validated-verb CLI over `.stateful-spec/flow-state.md` — atomic writes, monotonic `seq`, blocking poll + stall watchdog, the `advance` branch/commit gates + milestone archiving. Curated agnostic from the downstream `stand-in` `flow` crate: the spawn is **config-driven** (`.stateful-spec/flow.conf` command templates — no hardcoded agent runner or personas), and the cargo-fmt gate is dropped (gate evidence kept via `--gate`).
  - Rust ≡ TS parity is enforced by shared fixtures in `packages/conformance/`, which both test suites run.
  - Awareness + the ask-permission rule are wired into `methodology/multi-agent-flow.md` (tooling-contract note), `methodology/roles.md` (a boundary), `start-multi-agent-flow` (+ ports), root `AGENTS.md`, and `README.md`.

### Changed

- Lifecycle operation prompts (`resume-session`, `start-session`, `save-session`, `end-session`) and their `.claude/`, `.cursor/`, `.opencode/` ports updated to read, initialize, and maintain the Engramas section (resume now uses Engramas instead of reading all `history/` files).
- `templates/project/project-definition.md` — `Project Type` made load-bearing; variable sections (Stack/Materials, Conventions, Verification, Quality Gates, Delivery) branched per type via the conditional-section convention.
- `methodology/overview.md`, `phases/04-implement.md`, `phases/05-verify.md` — generalized to be type-aware (terminology mapping, checklist-based gates for non-software types).
- Initialization wizards (`new-project.md`, `onboard-existing.md`, `update-project.md`) — detect/select Project Type and drive type-aware generation and operation emission.
- Software-only operation prompts tagged "Applies to: software" (`write-tests`, `debug-issue`, `refactor-code`, `create-technical-spec`); existing five spec templates and `test-plan.md` tagged likewise.
- `presets/README.md`, root `README.md`, `templates/project/agents-md.md`, root `AGENTS.md`/`CLAUDE.md` — generalized scope and added Project Types pointers.
- Session-lifecycle operations (`start-session`, `resume-session`, `end-session`, `save-session`) and `review-changes` — and all three (`.claude/`, `.cursor/`, `.opencode/`) ports — wired for the supporting structures: intake triage at kickoff/close, the idempotent history-archiving step, next-`NNN` over both `history/` and `history/.archived/`, `[INCIDENT]` capture + a close-time failure sweep, and Engrama fold-preservation.
- `methodology/overview.md` (new "Supporting Structures" section + memory/directory trees), `roles.md` (three-agent + post-delivery-QA notes), and `phases/05-verify.md` (post-delivery-QA exit) integrate the new concepts while keeping the 5-phase cycle unchanged.
- `templates/project/agents-md.md` (Engramas maintenance + intake/archiving rules) and `templates/project/memory.md` (History Index `.archived/` note + Engrama fold note) updated; initialization wizards (`new-project`, `onboard-existing`) scaffold `.stateful-spec/intake/` + `backlog.md`, and `update-project` offers the new scaffolding on refresh.
- The intake/backlog, history-archiving, post-delivery-QA, and three-agent-flow additions were **back-ported** from the downstream `stand-in` consumer (where they were proven in practice), curated to strip project-specific content (tooling implementation, examples, naming, locale) and kept technology-agnostic and documentation-only.
- **`packages/` exception to documentation-only** — `CLAUDE.md`, root `AGENTS.md`, and `.stateful-spec/project-definition.md` now state the repository is *primarily* documentation-only, carving out `packages/` (optional, separately-published reference implementations) as the one place that carries build/test tooling. The methodology core stays Markdown-only.
- **`presets/rust-library.md` revised** — expanded the Rust library preset with learnings distilled from a real multi-crate Rust project: MSRV + `rust-toolchain.toml` pinning, an optional Cargo-workspace block (metadata/dependency inheritance, resolver 2), a proc-macro workspace layout, a Lints subsection (`#![deny(missing_docs)]` + broken-intra-doc-links + `[workspace.lints]`), richer patterns (mod-facade layout, prelude-as-public-API, `#[non_exhaustive]` + constructors, `tracing` without `tracing-subscriber`, serde wire-fidelity, secret-masking `Debug`, `env!`-derived identity), a Feature Flags subsection, an `assert_send_sync` compile-time test convention, and a split Publishing (crates.io) / CI-CD section. Stays a terse, generic preset.

### Fixed

- **O-008 — stale-cold-store report: documentation drift + a session-lifecycle gap.** Investigation (verified against git history) found the cold-store ledger `history/.archived/memory.md` was **not** actually broken — the verbatim append fires once per Engrama fold and every folded row was present. The reported "staleness" was the expected skew between the two independent archive cadences. The genuine defects were elsewhere:
  - The self-applied root `AGENTS.md` rule 7 had drifted from `templates/project/agents-md.md`: it dropped the *"append the folded row's full content verbatim to `history/.archived/memory.md`"* clause and was missing rules 8 (Intake & backlog) and 9 (History archiving). Restored the append clause and added rules 8–9 (paths adjusted for the repo-root methodology source). The same dropped clause was restored in `.stateful-spec/memory.md`'s Engramas note.
  - `methodology/history-archiving.md` and rule 9 of `AGENTS.md` / `templates/project/agents-md.md` now state the cadence independence explicitly — the cold store is written by the Engrama fold (threshold `N`), **independent** of the central-file `git mv` (threshold `RAW_HISTORY`), so it legitimately lags file-archiving by up to `N − RAW_HISTORY` iterations; a recently-archived central file with no cold-store row yet is expected, not a missed append.
  - `resume-session`'s direct-task entry (source + `.cursor`/`.claude`/`.opencode` ports) now inserts the new iteration's Engramas row and runs the two-tier compaction (including the cold-store append), closing a path that previously created an iteration without its engram row.

## [2.0.0] - 2026-03-06

### Changed

- Complete rewrite: transformed from Rust/Ollama-specific methodology to technology-agnostic framework
- Replaced 4-phase model (Foundation, Primitives, Implementation, Conveniences) with universal 5-phase model (Analyze, Plan, Specify, Implement, Verify)
- All artifacts now in English

### Added

- **Methodology core** — `methodology/overview.md`, 5 phase guides, roles definition, decision framework
- **LLM prompts** — 16 ready-to-use prompts organized in 3 categories:
  - Initialization (new project, onboard existing, resume session)
  - Phase transitions (start analysis, planning, specification, implementation, verification)
  - Operations (commit message, update docs, review changes, create spec, write tests, refactor, debug, save session)
- **Templates** — 10 user-fillable templates:
  - Project: Project Definition, Architecture Decision Record
  - Specification: feature, endpoint, component, bugfix, refactor
  - Implementation: implementation plan, test plan
- **Presets** — 5 pre-filled Project Definitions for common stacks:
  - Rust library, Node.js + Express API, Python + FastAPI, React web app, Go service

### Removed

- All Rust-specific content (phase guides, code patterns, type definitions)
- All Ollama-specific content (Modelfile, create-model-request.json)
- Claude Code-specific configuration (.claude/ directory, CLAUDE.md)
- UPDATE_PLAN.md (Rust-specific gap analysis)
- ollama-oxide reference example

## [1.0.0] - 2024-01-01

### Added

- Initial methodology for Rust library development with AI assistance
- Phase guides for Foundation, Primitives, Implementation, Conveniences
- Endpoint specification template (YAML)
- Implementation plan template
- Ollama Modelfile with embedded methodology
- ollama-oxide reference implementation example
