# Iteration: 015 — Publish flow-rs to crates.io

> One file per feature, bugfix, or refactor. Track progress and decisions here.

## Metadata

- **Type:** chore
- **Status:** done
- **Created:** 2026-06-28
- **Completed:** 2026-06-28
- **Author:** Francisco Tarcizo Bomfim Júnior

## Description

Publish the optional reference crate `stateful-spec-flow` (`packages/flow-rs/`) to crates.io.
Starting point is updating `Cargo.toml` with the metadata crates.io requires for a publish
(description, license, repository, keywords, categories, etc.) and preparing the release.

Promoted from the intake inbox (`intake/Backlog/prd.md`, origin: idea) as opportunity **O-002**.

## Acceptance Criteria

> Checkboxes for what "done" means. These come from the PRD (external intake source).

- [x] `packages/flow-rs/Cargo.toml`: `edition = "2024"` and `rust-version = "1.96"` (per PRD properties)
- [x] `keywords` / `categories` researched and chosen for best crates.io discoverability
- [x] All tests pass (`cargo test`) — 54 unit + 7 CLI + 1 conformance = 62
- [x] `cargo publish --dry-run` succeeds
- [x] Crate published to crates.io — v0.1.0 live at https://crates.io/crates/stateful-spec-flow

## PRD Properties (binding)

> From the intake PRD `…/new - 01 - publish flow-rs to cartes io/product requirements document.md`.

- **rust-version (MSRV):** `1.96`
- **rust-edition:** `2024`
- **Publish token:** provided in the PRD frontmatter — **secret**; used only at `cargo publish` time via env (`CARGO_REGISTRY_TOKEN`); never committed to the repo, git history, or any file.

## Implementation Tasks

> Breakdown of work, in PRD order. Check off as you go.

- [x] Update `Cargo.toml`: bump `edition` → `2024`, `rust-version` → `1.96`
- [x] Research & set best `keywords` / `categories` for discoverability
- [x] Run all tests (`cargo test`)
- [x] Run `cargo publish --dry-run`
- [x] Publish (`cargo publish`) — confirmed by developer; v0.1.0 live

## Quality Checks

> Standard checks from the Project Definition. Verify before marking done.

- [x] `cargo test` passes in `packages/flow-rs/` (62 tests)
- [x] `cargo publish --dry-run` clean
- [x] Documentation updated (README MSRV synced to 1.96)
- [x] No debug code or TODOs left behind

## Session Log

> Timestamped entries recording each operation performed during this session.
> Agents append entries automatically when an Open Session is active.

| Timestamp | Operation | Summary |
|-----------|-----------|---------|
| 2026-06-28 | start-session | Session opened. Triaged intake `prd.md` → promoted as O-002; iteration 015 created for publishing `flow-rs` to crates.io. |
| 2026-06-28 | implement | Applied PRD properties to `Cargo.toml` (edition 2024, MSRV 1.96), refined keywords/categories, synced README MSRV. `cargo check` + `cargo test` (62) green; `cargo publish --dry-run` clean. Committed on branch `chore/publish-flow-rs-crates-io`. |
| 2026-06-28 | implement | Published `stateful-spec-flow v0.1.0` to crates.io (token via env, not stored). Live at https://crates.io/crates/stateful-spec-flow. |
| 2026-06-28 | end-session | Session closed. Followed PRD properties (edition 2024, MSRV 1.96) and task order: updated Cargo.toml, refined keywords/categories, 62 tests green, dry-run clean, published v0.1.0. All criteria met. Flagged token rotation (PRD stored token in plaintext). |

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

- Publishing to crates.io requires a registry token and is an irreversible/outward-facing action — confirm with the developer before the actual `cargo publish`. (Confirmed; published v0.1.0.)
- **Security:** the PRD (`…/product requirements document.md`) stored the crates.io API token in plaintext frontmatter, and it was passed on the command line. Recommend revoking it at https://crates.io/settings/tokens and removing it from the PRD. The token was never written into this repo, git, or any committed file.
- Crate versions on crates.io are permanent — v0.1.0 can be yanked but never deleted or re-uploaded; future changes require a version bump.

## References

- **Specification:** intake/Backlog/prd.md (external PRD)
- **PR/MR:** branch `chore/publish-flow-rs-crates-io`
- **Commits:** `a6a957c` (prep) + end-session close commit
- **Related Issues:** Backlog O-002; intake/Backlog/prd.md
- **Published:** https://crates.io/crates/stateful-spec-flow (v0.1.0)
