# Iteration: 021 — Fix Archived Memory Ledger

> One file per feature, bugfix, or refactor. Track progress and decisions here.

## Metadata

- **Type:** bugfix
- **Status:** done
- **Created:** 2026-06-28
- **Completed:** 2026-06-28
- **Author:** Francisco Tarcizo Bomfim Júnior

## Description

The methodology's history-archiving does not keep `history/.archived/memory.md` updated as
iterations are archived — the archived-engram ledger goes stale. The `0-archived` Engramas row in
`memory.md` claims its full content is "preserved in `history/.archived/memory.md`", but the
verbatim-append step that should fire when an active engram row collapses into the Archive row is
not actually happening in practice.

Review the session-lifecycle operations (`start-session` STEP 5/5.5, `end-session`, `save-session`)
and `methodology/history-archiving.md` to find where the verbatim-append of a collapsed engram row
to `history/.archived/memory.md` is being skipped or under-specified, fix it across the source
prompts **and** the three tool ports (`.cursor/rules/`, `.claude/commands/`, `.opencode/commands/`),
then reconcile (backfill) `history/.archived/memory.md` itself so it is consistent with the engrams
that have already been folded into the `0-archived` row.

Backlog: **O-008**. Source: `intake/Backlog/prd.md` ("Falhas nas operações da metodologia").

## Acceptance Criteria

> Checkboxes for what "done" means. These come from the specification or user story.

- [x] Root cause identified: **not** a skipped append. The cold store was already correct (verified
      against git history — the append fires once per fold). The real defects were (1) doc drift in
      the self-applied root `AGENTS.md` rule 7 + missing rules 8/9, (2) the same dropped clause in
      `memory.md`'s Engramas note, (3) a `resume-session` direct-task gap, and the report itself was
      a **cadence misperception** (engram fold `N` vs file-archiving `RAW_HISTORY`).
- [x] Fix applied to the source prompts / archiving doc (`methodology/history-archiving.md` cadence
      clarity; `resume-session` source; `AGENTS.md` rules 7–9; `templates/project/agents-md.md`
      rule 9; `.stateful-spec/memory.md` Engramas note).
- [x] Fix mirrored across the three tool ports (`.cursor/rules/`, `.claude/commands/`,
      `.opencode/commands/`) for the `resume-session` change — verified identical.
- [x] `history/.archived/memory.md` reconciled — it already matches the engrams folded into
      `0-archived` (`001-008`, `010`, `011`); `011` was appended during this session's start-session.
      Under the chosen N=10 design, **no backfill is owed**.
- [x] CHANGELOG updated (Keep a Changelog format) — `Fixed` entry for O-008.

## Implementation Tasks

> Breakdown of work. Check off as you go.

- [x] Diagnose: traced every place the engram lifecycle preserves content in `.archived/memory.md`
      (9-agent adversarial workflow + git-history reconstruction).
- [x] Decide the fix shape: the fold **owns** the append (single owning step); other ops reference
      `start-session` STEP 5 item 4 rather than duplicating the algorithm.
- [x] Edit `methodology/history-archiving.md` (cadence-skew clarity) + `resume-session` source +
      root `AGENTS.md` (rules 7–9) + `templates/project/agents-md.md` (rule 9) + `memory.md` note.
- [x] Mirror `resume-session` edit to `.cursor/rules/`, `.claude/commands/`, `.opencode/commands/`.
- [x] Backfill `history/.archived/memory.md` — N/A under N=10 (already current; `011` appended at start-session).
- [x] Update CHANGELOG.

## Quality Checks

> Standard checks from the Project Definition. Verify before marking done.

- [x] Manual review (documentation-only project — no lint/format/test/build).
- [x] Source ↔ tool-port sync verified across `.cursor`/`.claude`/`.opencode` (resume-session: 4 files identical).
- [x] Documentation updated (CHANGELOG; `history-archiving.md`; `AGENTS.md` + template; `memory.md` note).
- [x] No debug code or TODOs left behind.

## Session Log

> Timestamped entries recording each operation performed during this session.
> Agents append entries automatically when an Open Session is active.

