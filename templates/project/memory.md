# Project Memory

> This file is the AI's entry point for understanding the project's current state. Keep it updated as work progresses.

## Project Summary

- **Project:** [Project name]
- **Description:** [One-sentence description]
- **Last Updated:** [YYYY-MM-DD]
- **Current Status:** [e.g., Active development, Maintenance, Planning]

## Active Work

> What is currently in progress? Reference the iteration file.

- [ ] [Brief description] → `history/NNN-name.md`

## Open Session

> When a session is active, this section points to the current iteration file.
> Managed by `start-session` and `end-session` — do not edit manually.
>
> **Relationship to Active Work:** Open Session holds at most one iteration (the current implementation cycle). Active Work lists all work items in progress — the Open Session entry should also appear in Active Work while open. `end-session` moves it from Active Work to Recent Completions and clears Open Session.

_(none)_

## Recent Completions

> Last 3-5 completed iterations for quick context.

| # | Name | Type | Completed |
|---|------|------|-----------|
| [NNN] | [Name] | [feature/bugfix/refactor] | [YYYY-MM-DD] |

## Key Decisions

> Important decisions that affect how the AI should work on this project. For detailed ADRs, see `history/` files or `docs/adr/`.

- [Decision summary] — [Brief rationale]

## Constraints & Reminders

> Things the AI must always remember when working on this project.

- [e.g., Must support Node 18+]
- [e.g., No new dependencies without discussion]
- [e.g., All public APIs must have JSDoc]

## History Index

> Complete list of iterations. Newest first. The `File` cell may point into `history/.archived/`
> for older iterations — archived rows stay resolvable but are never bulk-read (see
> `methodology/history-archiving.md`).

| # | Name | Type | Status | File |
|---|------|------|--------|------|
| [NNN] | [Name] | [feature/bugfix/refactor] | [done/in-progress/blocked] | `history/NNN-name.md` |

## Engramas

> Compiled memory from history. The agent reads this section for context without
> loading all `history/` files. Each row links to the History Index via `#`.
> If the engram is insufficient, the agent should consult the corresponding
> history file.
>
> **Two-tier compaction:** The table holds at most `N` active rows (default 10)
> plus one Archive row. N is the number after this comment. When the active row
> count exceeds N, the oldest row merges into the Archive row at the bottom —
> first appending that row's full content verbatim to `history/.archived/memory.md`
> so the fold loses no detail (see `methodology/history-archiving.md`).
> Adjust N to match your context budget — smaller N = less context, more loss.

<!-- N = 10 -->

| # | Summary | Key Decisions | Learnings |
|---|---------|---------------|------------|
| [NNN] | [1-2 sentences of what was done] | [— or relevant decisions, 1 line each] | [— or blockers/insights, 1 line each] |
| 0-archived | [summary of older iterations] | [—] | [—] |

## How to Use This File

1. **AI assistants:** Read this file first when joining the project. It provides context about what's happening and what to remember.
2. **Developers:** Update this file when starting or completing work. Keep the Active Work and History Index current.
3. **New team members:** This file + the Project Definition give you everything needed to onboard an AI assistant.
