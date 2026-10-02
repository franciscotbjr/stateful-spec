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

- [ ] Analyze: web research on multi-repo / workspace context management and pointer-based memory indexing (validated references)
- [ ] Analyze: map current single-repo assumptions in `methodology/`, `prompts/`, and `templates/`
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

## Decisions Made

| Decision | Rationale | Date |
|----------|-----------|------|
| Promote the multi-repo Workspace PRD as O-010 and take it as this session's work | Only `ready` intake item; matches the `feat/multi-context` branch; developer's choice | 2026-10-02 |

## Blockers & Notes

- The PRD requires one-question-at-a-time exhaustive Q&A before any design is settled. Questions the repository can answer are answered by exploration, not asked.
- The cold-store ledger `history/.archived/memory.md` had a stray History Index row (`| 013 | flow-packages | feature | done | … |`) inside the `## 013` section, left over from the 023 fold-script incident. Removed at the developer's request (scoped to the `## 013` section; the 013 Engrama row is intact).

## References

- **Specification:** — (to be written)
- **PR/MR:** —
- **Commits:** —
- **Related Issues:** O-010; `intake/Backlog/prd.md`; precedent `D:\development\public\stand-in`
