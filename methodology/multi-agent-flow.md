# Multi-Agent Flow (optional mode)

> **Applies to: software.** An optional, additive mode layered on top of the standard
> 5-phase cycle. It does **not** change how single-agent Stateful Spec works — if you
> never start a multi-agent flow, nothing here applies. Skills/studies are out of scope
> for now (see [`project-types.md`](project-types.md)).

This mode formalizes a **two-agent autonomous flow** for one long task delivered in many
milestones. Two agents (possibly different tools) take turns through a shared **state
file**, so the work proceeds without a human relaying each step.

## When to use it

Use it when a single work unit is too large to implement reliably in one long run — the
failure mode where an agent produces a façade that never actually runs. The proven remedy
(see the precedent below) is to **break the task into milestones**, give each milestone its
own spec, implementation, quality gate, and code review, and only advance when a reviewer
is satisfied.

> **Precedent.** This protocol generalizes the pattern that the `stand-in` project's
> `stand-in-client` SDK iteration ran by hand after three failed single-run attempts:
> a planner/reviewer agent broke the SDK into milestones M1…M10, wrote a spec per milestone
> and a review handoff per delivery; an implementer agent built each milestone, ran the
> full quality gate, and fixed findings — looping until approved, then committing.

For small or medium work, do **not** use this mode — the ordinary single-agent cycle is
lighter.

## Roles

