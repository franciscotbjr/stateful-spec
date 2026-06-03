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

### Changed

- Lifecycle operation prompts (`resume-session`, `start-session`, `save-session`, `end-session`) and their `.claude/`, `.cursor/`, `.opencode/` ports updated to read, initialize, and maintain the Engramas section (resume now uses Engramas instead of reading all `history/` files).
- `templates/project/project-definition.md` — `Project Type` made load-bearing; variable sections (Stack/Materials, Conventions, Verification, Quality Gates, Delivery) branched per type via the conditional-section convention.
- `methodology/overview.md`, `phases/04-implement.md`, `phases/05-verify.md` — generalized to be type-aware (terminology mapping, checklist-based gates for non-software types).
- Initialization wizards (`new-project.md`, `onboard-existing.md`, `update-project.md`) — detect/select Project Type and drive type-aware generation and operation emission.
- Software-only operation prompts tagged "Applies to: software" (`write-tests`, `debug-issue`, `refactor-code`, `create-technical-spec`); existing five spec templates and `test-plan.md` tagged likewise.
- `presets/README.md`, root `README.md`, `templates/project/agents-md.md`, root `AGENTS.md`/`CLAUDE.md` — generalized scope and added Project Types pointers.

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
