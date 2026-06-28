# Iteration: 014 — Self-adoption (repo adopts its own methodology)

> Umbrella iteration. Track progress and decisions here.

## Metadata

- **Type:** chore
- **Status:** done
- **Created:** 2026-06-28
- **Completed:** 2026-06-28
- **Author:** Francisco Tarcizo Bomfim Júnior

## Description

This repository **is** the Stateful Spec methodology source, and it consumes its own methodology
(`.stateful-spec/` uses the files at the repo root). After iterations 012 (back-port of
intake/backlog, history-archiving, qa-phase) and 013 (flow packages), an audit *"as a user of its
own methodology"* found the repo's own `.stateful-spec/` instance had not yet adopted the structures
the methodology now documents. This iteration closes those self-adoption gaps so the repo practices
what it ships.

## Acceptance Criteria

- [x] `.stateful-spec/intake/` exists with `Backlog/`, `Discovery/`, `QA/`, each carrying the READY-convention README (from `templates/intake/`)
- [x] `.stateful-spec/backlog.md` exists with the `O-NNN` table; the self-adoption opportunity is recorded as `O-001` (promoted → 014)
- [x] `project-definition.md` Repository Structure + Key Directories list `packages/` and the artifacts added in 007/008/012/013
- [x] `RAW_HISTORY = 3` documented in `memory.md` beside the Engramas `N`
- [x] Intake triage at kickoff/close now has a real inbox to scan (no longer a silent no-op)

## Implementation Tasks

- [x] Scaffold `.stateful-spec/intake/` + 4 READMEs
- [x] Create `.stateful-spec/backlog.md` (O-001 = self-adoption → 014)
- [x] Update `project-definition.md` (structure tree + Key Directories)
- [x] Document `RAW_HISTORY=3` in `memory.md`
- [x] Close iteration (Engrama 014; refold 003; archive 011)

## Quality Checks

> Documentation-only — quality gate is manual review.

- [x] intake/ READMEs reference the root `methodology/` canon correctly
- [x] backlog.md follows the `templates/backlog.md` format (O-NNN, states)
- [x] project-definition.md structure matches the actual repo tree
- [x] memory.md History Index `File` cells still resolve

## Session Log

| Timestamp | Operation | Summary |
|-----------|-----------|---------|
| 2026-06-28 | start-session | Opened iteration 014 (self-adoption) from the audit *"repo as a user of its own methodology"* — gaps: missing `intake/` + `backlog.md`, stale `project-definition.md` structure, undocumented `RAW_HISTORY`. Triage at kickoff: intake inbox does not exist yet (this iteration creates it); nothing to triage. |
| 2026-06-28 | implement | Closed the 4 self-adoption gaps: scaffolded `.stateful-spec/intake/` (README + `Backlog/`/`Discovery/`/`QA/` READMEs from `templates/intake/`); created `.stateful-spec/backlog.md` with `O-001` (self-adoption, promoted → 014); refreshed `project-definition.md` Repository Structure + Key Directories (now list `packages/` and the 007/008/012/013 artifacts); documented `RAW_HISTORY=3` in `memory.md` beside the History Index. |
| 2026-06-28 | end-session | Session closed; Open Session cleared. Status → **done**. Engrama 014 compiled (N=10 folded **003** into `0-archived`, preserved verbatim to `history/.archived/memory.md`); archive op `git mv`'d the **011** central into `history/.archived/` (RAW_HISTORY=3 → `history/` keeps 012/013/014); moved to Recent Completions; History Index updated. No `[INCIDENT]` entries to sweep. |

## Decisions Made

| Decision | Rationale | Date |
|----------|-----------|------|
| Repo adopts its own intake/backlog now | The methodology documents them (012); a self-referential source must practice what it ships, or the triage steps stay silent no-ops | 2026-06-28 |
| Self-adoption recorded as O-001 (promoted → 014) | Demonstrates the intake → backlog → roadmap pipeline on the repo itself | 2026-06-28 |

## Blockers & Notes

- The `intake/` READMEs are copied from `templates/intake/`; their prose references to `methodology/...` resolve to the repo-root methodology (this repo IS the source — see `CLAUDE.md` "Methodology source location").
- Closing 014 will refold Engrama 003 into `0-archived` (N=10) and `git mv` the 011 central into `history/.archived/` (RAW_HISTORY=3).

## References

- **Audit:** self-adoption review (this session)
- **Templates:** `templates/intake/`, `templates/backlog.md`, `templates/intake-item.md`
- **Commits:** —
