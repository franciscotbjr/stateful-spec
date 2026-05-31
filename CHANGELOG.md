# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added

- **Multi-project-type support** — `Project Type` is now a first-class, behavior-driving dimension. The methodology natively supports non-software projects.
  - `methodology/project-types.md` — central registry defining each type (detection signals, terminology mapping, applicable sections/templates/operations).
  - New project types: **skills** (repositories of Agent Skills; zero-code) and **studies** (research/study projects). `software` remains the default.
  - New spec templates: `templates/specification/skill-spec.md`, `study-spec.md`.
  - New implementation template: `templates/implementation/verification-plan.md` (type-aware).
  - New skills operations: `create-skill-spec`, `write-examples`, `diagnose-skill`, `revise-skill` (with `.claude/`, `.opencode/`, `.cursor/` ports).
  - New studies operations: `create-study-spec`, `verify-sources`, `resolve-inconsistency`, `restructure-argument` (with ports).
  - New presets: `presets/skills-repo.md`, `presets/studies-project.md`.

### Changed

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
