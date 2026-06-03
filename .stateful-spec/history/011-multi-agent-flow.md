# Iteration: 011 — Multi-Agent Flow

> One file per feature, bugfix, or refactor. Track progress and decisions here.

## Metadata

- **Type:** feature
- **Status:** done
- **Created:** 2026-06-03
- **Completed:** 2026-06-03
- **Author:** Francisco Tarcizo Bomfim Júnior

## Description

Formalize a **two-agent autonomous flow** in the methodology, reproducing the pattern that
iteration 011 of the downstream `stand-in` project (`stand-in-client` SDK) executed by hand
after 3 failed attempts at doing a large task in one long run. **Agent 1 (PM/Architect)**
breaks a long task into milestones (M1…Mn), writes one tech-spec per milestone, and code-
reviews each delivery; **Agent 2 (Senior Engineer)** implements each spec, runs the quality
gate, and fixes review findings. The cycle `spec → implement → review → fix` loops until
Agent 1 approves, then Agent 2 commits; repeat across all milestones until done. The two
agents coordinate through a **dedicated state file** (`.stateful-spec/flow-state.md`) so any
two different agents/tools can play the roles. The change is **additive** — it adds a
protocol doc, one operation (+ ports), and templates, reusing the existing 5-phase cycle and
operations; it does not alter single-agent behavior.

Scope: **software-only** for now (operations tagged "Applies to: software"); generalization
to skills/studies is deferred.

## Acceptance Criteria

> Checkboxes for what "done" means.

- [x] `methodology/multi-agent-flow.md` defines the protocol (roles, state machine, signal
      file schema, milestone file layout, session-per-milestone, stall cap, branch/commit,
      completion) and is marked "Applies to: software".
- [x] `prompts/operations/start-multi-agent-flow.md` exists (arg `pm|engineer`), reuses
      existing ops (`create-technical-spec`, `review-changes`, Phase 4, `write-commit-message`),
      and is mirrored to `.claude/`, `.cursor/` (with frontmatter), `.opencode/`.
- [x] `templates/implementation/flow-state.md` and `templates/implementation/review-handoff.md`
      added.
- [x] `methodology/roles.md` gains an additive "Multi-Agent Flow (optional mode)" subsection;
      `start-session.md`/`end-session.md` (+ 3 ports each) gain an additive milestone-session note.
- [x] The op is surfaced in `methodology/project-types.md` (software), root `AGENTS.md`,
      `CLAUDE.md`, `README.md`, `templates/project/agents-md.md`, and `CHANGELOG.md`.
- [x] Sync rule satisfied: the new op + the start/end-session edits appear in all three ports.
- [x] Protocol dry-run against `stand-in` 011 (M1→M2, plus the irreversible-publish gate)
      confirms the state machine reproduces the manual flow.

## Implementation Tasks

> Breakdown of work. Check off as you go.

- [x] Create iteration 011 + update `memory.md` (Active Work, History Index, Engramas)
- [x] Write `methodology/multi-agent-flow.md`
- [x] Write `prompts/operations/start-multi-agent-flow.md` + 3 ports
- [x] Add `templates/implementation/flow-state.md` + `templates/implementation/review-handoff.md`
- [x] Additive edits: `roles.md`, `start-session`/`end-session` (+ ports), `project-types.md`
- [x] Surface op in `AGENTS.md`, `CLAUDE.md`, `README.md`, `agents-md.md` template, `CHANGELOG.md`
- [x] Verify (sync rule, table consistency, dry-run vs 011, minimal-diff)

## Quality Checks

> Standard checks from the Project Definition. Verify before marking done.

- [x] All quality gates pass (manual review — documentation-only repo)
- [x] Sync rule honored across `.cursor/`, `.claude/`, `.opencode/` ports (verified: ports identical to source except `.cursor` frontmatter; session notes additive 7/0 + 9/0)
- [x] Documentation updated (README, CHANGELOG, AGENTS/CLAUDE tables)
- [x] No duplicated milestone lists; no `flow-state` inside `memory.md`; phases untouched (0 deletions on methodology/doc edits)

## Session Log

> Timestamped entries recording each operation performed during this session.

