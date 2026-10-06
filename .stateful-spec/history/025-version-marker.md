# Iteration: 025 — Version Marker

> One file per feature, bugfix, or refactor. Track progress and decisions here.

## Metadata

- **Type:** feature
- **Status:** in-progress
- **Created:** 2026-10-05
- **Completed:** —
- **Author:** Francisco Tarcizo Bomfim Júnior

## Description

O-010 milestone **M1**: add a dedicated **methodology version marker** to Stateful Spec projects.
The design decisions come from iteration 024 (resolve via the History Index): **Plan Q6** (a
Workspace member must be at the current methodology version, which needs a recorded version),
**Plan Q6b** (a new dedicated marker file, numbered like `CHANGELOG.md`; the release that
introduces it is 3.0.0; a project without the marker counts as pre-3.0.0, i.e. ≤ 2.0.0) and
**Plan R6** (the reference for "current" is the Workspace's own marker). Q6b leaves three items
for Specify: the marker's file name and location, the methodology repo's canonical source for
the current version, and which wizards write it (`new-project`, `onboard-existing`,
`update-project`). The marker is a new, additive file. It must not change single-repo behavior
(PRD AC1).

## Acceptance Criteria

> Draft derived from 024 Plan Q6/Q6b/R6. Confirm or revise at Specify.

- [ ] Marker file name and location decided and recorded with the alternatives not taken (Q6b open item)
- [ ] The methodology repo has one canonical source for the current version, aligned with `CHANGELOG.md` numbering (Q6b open item)
- [ ] `new-project`, `onboard-existing` and `update-project` write or refresh the marker (Q6b open item)
- [ ] The no-marker case (implicitly ≤ 2.0.0) and the "current version" comparison rule (R6) are documented
- [ ] AC1 — no regression: an already-configured single-repo project works exactly as before; the marker is additive
- [ ] Sync rule honored for any changed source prompt; `CHANGELOG.md` `[Unreleased]` updated (cutting/tagging 3.0.0 stays a human gate after M7)

## Implementation Tasks

- [ ] Specify: resolve the three Q6b open items (name/location, canonical source, writers)
- [ ] Implement: marker in the core templates and the canonical source in this repo
- [ ] Implement: initialization wizards write/refresh the marker (+ ports where they exist)
- [ ] Implement: document the no-marker case and the R6 comparison rule
- [ ] Verify: AC1 dry run reasoning on an existing single-repo adopter; persona §6 on the deliverable

## Quality Checks

- [ ] Manual review of the diff (no quality-gate commands exist for this repo)
- [ ] Sync rule honored: every new/changed source prompt mirrored in the 3 tool ports
- [ ] Documentation updated (if applicable)
- [ ] No debug code or TODOs left behind

## Session Log

| Timestamp | Operation | Summary |
|-----------|-----------|---------|
| 2026-10-05 20:49 | start-session | Session opened for O-010 M1 (version marker; 024 Plan Q6, Q6b, R6). Kickoff triage: no `ready` intake items (`prd.md` triaged; `review-iteration-lifecycle.md`, `workspace-upstream-contribution.md`, `persona-ex-post-evaluation.md` draft). Engrama 015 folded into `0-archived` (verbatim copy in `history/.archived/memory.md`). Archive step: no-op (`history/` holds 022–024 + 025). |

## Decisions Made

| Decision | Rationale | Date |
|----------|-----------|------|

## Blockers & Notes

- Design source: iteration 024 **Decisions Made** — cite rows (e.g. "024 Plan Q6b") instead of re-reading the whole central file (~51 KB).

## References

- **Specification:** —
- **PR/MR:** —
- **Commits:** —
- **Related Issues:** O-010 (`.stateful-spec/backlog.md`); `intake/Backlog/prd.md`