| Role | Flag | Responsibilities |
|------|------|------------------|
| **Agent 1 — PM / Architect** | `pm` | Analyze + Plan (the master plan = milestone breakdown), Specify each milestone (via [`create-technical-spec`](../prompts/operations/create-technical-spec.md)), and **code-review** each delivery (via [`review-changes`](../prompts/operations/review-changes.md)), producing a review handoff. |
| **Agent 2 — Senior Engineer** | `engineer` | Implement each milestone (Phase 4 + the project's quality gate), **fix** review findings, and create the branch + commit per approved milestone (via [`write-commit-message`](../prompts/operations/write-commit-message.md)). |
| **Human** | — | Gives the task, **approves the milestone plan once**, and authorizes irreversible / outward-facing actions (push, open PR, publish, release). May interrupt at any time. |

This mode does not replace [`roles.md`](roles.md); it adds two AI roles and narrows the
human's in-loop involvement to plan approval + irreversible-action gates. The human still
owns all decisions — approving the plan is the decision; the agents only execute it.

## The signal file — `.stateful-spec/flow-state.md`

A single state file is the **authoritative signal** for whose turn it is and where the flow
stands. The spec files and review handoffs carry the substance; this file carries only the
coordination state. It lives at `.stateful-spec/flow-state.md` (one active flow at a time,
mirroring the single Open Session). Scaffold:
[`templates/implementation/flow-state.md`](../templates/implementation/flow-state.md).

```yaml
---
process_status: PLANNING | AWAITING_PLAN_APPROVAL | RUNNING | BLOCKED | DONE
iteration: NNN-<name>          # the umbrella iteration this flow drives
total_milestones: <n>          # set once the plan is approved
current_milestone: <k>         # 1-based
turn: pm | engineer            # whose turn it is right now
step: SPEC_PENDING | SPEC_READY | AWAITING_REVIEW | CHANGES_REQUESTED | APPROVED | COMMITTED
review_round: <int>            # rounds for the current milestone (stall guard)
max_review_rounds: 3           # cap before BLOCKED (configurable)
blocked_reason: <text|null>
done: false
updated_by: pm | engineer
updated_at: <ISO-8601>
---
```

The Markdown body below the frontmatter is an **append-only turn-log**: one line per turn
(`timestamp · role · action · file touched`). It is the human-readable trace of the flow.

## The state machine

Each milestone advances through explicit steps. The agent whose `turn` matches its role
acts; the other waits.

```
PLANNING ─(PM writes master plan)→ AWAITING_PLAN_APPROVAL ─(human approves)→ RUNNING

  SPEC_PENDING      (PM writes milestone spec)        ──────────────→ SPEC_READY      (turn=ENG)
  SPEC_READY        (ENG implements + quality gate)   ──────────────→ AWAITING_REVIEW (turn=PM)
  AWAITING_REVIEW   (PM reviews → handoff) ──┬── pass ──────────────→ APPROVED        (turn=ENG)
                                             └── fail ──────────────→ CHANGES_REQUESTED(turn=ENG, review_round++)
  CHANGES_REQUESTED (ENG fixes + quality gate)        ──────────────→ AWAITING_REVIEW (turn=PM)
  APPROVED          (ENG creates branch [first M] + commits) ───────→ COMMITTED
  COMMITTED         → next milestone (SPEC_PENDING, turn=PM)  |  last milestone → process DONE

  review_round > max_review_rounds  ⇒  BLOCKED (blocked_reason set) — halt for the human
```

- **`PLANNING` → `AWAITING_PLAN_APPROVAL`**: the PM produces the master plan (the milestone
  checklist) and stops for the one human gate. On approval the human (or PM, once approved)
  sets `process_status: RUNNING`, `total_milestones`, `current_milestone: 1`,
  `step: SPEC_PENDING`, `turn: pm`.
- **`BLOCKED`**: any agent may set this with a `blocked_reason` when it needs the human —
  an unresolved question, a stall (review cap exceeded), or an **irreversible/outward-facing
  action** (push, PR, publish). The flow resumes when the human clears the block.
- **`DONE`**: terminal and **sticky**. Both agents, on their next poll, observe `DONE` and
  exit their loops. The file persists as the record until a new flow resets it.

## File layout per flow

Flat in `.stateful-spec/history/` (the proven layout — no per-milestone subfolders, no
duplicated milestone lists):

```
.stateful-spec/
├── flow-state.md                        # the coordination signal (this flow)
└── history/
    ├── NNN-<name>.md                     # master plan = ONE iteration; milestone checklist M1..Mn
    │                                     #   (single source of truth — never duplicate the list)
    ├── NNN-<name>-m1-spec.md             # PM, Specify phase, milestone 1
    ├── handoff-NNN-<name>-m1-review.md   # PM, review of ENG's milestone-1 delivery
    ├── NNN-<name>-m2-spec.md
    ├── handoff-NNN-<name>-m2-review.md
    └── ...
```

- The **master plan** (`NNN-<name>.md`) is the umbrella iteration and the **single source**
  for the milestone list. Detailed milestone specs **refine** an item; they never re-list
  the milestones.
- Each milestone spec uses [`create-technical-spec`](../prompts/operations/create-technical-spec.md)
  output; each review handoff uses
  [`templates/implementation/review-handoff.md`](../templates/implementation/review-handoff.md).

## Sessions and memory

One flow = one **umbrella iteration** (one `NNN`, one History Index row, one Engrama row).
Within it, **each milestone is its own session** (`SPEC_PENDING` → … → `COMMITTED`):

- The `pm` kickoff initializes the umbrella iteration, the Open Session, and `flow-state.md`.
- The `engineer` kickoff detects the existing flow and **attaches** — it never creates a
  competing iteration (per the existing Open Session rule in [`roles.md`](roles.md)).
- Each milestone's events (spec written, implementation, each review verdict, fixes, commit)
  are Session Log entries in the **master iteration file**, tagged by role.
- Because the protocol is **turn-based**, only the agent whose turn it is ever writes — the
  two agents never collide editing `memory.md` or the iteration file.
- **History Index and Engramas stay per umbrella iteration** (one row each). On `DONE`, the
  Engrama is compiled from all milestone session logs via the usual map-reduce — no per-
  milestone Engrama rows. See the Engramas maintenance rule in `AGENTS.md`.

## How the agents run (the loop)

Each agent is started once via [`start-multi-agent-flow`](../prompts/operations/start-multi-agent-flow.md)
with its role and then **polls** `flow-state.md`:

1. Read `flow-state.md`.
2. If `process_status` is `DONE` or `BLOCKED` → report and **exit** the loop.
3. If `turn` ≠ my role → **re-schedule a wake** (sleep) and stop for now.
4. If `turn` == my role → perform the action for the current `step` (delegating to the
   right existing operation), update the frontmatter (flip `turn`/`step`, bump
   `review_round` if applicable, set `updated_by`/`updated_at`), append a turn-log line,
   then re-schedule a wake to monitor for the next turn.

The polling mechanism is tool-specific (in Claude Code, the `/loop` skill or scheduled
wake-ups; other tools use their own loop/watch). The **protocol is identical regardless of
trigger** — a human can also relay turns manually by invoking each agent in sequence, since
the file contract is the same.

## Stall handling

`review_round` counts review cycles for the current milestone (one round =
`AWAITING_REVIEW` → `CHANGES_REQUESTED` → fix → `AWAITING_REVIEW`). When it exceeds
`max_review_rounds` (default 3), the PM sets `process_status: BLOCKED` with a
`blocked_reason` and halts for the human instead of looping indefinitely. This encodes the
core lesson behind this mode: bound the work, never run unbounded.

## Branches, commits, and human gates

- **One feature branch** for the whole umbrella iteration; the Engineer creates it at the
  **first** approved milestone and adds **one commit per approved milestone** thereafter.
  Each commit leaves the repository green (the quality gate runs at the end of every
  milestone — never deferred to the end of the flow).
- **Commit to the local feature branch is pre-authorized** inside the flow.
- **Irreversible / outward-facing actions are NOT** pre-authorized: pushing, opening a PR on
  a shared remote, publishing, or releasing. When the flow reaches such a step, the agent
  sets `BLOCKED` (reason) and waits for explicit human authorization, consistent with
  [`roles.md`](roles.md) (the AI does not make irreversible changes without instruction).

## Completion

When the last milestone reaches `COMMITTED`, the PM sets `process_status: DONE` and
`done: true`, writes a final turn-log summary, runs `end-session` for the final milestone,
and closes the umbrella iteration (move to Recent Completions, set History Index status
`done`, compile the Engrama). Delivery (push/PR/publish) remains a human gate. `DONE` is
sticky; the file persists as the record of the completed flow until a new flow resets it.

## Relationship to the 5 phases and existing operations

This mode **reuses**, it does not replace:

| Flow step | Phase | Operation reused |
|-----------|-------|------------------|
| Master plan (milestones) | Analyze + Plan | the planning phases ([`phases/01-analyze.md`](phases/01-analyze.md), [`phases/02-plan.md`](phases/02-plan.md)) |
| Milestone spec | Specify | [`create-technical-spec`](../prompts/operations/create-technical-spec.md) |
| Milestone implementation + gate | Implement + Verify | [`phases/04-implement.md`](phases/04-implement.md), [`phases/05-verify.md`](phases/05-verify.md) |
| Code review (handoff) | Verify | [`review-changes`](../prompts/operations/review-changes.md) |
| Commit | Verify | [`write-commit-message`](../prompts/operations/write-commit-message.md) |
| Session bookkeeping | — | [`start-session`](../prompts/operations/start-session.md) / [`end-session`](../prompts/operations/end-session.md) (milestone-session variant) |

## Anti-patterns

- **Using it for small work** — overhead with no benefit; use the single-agent cycle.
- **Duplicating the milestone list** — the master plan is the only source; specs refine, they
  don't re-list (a real cause of confusion in the precedent).
- **Façade milestones** — marking a milestone done without a green quality gate and a real
  review; the gate runs every milestone, the review is mandatory.
- **Skipping the human gate on irreversible actions** — never push/PR/publish autonomously.
- **Two agents writing at once** — the turn flag is authoritative; act only on your turn.
- **Unbounded review loops** — respect `max_review_rounds`; escalate via `BLOCKED`.
