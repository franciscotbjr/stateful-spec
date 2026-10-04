# Iteration: 024 — Multi-Repo Workspace

> One file per feature, bugfix, or refactor. Track progress and decisions here.

## Metadata

- **Type:** feature
- **Status:** in-progress
- **Created:** 2026-10-02
- **Completed:** —
- **Author:** Francisco Tarcizo Bomfim Júnior

## Description

O-010 — let an AI agent run from a **Workspace** folder/repository that maps several other
repositories (or folders) while still following the methodology. The Workspace shares knowledge
across its members, so progress and learnings from one repository benefit the others (precedent:
`stand-in` learnings fed back into commands, skills, and the methodology). The Workspace keeps a
**pointer index** to each member's memory instead of replicating it, and the agent follows
pointers on demand. Source: `intake/Backlog/prd.md`. The PRD asks for exhaustive design Q&A, one
question at a time, with every decision and its alternatives documented. It also asks for web
research on validated references.

## Acceptance Criteria

> From the PRD (`intake/Backlog/prd.md`); to be refined during Analyze/Plan.

- [ ] No regression: running the agent from an already-configured repository keeps the methodology exactly as it is
- [ ] The list of known repositories lives in a registry (YAML, XML, or a more AI-efficient format) that forms the Workspace
- [ ] From a new session in the Workspace, the agent can work on any of its member projects
- [ ] From a new session in the Workspace, the agent can create a new project (folder/repository)
- [ ] From a new session in the Workspace, an idea can be drafted until it becomes an actionable item, through a dedicated drafting command
- [ ] A new command adds a folder/repository to the Workspace: it updates the registry and indexes pointers to that member's knowledge (memory)
- [ ] More than one session may be active at once, at most one per member folder/repository
- [ ] The Workspace has its own `AGENTS.md` and its own methodology configuration

## Implementation Tasks

- [x] Analyze: web research on multi-repo / workspace context management and pointer-based memory indexing (validated references)
- [x] Analyze: map current single-repo assumptions in `methodology/`, `prompts/`, and `templates/`
- [ ] Plan: exhaustive design Q&A with the developer (one question at a time; alternatives and rationale recorded in Decisions Made)
- [ ] Specify: technical spec for the Workspace layer (registry format, pointer index, session concurrency, new commands)
- [ ] Implement: methodology doc(s), templates, new operation prompts + 3 tool ports (`.cursor`/`.claude`/`.opencode`)
- [ ] Verify: no-regression check for single-repo use; persona §6 Sensor; CHANGELOG

## Quality Checks

- [ ] Manual review of the diff (no quality-gate commands exist for this repo)
- [ ] Sync rule honored: every new/changed source prompt mirrored in the 3 tool ports
- [ ] Persona §6 output checklist run as a separate critique step (methodology design craft)
- [ ] Documentation updated (README, CHANGELOG, AGENTS/CLAUDE where applicable)
- [ ] No debug content or TODOs left behind

## Session Log

