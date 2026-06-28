# Iteration: 019 — rust-gpui-app-preset

> One file per feature, bugfix, or refactor. Track progress and decisions here.

## Metadata

- **Type:** feature
- **Status:** done
- **Created:** 2026-06-28
- **Completed:** 2026-06-28
- **Author:** Francisco Tarcizo Bomfim Júnior

## Description

Add a `rust-gpui-app` preset (`presets/rust-gpui-app.md`) — a pre-filled Project
Definition for a Rust retained-mode/GPU desktop/UI application (GPUI or egui/eframe),
distinct from `rust-library`. Captures the render-from-state architecture (GUI-agnostic
async engine + `UiCommand`/`EngineEvent` channels + a pure headless-testable reducer),
the "UI-executor-is-not-tokio" async boundary enforced via clippy `disallowed-methods`,
a 3-gate verification model (technical/visual/functional) with smoke + headless-geometry
tests, theming-via-design-system composition, serde-enum settings + mandatory-`Lang`
i18n, virtualized lists / `Arc<[T]>` snapshot caching, committed-`Cargo.lock` pinning of
pre-1.0 git deps, a release-tuned profile, and a per-OS human-gated binaries CI.

This opportunity (**O-006**) was surfaced during iteration 018's stand-in analysis as the
"propose other presets" half of the rust PRD, deferred to the backlog. Documentation-only.

## Acceptance Criteria

> Checkboxes for what "done" means. These come from the specification or user story.

- [x] `presets/rust-gpui-app.md` exists and mirrors the structure of `templates/project/project-definition.md` as a **terse preset** (not an architecture essay)
- [x] Covers the render-from-state architecture: GUI-agnostic async engine, `UiCommand`/`EngineEvent` channels, pure headless-testable reducer
- [x] Documents the "UI-executor-is-not-tokio" async boundary enforced via clippy `disallowed-methods`
- [x] Specifies the 3-gate verification model (technical / visual / functional) with smoke + headless-geometry tests
- [x] Captures theming-via-design-system composition, serde-enum settings + mandatory-`Lang` i18n, virtualized lists / `Arc<[T]>` snapshot caching, committed-`Cargo.lock` pinning, release-tuned profile, per-OS human-gated binaries CI
- [x] Stand-in/MCP/GPUI-specific evidence is generalized to framework-neutral guidance with clearly-marked GPUI-only callouts (no leakage — grep clean)
- [x] The `--capture` CLI spec is reduced to a one-line "fixtures gated, never ship mocks" constraint
- [x] A row for `rust-gpui-app` is added to the presets index
- [x] CHANGELOG `[Unreleased]` updated

## Implementation Tasks

> Breakdown of work. Check off as you go.

- [x] Read the external PRD / stand-in evidence and the `rust-library` preset for shape parity
- [x] Draft `presets/rust-gpui-app.md` mirroring the project-definition template
- [x] Generalize GPUI/stand-in specifics to framework-neutral guidance with marked GPUI-only callouts
- [x] Add the presets-index row
- [x] Update CHANGELOG
- [x] Self-review: leakage check (grep stand-in/MCP), terseness, template parity

## Quality Checks

> Standard checks from the Project Definition. Verify before marking done.

- [x] All quality gates pass (manual review — documentation-only, no build/lint/test)
- [x] Preset mirrors `templates/project/project-definition.md` structure
- [x] Documentation updated (presets index + CHANGELOG)
- [x] No debug code or TODOs left behind

## Session Log

> Timestamped entries recording each operation performed during this session.
> Agents append entries automatically when an Open Session is active.

