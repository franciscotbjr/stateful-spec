# Iteration: 015 — Publish flow-rs to crates.io

> One file per feature, bugfix, or refactor. Track progress and decisions here.

## Metadata

- **Type:** chore
- **Status:** in-progress
- **Created:** 2026-06-28
- **Completed:** —
- **Author:** Francisco Tarcizo Bomfim Júnior

## Description

Publish the optional reference crate `stateful-spec-flow` (`packages/flow-rs/`) to crates.io.
Starting point is updating `Cargo.toml` with the metadata crates.io requires for a publish
(description, license, repository, keywords, categories, etc.) and preparing the release.

Promoted from the intake inbox (`intake/Backlog/prd.md`, origin: idea) as opportunity **O-002**.

## Acceptance Criteria

> Checkboxes for what "done" means. These come from the PRD (external intake source).

- [ ] `packages/flow-rs/Cargo.toml`: `edition = "2024"` and `rust-version = "1.96"` (per PRD properties)
- [ ] `keywords` / `categories` researched and chosen for best crates.io discoverability
- [ ] All tests pass (`cargo test`)
- [ ] `cargo publish --dry-run` succeeds
- [ ] Crate published to crates.io

## PRD Properties (binding)

> From the intake PRD `…/new - 01 - publish flow-rs to cartes io/product requirements document.md`.

- **rust-version (MSRV):** `1.96`
- **rust-edition:** `2024`
- **Publish token:** provided in the PRD frontmatter — **secret**; used only at `cargo publish` time via env (`CARGO_REGISTRY_TOKEN`); never committed to the repo, git history, or any file.

## Implementation Tasks

> Breakdown of work, in PRD order. Check off as you go.

- [ ] Update `Cargo.toml`: bump `edition` → `2024`, `rust-version` → `1.96`
- [ ] Research & set best `keywords` / `categories` for discoverability
- [ ] Run all tests (`cargo test`)
- [ ] Run `cargo publish --dry-run`
- [ ] Publish (`cargo publish`) — confirm with developer first (irreversible)

## Quality Checks

> Standard checks from the Project Definition. Verify before marking done.

- [ ] `cargo test` passes in `packages/flow-rs/`
- [ ] `cargo publish --dry-run` clean
- [ ] Documentation updated (if applicable)
- [ ] No debug code or TODOs left behind

## Session Log

> Timestamped entries recording each operation performed during this session.
> Agents append entries automatically when an Open Session is active.

| Timestamp | Operation | Summary |
|-----------|-----------|---------|
| 2026-06-28 | start-session | Session opened. Triaged intake `prd.md` → promoted as O-002; iteration 015 created for publishing `flow-rs` to crates.io. |

> **Timestamp format:** `YYYY-MM-DD HH:MM` (local time). Example: `2026-05-03 14:30 | start-session | Session opened for feature work.`
>
> **Note:** Iterations created prior to the session management feature may lack this section. This is expected and does not require migration.

## Decisions Made

> Decisions made during this iteration. Include rationale.

| Decision | Rationale | Date |
|----------|-----------|------|
| Promote `prd.md` to O-002 and take it as this session's work | The idea is a concrete, durable release opportunity for the existing `packages/flow-rs/` crate | 2026-06-28 |

## Blockers & Notes

> Anything that blocked progress or is worth remembering.

- Publishing to crates.io requires a registry token and is an irreversible/outward-facing action — confirm with the developer before the actual `cargo publish`.

## References

- **Specification:** —
- **PR/MR:** —
- **Commits:** —
- **Related Issues:** Backlog O-002; intake/Backlog/prd.md