| Timestamp | Operation | Summary |
|-----------|-----------|---------|
| 2026-06-03 | resume-session | Loaded context; executed `prompt.md` via plan mode — exhaustive 14-decision Q&A; plan approved. |
| 2026-06-03 | start (direct-task) | Created iteration 011; updated memory.md Active Work / History Index / Engramas. |
| 2026-06-03 | implement | Wrote `methodology/multi-agent-flow.md`; `start-multi-agent-flow` op + 3 ports; `flow-state.md` + `review-handoff.md` templates. |
| 2026-06-03 | implement | Additive edits: `roles.md`, `start-session`/`end-session` (+ 3 ports each), `project-types.md`; surfaced op in `AGENTS.md`/`CLAUDE.md`/`README.md`/`agents-md.md`/`CHANGELOG.md`. |
| 2026-06-03 | review-changes | Verified sync rule (ports == source bar frontmatter), table consistency, additive-only diffs (0 deletions), dry-run vs stand-in 011. Clean. |
| 2026-06-03 | end-session | Closed iteration 011; all acceptance criteria met. |

## Decisions Made

> Decisions made during this iteration. Include rationale. (Full alternatives table in the
> approved plan; condensed here.)

| Decision | Rationale | Date |
|----------|-----------|------|
| D1 Autonomous polling loop | Matches "fluxo autônomo" + "any two different agents" (file is the contract; loop is the trigger). | 2026-06-03 |
| D2 Approve plan once, then run; stop at irreversible | Mirrors 011 (publish was human-gated); reconciles with "human owns decisions". | 2026-06-03 |
| D3 Software-only for now | Smaller blast radius; defer skills/studies. | 2026-06-03 |
| D4 Dedicated state file + artifacts | Explicit turn-ownership; easy done/stall detection. | 2026-06-03 |
| D5 Fine-grained state machine | Unambiguous turn detection; review_round stall cap; clear DONE. | 2026-06-03 |
| D6 Singleton `.stateful-spec/flow-state.md` (frontmatter+log) | Mirrors single Open Session; consistent with Markdown+frontmatter convention. | 2026-06-03 |
| D7 New `methodology/multi-agent-flow.md` + pointer in roles.md | Clean separation; minimal edit to roles.md; discoverable. | 2026-06-03 |
| D8 Flat in `history/` (master plan + mK-spec + handoff) | Proven in 011; minimal change to conventions. | 2026-06-03 |
| D9 One op `start-multi-agent-flow <pm\|engineer>` | Matches prompt hypothesis; one source + one port-set. | 2026-06-03 |
| D10 One session per milestone | Developer choice; each milestone is a self-contained cycle. | 2026-06-03 |
| D10b Umbrella = 1 iteration / 1 Engrama row | Keeps History Index/Engramas at current granularity (minimal change). | 2026-06-03 |
| D11 Sticky DONE; file persists as record | No deletion race against a polling agent; clear historical record. | 2026-06-03 |
| D12 Cap review_round (default 3) → BLOCKED + escalate | Bounded; 011 lesson about unbounded runs. | 2026-06-03 |
| D13 Web investigation waived | Developer explicitly chose "ignore" (see Blockers & Notes). | 2026-06-03 |
| D14 One feature branch; one commit per milestone | Matches 011 incremental commits + "main + feature branches via PRs". | 2026-06-03 |

## Blockers & Notes

- **Web-investigation waiver (D13):** `prompt.md` §Investigação asked to research
  "information-summarization models on the web relentlessly" — phrasing copy-pasted from the
  008/engramas prompt and not relevant to this coordination problem. When asked to confirm
  the target, the developer answered **"ignore"**. The mandatory web investigation is
  therefore **formally waived by the developer**, recorded honestly here (the 009 review-
  handoff established that an explicit, documented dispensation is acceptable). No external-
  source alternatives table is produced; design alternatives are documented in the approved
  plan's Decision Log.
- Turn-based protocol ⇒ only the agent whose turn it is writes, so two agents never collide
  editing `memory.md`/the iteration file.

## References

- **Specification:** approved plan `C:\Users\franciscotbjr\.claude\plans\kind-growing-hartmanis.md`; source request `prompt.md`
- **Real-world precedent:** `D:\development\public\stand-in\.stateful-spec\history\011-stand-in-client-sdk.md` (+ its `mK-spec` and `handoff-mK-review` files); strategy note `Revisar estratégia stand-in-client.md`
- **PR/MR:** —
- **Commits:** —
- **Related Issues:** —