| Timestamp | Operation | Summary |
|-----------|-----------|---------|
| 2026-06-28 19:20 | start-session | Session opened for O-006 — add `presets/rust-gpui-app.md`. Kickoff triage: intake `prd.md` already triaged (→ O-005); no new ready items. |
| 2026-06-28 19:25 | analyze | Read `presets/rust-library.md` (Rust-convention parity), `presets/go-service.md` (binary/app variants: release profile, no crates.io), the `templates/project/project-definition.md` skeleton, `presets/README.md` index, CHANGELOG, and iteration 018 (which distilled O-006 from the stand-in app-ui learnings, 116 of them, adversarially verified). Decision: author the preset from the **O-006 verifier-refined spec + rust-library parity**, not a fresh stand-in multi-agent re-analysis — that synthesis was already de-leaked in 018; re-running risks re-introducing leaks at high cost. |
| 2026-06-28 19:30 | implement | Wrote `presets/rust-gpui-app.md` (~180 lines) directly to `presets/` (not the session-scoped scratchpad, per the 018 lesson): render-from-state architecture (engine ↔ UI via `UiCommand`/`EngineEvent`, pure `reduce()`), the async-off-the-UI-executor clippy `disallowed-methods` boundary (with `clippy.toml` example), 3-gate verification (technical/visual/functional) + headless engine/reducer/geometry/smoke tests, theming-via-design-system, serde-enum settings + mandatory-`Lang` i18n, `Arc<[T]>` snapshots + list virtualization, committed `Cargo.lock` + exact-`rev` git-dep pinning, release-tuned `[profile.release]`, per-OS human-gated binaries CI. Framework-neutral with marked **GPUI-only** callouts. Added the `presets/README.md` index row and a CHANGELOG `[Unreleased] › Added` entry. |
| 2026-06-28 19:32 | review-changes | Self-review: leakage grep (`stand-in`/`MCP`/`explorer`/`studio`/…) clean — GPUI appears only in intentional callouts; preset mirrors the project-definition template and stays terse (sibling-comparable to `rust-library`); all 9 acceptance criteria met. Awaiting developer review before commit. |
| 2026-06-28 19:35 | write-commit-message | Developer approved. Committed to new branch `feature/019-rust-gpui-app-preset` (commit `82283ce`, 6 files: preset + README index + CHANGELOG + iteration + memory bookkeeping) — single-line message ≤300 chars (298) per convention, Co-Authored-By trailer. Not pushed / no PR (awaiting explicit instruction). Note: `rtk` is not installed in this shell, fell back to plain `git`. |
| 2026-06-28 19:40 | review-changes | Post-commit review of the branch diff surfaced two accuracy fixes (applied): (1) **egui is immediate-mode, not retained-mode** — reframed the preset + CHANGELOG from "retained-mode/GPU UI framework" to "GPU-accelerated, retained-mode (GPUI) / immediate-mode (egui)", noting render-from-state is the app's discipline layered on either mode; (2) resolved the apparent clippy contradiction — added that the **engine spawns via an explicit runtime `Handle`** (lint untouched) and only the **ambient free-function** `tokio::spawn` (the UI-thread mistake) is blocked. Rest of the diff verified: template parity, terseness, leakage-clean. Committed as `57c075e`. |
| 2026-06-28 19:56 | end-session | Session closed. Delivered `presets/rust-gpui-app.md` (O-006) + `presets/README.md` index row + CHANGELOG `[Unreleased]` entry; all 9 acceptance criteria + 4 quality checks met → status `done`. Two commits on branch `feature/019-rust-gpui-app-preset` (`82283ce`, `57c075e`); **not pushed / no PR** (developer chose to close without publishing). Close triage: only intake item `prd.md` already `triaged` (→ O-005), no `ready` items. Failure sweep: no `[INCIDENT]` entries. Compiled the 019 Engrama; archived 016 central (RAW_HISTORY=3). O-007 (`rust-design-system`) remains `new` in the backlog. |

> **Timestamp format:** `YYYY-MM-DD HH:MM` (local time). Example: `2026-05-03 14:30 | start-session | Session opened for feature work.`
>
> **Note:** Iterations created prior to the session management feature may lack this section. This is expected and does not require migration.

## Decisions Made

> Decisions made during this iteration. Include rationale.

| Decision | Rationale | Date |
|----------|-----------|------|
| Pick O-006 (`rust-gpui-app`) over O-007 (`rust-design-system`) | Developer choice at session start; both are `new` backlog opportunities from iteration 018, taken one at a time | 2026-06-28 |

## Blockers & Notes

> Anything that blocked progress or is worth remembering.

- O-007 (`rust-design-system`) remains `new` in the backlog for a later session.

## References

- **Specification:** O-006 in `.stateful-spec/backlog.md`
- **PR/MR:** —
- **Commits:** `82283ce` (branch `feature/019-rust-gpui-app-preset`)
- **Related Issues:** Iteration 018 (`history/018-apply-preset-rust.md`) — stand-in analysis that surfaced O-006
