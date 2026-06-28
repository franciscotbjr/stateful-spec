# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added

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
