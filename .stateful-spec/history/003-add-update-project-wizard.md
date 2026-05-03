# Iteration: 003 — Add update-project initialization wizard

> One file per feature, bugfix, or refactor. Track progress and decisions here.

## Metadata

- **Type:** feature
- **Status:** done
- **Created:** 2026-04-03
- **Completed:** 2026-04-03
- **Author:** (project)

## Description

Add an initialization-style wizard for **refreshing** Stateful Spec in projects that already have `.stateful-spec/`, without treating it like first-time onboarding. Ships `prompts/initialization/update-project.md` (persona, step-by-step flow, selectable options), and wires discovery from `README.md` and `AGENTS.md`.

## Acceptance Criteria

- [x] `prompts/initialization/update-project.md` exists with frontmatter, methodology source, safe-update principles, and STEPs 0–8
- [x] README documents the path for “updating in an existing project”
- [x] AGENTS.md points maintainers at the prompt for upstream refreshes

## Implementation Tasks

- [x] Author `prompts/initialization/update-project.md`
- [x] Add README “Get Started” / discovery row for update flow
- [x] Add AGENTS.md pointer to `update-project.md`

## Quality Checks

- [x] Manual review of cross-references and links

## Decisions Made

| Decision | Rationale | Date |
|----------|-----------|------|
| Retroactive iteration 003 | Substantial work landed without a prior history file; record for audit and `save-session` STEP 2.5 | 2026-04-03 |

## Blockers & Notes

- _(none)_

## References

- **Specification:** _(none — prompt-led work)_
- **Files:** `prompts/initialization/update-project.md`, `README.md`, `AGENTS.md`
- **Commits:** Subject `Add update-project initialization wizard` — resolve hash with `git log -1 --oneline` on the integrating branch (hash not embedded; same commit would change the hash).