| Timestamp | Operation | Summary |
|-----------|-----------|---------|
| 2026-06-28 | start-session | Session opened for O-008 — fix stale `history/.archived/memory.md` archiving. Triaged intake `prd.md` → promoted O-008. |
| 2026-06-28 | analyze (workflow) | Diagnosed via 9-agent workflow (5 readers → synth → 3 adversarial verifiers, ~460k tok; 1 verifier reconstructed git history). Verified finding: cold store is **not** broken — the verbatim-append fires per-iteration (git-confirmed 004→i015 … 011→i021/this session); the "staleness" is a cadence **misperception** (engram-fold N=10 vs file-archiving RAW_HISTORY=3, ≤7-iteration skew, documented as "independent knobs"). Real defects are doc-drift: (1) root `AGENTS.md` rule 7 dropped the cold-store-append clause + missing rules 8/9; (2) `memory.md` Engramas note dropped the same clause; (3) `resume-session` direct-task entry silent on engram-row insert/fold. No backfill owed under the current (correct) design. |
| 2026-06-28 | decision (dev) | Developer chose **keep N=10** (engram-fold cadence) over re-wiring the cold store to mirror file-archiving (RAW_HISTORY). → no backfill; fix = doc-drift repair + resume-session gap + cadence-clarity. |
| 2026-06-28 | implement | Restored the cold-store-append clause to root `AGENTS.md` rule 7 and added the missing rules 8 (Intake & backlog) + 9 (History archiving), path-adjusted to repo-root `methodology/`. Added cadence-independence sentence to rule 9 (root **and** `templates/project/agents-md.md`). Restored the append clause in `.stateful-spec/memory.md`'s Engramas note. Added a "Timing skew" bullet to `methodology/history-archiving.md` *Preserving folded Engramas*. Wired `resume-session` direct-task entry to insert the Engramas row + run the fold/append (source + 3 ports, verified identical). CHANGELOG `Fixed` entry added. |
| 2026-06-28 | end-session | Session closed; all criteria met (Status → done). Fix committed on `fix/021-fix-archived-memory` (`e3f7978`). Close triage: no `ready` intake items. Engrama compiled; O-008 marked `promoted` (→ 021); iteration 018 central archived (RAW_HISTORY=3). No `[INCIDENT]` entries. |

> **Timestamp format:** `YYYY-MM-DD HH:MM` (local time). Example: `2026-05-03 14:30 | start-session | Session opened for feature work.`
>
> **Note:** Iterations created prior to the session management feature may lack this section. This is expected and does not require migration.

## Decisions Made

> Decisions made during this iteration. Include rationale.

| Decision | Rationale | Date |
|----------|-----------|------|
| Track O-008 as a `bugfix` iteration | It corrects a defect in existing methodology operations (stale archived ledger), not new behavior. | 2026-06-28 |
| Keep the cold store tied to the **engram fold (N=10)**, not file-archiving (RAW_HISTORY=3) | Developer's call. The cold store's purpose is a lossless backup of a row about to collapse into `0-archived`; tying it to N=10 is internally consistent and lossless. Mirroring file-archiving would duplicate live rows and need a de-dup guard. | 2026-06-28 |
| **No backfill** of `history/.archived/memory.md` | Git history confirms the append fired at every fold; the cold store already holds exactly the folded set (`001-008`, `010`, `011`). Backfilling `012-017` (still-active rows) would double-write when they later fold. | 2026-06-28 |
| Fix by **template-alignment**, not new invention | The operative procedure (canon + 12 op prompts/ports) was already correct; the defects were stale self-applied summary copies. Re-aligning root `AGENTS.md` + `memory.md` note to their templates eliminates drift at the source. | 2026-06-28 |

## Blockers & Notes

> Anything that blocked progress or is worth remembering.

- The PRD carries a curator persona + exhaustive-questioning protocol: investigate hypotheses from
  the repo first, ask one question at a time only when the repo cannot answer it. Honor this during
  the diagnosis/plan phase before implementing.

## References

- **Specification:** `.stateful-spec/intake/Backlog/prd.md` (intake item, triaged → O-008)
- **PR/MR:** [Link when available]
- **Commits:** [Key commit hashes or range]
- **Related Issues:** Backlog O-008
