# Iteration: 023 — Fix Post-022 Drift

> One file per feature, bugfix, or refactor. Track progress and decisions here.

## Metadata

- **Type:** chore
- **Status:** done
- **Created:** 2026-10-02
- **Completed:** 2026-10-02
- **Author:** Francisco Tarcizo Bomfim Júnior

## Description

**Retroactive** iteration (created by `save-session` STEP 2.5). The `resume-session` context load
surfaced two documentation drifts, both corrected in this session:

1. `.stateful-spec/memory.md` **Active Work** still said iteration 022's PR was "drafted, not yet
   opened", while `git log` shows it merged to `main` as **PR #43** (`6c0fdda`, 2026-07-04).
2. The root `README.md` **Available Presets** table listed 7 presets and omitted
   `rust-gpui-app` (iteration 019) and `rust-design-system` (iteration 020), both present in
   `presets/` and indexed in `presets/README.md:19-20`.

## Acceptance Criteria

- [x] `memory.md` Active Work reflects 022's merge via PR #43 (`6c0fdda`); Last Updated → 2026-10-02
- [x] Root `README.md` presets table lists all 9 presets in `presets/`, wording aligned with `presets/README.md`

## Implementation Tasks

- [x] Load context (`resume-session`), kickoff triage — no `ready` intake items
- [x] Fix `memory.md` Active Work + Last Updated
- [x] Add `rust-gpui-app` and `rust-design-system` rows to root `README.md`
- [x] Draft commit message (`write-commit-message`)

## Quality Checks

- [x] Manual review of the diff (2 files, +4/−2) — no quality-gate commands exist for this repo
- [x] Documentation updated (this iteration is itself a documentation fix)
- [x] No debug content or TODOs left behind

## Session Log

| Timestamp | Operation | Summary |
|-----------|-----------|---------|
| 2026-10-02 15:12 | resume-session | Context loaded (memory, project-definition, `methodology/`, persona, README, CHANGELOG). Kickoff triage: no `ready` intake items (`Backlog/prd.md` triaged → O-009; `Discovery/persona-ex-post-evaluation.md` draft until ≈027). Surfaced 2 drifts: stale 022 Active Work line; README presets table missing 2 presets. |
| 2026-10-02 15:12 | direct-task | Fixed both drifts: `memory.md` Active Work (022 merged via PR #43) + Last Updated; `README.md` +2 preset rows. Treated as trivial at the time (no iteration, §6 checklist skipped per persona rule 3.1). |
| 2026-10-02 15:12 | write-commit-message | Drafted a single-line commit message (`docs: corrige drifts pós-022 …`). |
| 2026-10-02 15:12 | end-session | No Open Session — nothing to close; offered commit / retroactive save / start-session. |
| 2026-10-02 15:12 | save-session | Developer chose a retroactive iteration: created this file (023); History Index + Engramas + Recent Completions updated; Engrama fold of 013 (verbatim append to `history/.archived/memory.md`); archive op moved central 020 to `history/.archived/` (RAW_HISTORY=3). |

## Decisions Made

| Decision | Rationale | Date |
|----------|-----------|------|
| Record the trivial drift fix as a retroactive iteration instead of a memory-only note | Developer's choice at `save-session`; keeps the History Index / Engramas trail complete | 2026-10-02 |
| Leave the 022 Engrama text ("texto de PR redigido, abertura manual") unchanged | It records what was true when 022 closed; the correction belongs in Active Work, not in the historical summary | 2026-10-02 |

## Blockers & Notes

- Session Log timestamps are **approximate**: the iteration was created retroactively and earlier operations were not timestamped live; all entries carry the save time.
- Drift class observed again: adding a preset updates `presets/README.md` but not the root `README.md` index (019/020). Not routed to the backlog — single occurrence pair, fixed here.
- [INCIDENT/P] The Engrama-fold script dropped rows by the prefix `| 013 |`, which also matched the History Index row · caught by a per-number row-count check before saving and restored. One-off; lesson: match the row inside its table section, not by prefix across the whole file.

## References

- **Specification:** —
- **PR/MR:** —
- **Commits:** `ac70502` (drift fix, on `main`); save-session bookkeeping uncommitted at save time
- **Related Issues:** iteration 022 (PR #43, `6c0fdda`); iterations 019/020 (presets)