| Timestamp | Operation | Summary |
|-----------|-----------|---------|
| 2026-10-02 17:40 | start-session | Session opened. Kickoff triage: `intake/Backlog/prd.md` (ready) promoted → O-010 (→024); `Discovery/persona-ex-post-evaluation.md` still draft. Engrama fold of 014 (verbatim append to `history/.archived/memory.md`); archive op a no-op (021–023 + 024 in `history/`). |
| 2026-10-02 17:50 | direct-task | Removed the stray History Index row from the `## 013` section of `history/.archived/memory.md` (023 fold-script leftover); 013 Engrama row preserved. |
| 2026-10-02 18:00 | end-session | Session closed in `review` (developer's choice): only kickoff triage (O-010 promoted) and cold-store housekeeping (014 fold; stray 013 row removed) were done; Analyze/Plan not started, 0/8 criteria. Close triage: no `ready` items (`prd.md` triaged, Discovery item draft). No `[INCIDENT]` entries. Archive op a no-op (024 in `review` is not closed). |
| 2026-10-02 17:48 | start-session | Session reopened: 024 resumed from `review` → `in-progress` (no new NNN — O-010's destination). Kickoff triage: no `ready` items (`prd.md` triaged → O-010; Discovery `persona-ex-post-evaluation.md` draft; QA empty). Engramas: no new row, no fold (10 active = N). Archive op a no-op (021–023 closed + 024 open). Lifecycle gap captured as draft `intake/Backlog/review-iteration-lifecycle.md`. Time read via `date`; the 17:50/18:00 entries above post-date the 17:42 file writes, so they were estimates. |
| 2026-10-02 18:04 | analyze (resume-session) | Phase 1 done → `history/024-multi-repo-workspace-analysis.md`. Internal map: 11 single-repo assumptions (A1–A11) with anchors; measured the 6 adopters under `D:\development\public` (≥4 methodology versions; 3/6 memories lack Engramas; memories total 131,949 bytes, `stand-in` 77,789). Research in 8 fronts (F8 = contrary evidence); 15 dependency-ordered open questions for the Plan Q&A. Persona §6 critique (separate step): 7/7 PASS after 6 fixes (a "measured" budget claim reworded to documented defaults; the F4 implication labeled as inference by analogy; 2 anchors corrected; 1 link moved to its primary source; 1 INCONCLUSIVO added on member-command discovery). Usage-scenario refutation found the analysis unlinked (fixed in References) and an archive-op ambiguity (see Blockers & Notes). |
| 2026-10-04 11:14 | resume-session | Resumed; kickoff triage: no `ready` items. Resolved the two pending points (developer's choices): Engrama 024 recompiled via the `save-session` map-reduce (it still described the `review` close); analysis auxiliary kept in `history/` while 024 is open; the canon gap (auxiliary of an iteration without milestones) appended to the draft `intake/Backlog/review-iteration-lifecycle.md`, kept `draft`. |

## Decisions Made

| Decision | Rationale | Date |
|----------|-----------|------|
| Promote the multi-repo Workspace PRD as O-010 and take it as this session's work | Only `ready` intake item; matches the `feat/multi-context` branch; developer's choice | 2026-10-02 |
| Close the session in `review`, not `done`; keep 024 in Active Work (not Recent Completions) as O-010's destination | 0/8 criteria met — `end-session` mandates `review` when criteria are unmet; O-010 resumes under 024 | 2026-10-02 |
| Treat a `review` iteration as not closed for history archiving (021 stays in `history/`) | `history-archiving.md` keys `RAW_HISTORY` on **closed** iterations; 024 will be resumed | 2026-10-02 |
| Reopen 024 as the open session instead of creating 025 | O-010 resumes under 024 (decision at the 2026-10-02 close); `start-session` only defines creating a new NNN, so reopening a `review` iteration is a lifecycle gap — handled by judgment and captured as a draft intake item | 2026-10-02 |
| On reopen, recompile the Engrama row via the `save-session` map-reduce (not reset to `_In progress_`, not left as is) | The row still described the `review` close ("Analyze/Plan não iniciados"), false after the reopen; `save-session` already refreshes an open iteration's row (`prompts/operations/save-session.md:76-80`), so no new rule is invented | 2026-10-04 |
| Keep `024-multi-repo-workspace-analysis.md` in `history/` as a current auxiliary while 024 is open | 024 has no milestones, so the "current-milestone auxiliaries" rule (`methodology/history-archiving.md:76`) is silent; the analysis is the Plan Q&A's input. At 024's close it moves to `.archived/` like any closed iteration's auxiliary | 2026-10-04 |
| Route the canon gap to the draft `intake/Backlog/review-iteration-lifecycle.md` (kept `draft`), not fix it in 024 | Outside O-010's scope; the draft still has open shaping questions (which prompt owns the reopen path) | 2026-10-04 |

## Blockers & Notes

- The PRD requires one-question-at-a-time exhaustive Q&A before any design is settled. Questions the repository can answer are answered by exploration, not asked.
- The cold-store ledger `history/.archived/memory.md` had a stray History Index row (`| 013 | flow-packages | feature | done | … |`) inside the `## 013` section, left over from the 023 fold-script incident. Removed at the developer's request (scoped to the `## 013` section; the 013 Engrama row is intact).
- `history-archiving.md` keeps in `history/` only the open iteration's central file and its **current-milestone** auxiliaries; it does not say what happens to an auxiliary of an open iteration that has no milestones (e.g. `024-multi-repo-workspace-analysis.md`). Read literally, the next archive run would move that file to `.archived/`. Resolved 2026-10-04 (see Decisions Made): kept as a current auxiliary while 024 is open; the gap is appended to the draft `intake/Backlog/review-iteration-lifecycle.md`. When the file moves (at 024's close at the latest), its link in References must be repointed by hand — the archive op only repoints the History Index `File` cell.

## References

- **Analysis (Phase 1):** [`024-multi-repo-workspace-analysis.md`](024-multi-repo-workspace-analysis.md)
- **Specification:** — (to be written)
- **PR/MR:** —
- **Commits:** —
- **Related Issues:** O-010; `intake/Backlog/prd.md`; precedent `D:\development\public\stand-in`
