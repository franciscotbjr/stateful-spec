# Multi-Agent Flow (optional mode)

> **Applies to: software.** An optional, additive mode layered on top of the standard
> 5-phase cycle. It does **not** change how single-agent Stateful Spec works — if you
> never start a multi-agent flow, nothing here applies. Skills/studies are out of scope
> for now (see [`project-types.md`](project-types.md)).

This mode formalizes an **autonomous multi-agent flow** for one long task delivered in many
milestones. Agents (possibly different tools) take turns through a shared **state file**, so the
work proceeds without a human relaying each step. It comes in two variants:

- **Two-agent flow (lighter)** — a PM/Architect and a Senior Engineer. Use it for most large,
  milestone-delivered work.
- **Three-agent flow (more rigorous)** — adds a **read-only Architect reviewer** that gates each
  milestone **spec** (a *design gate*) before any code is written, plus per-profile engineer
  personas and a hand-back safety valve. Use it when design-level defects are costly — e.g.
  UI or cross-cutting work where a spec-level gate catches mis-scoped or under-specified milestones
  before they are implemented.

Both variants share the same signal file, the same atomic-verb contract, the same blocking-poll
loop, and the same human gates. The three-agent flow only **adds** one role and one design-gate
loop on top of the two-agent flow.

## When to use it

Use it when a single work unit is too large to implement reliably in one long run — the
failure mode where an agent produces a façade that never actually runs. The proven remedy
is to **break the task into milestones**, give each milestone its own spec, implementation,
quality gate, and code review, and only advance when a reviewer is satisfied.

> **Precedent.** This protocol generalizes a pattern proven by hand on a large SDK delivery after
> several failed single-run attempts: a planner/reviewer agent broke the work into milestones,
> wrote a spec per milestone and a review handoff per delivery; an implementer agent built each
> milestone, ran the full quality gate, and fixed findings — looping until approved, then committing.

For small or medium work, do **not** use this mode — the ordinary single-agent cycle is lighter.

Reach for the **three-agent** flow over the two-agent flow when a *delivery* review is too late: if
specs tend to ship mis-scoped or under-specified and the cost surfaces only in testing, a reviewer
that reads the **design** (the spec) before a line is written is the gate that catches it.

## Roles

| Role | `turn` | Spawned? | Responsibilities |
|------|--------|----------|------------------|
| **Agent 1 — PM / Architect-planner** | `pm` | no (driver) | Master plan (the milestone breakdown); **drafts** each milestone spec (via [`create-technical-spec`](../prompts/operations/create-technical-spec.md)); **code-reviews** each delivery (via [`review-changes`](../prompts/operations/review-changes.md)), producing a review handoff. |
| **Agent 2 — Senior Engineer** | `engineer` | yes | Implements each milestone (Phase 4 + the project's quality gate), **fixes** review findings, and creates the branch + commits per approved milestone (via [`write-commit-message`](../prompts/operations/write-commit-message.md)). |
| **Agent 3 — Architect reviewer** *(three-agent flow only)* | `architect` | yes | **Read-only.** Reviews the milestone **spec** (the design gate) against known failure classes, looping with the PM until the design is approved. Reads the spec + repo only — never edits code, never runs the build. |
| **Human** | — | — | Gives the task, **approves the milestone plan once**, and authorizes irreversible / outward-facing actions (push, open PR, publish, release). May interrupt at any time. |

This mode does not replace [`roles.md`](roles.md); it adds AI roles and narrows the human's in-loop
involvement to plan approval + irreversible-action gates. The human still owns all decisions —
approving the plan is the decision; the agents only execute it. The boundaries in `roles.md` (no
irreversible actions without instruction; one writer per turn) apply to **every** agent; the
architect's read-only boundary is the strongest form of "the AI makes no changes without
instruction" — by construction it makes none.

In the three-agent flow the single Engineer role is realized by one of several **profiles** — a
persona per milestone, selected from the project's skill set by the milestone's domain (for example
a frontend profile vs. a backend/library profile). The **PM keeps the delivery review**; the
architect owns only the pre-implementation spec gate.

## The signal file — `.stateful-spec/flow-state.md`

A single state file is the **authoritative signal** for whose turn it is and where the flow stands.
The spec files and review handoffs carry the substance; this file carries only the coordination
state. It lives at `.stateful-spec/flow-state.md` (one active flow at a time, mirroring the single
Open Session). Scaffold: [`templates/implementation/flow-state.md`](../templates/implementation/flow-state.md).

```yaml
---
process_status: PLANNING | AWAITING_PLAN_APPROVAL | RUNNING | BLOCKED | DONE
iteration: NNN-<name>          # the umbrella iteration this flow drives
total_milestones: <n>          # set once the plan is approved
current_milestone: <k>         # 1-based
turn: pm | engineer | architect          # `architect` only in the three-agent flow
step: SPEC_PENDING | SPEC_READY | AWAITING_REVIEW | CHANGES_REQUESTED | APPROVED | COMMITTED
#  three-agent flow adds the design-gate steps: ARCH_REVIEW | SPEC_APPROVED | SPEC_CHANGES_REQUESTED
profile: <domain|null>         # three-agent flow: the current milestone's engineer profile
review_round: <int>            # delivery-gate rounds for the current milestone (stall guard)
max_review_rounds: 3           # delivery-gate cap before BLOCKED (configurable)
spec_review_round: <int>       # three-agent flow: design-gate rounds (reset per milestone)
max_spec_review_rounds: 3      # three-agent flow: design-gate cap
blocked_reason: <text|null>
done: false
updated_by: pm | engineer | architect
updated_at: <ISO-8601>
seq: <int>                     # monotonic; bumped on every transition (strictly chronological log)
---
```

> **Write-side mechanism.** Agents do **not** hand-edit this frontmatter — every transition goes
> through a **validated verb** (see *The coordination tool* below), which is the only writer. This
> makes illegal transitions (e.g. the engineer setting `SPEC_READY`, or the read-only architect
> running an engineer verb) impossible by construction, and keeps the turn-log strictly
> chronological via `seq`.

The Markdown body below the frontmatter is an **append-only turn-log**: one `#seq`-tagged line per
turn (`timestamp · role · action · file touched`). It is the human-readable trace of the flow.

## The state machine

Each milestone advances through explicit steps. The agent whose `turn` matches its role acts; the
others wait. The **two-agent flow** runs the delivery gate; the **three-agent flow** inserts a
design gate before the handoff to the engineer — the delivery side is identical.

```
PLANNING ─(PM master plan)→ AWAITING_PLAN_APPROVAL ─(human approves)→ RUNNING

  ── design gate (three-agent flow only) ─────────────────────────────────────
  SPEC_PENDING            (PM drafts the spec)                ──submit-spec(PM)──→ ARCH_REVIEW            (turn=architect)
  ARCH_REVIEW             (architect reviews spec, read-only) ──┬─ approve-spec ───────→ SPEC_APPROVED   (turn=pm)
                                                               └─ request-spec-changes ─→ SPEC_CHANGES_REQUESTED (turn=pm, spec_review_round++)
  SPEC_CHANGES_REQUESTED  (PM revises the spec)               ──submit-spec(PM)──→ ARCH_REVIEW            (turn=architect)   [loop]
  SPEC_APPROVED           (PM hands off)                      ──spec-ready(PM)───→ SPEC_READY             (turn=engineer)

  ── delivery gate (both flows) ──────────────────────────────────────────────
  SPEC_PENDING            (PM writes spec)        [two-agent] ──spec-ready(PM)───→ SPEC_READY            (turn=engineer)
  SPEC_READY              (ENG implements + gate)             ──submit(ENG)─────→ AWAITING_REVIEW        (turn=pm)
  SPEC_READY              (ENG judges it mis-scoped)          ──hand-back(ENG)──→ SPEC_PENDING           (turn=pm)   [three-agent flow]
  AWAITING_REVIEW         (PM reviews delivery → handoff) ──┬── approve ─────────→ APPROVED              (turn=engineer)
                                                           └── request-changes ──→ CHANGES_REQUESTED     (turn=engineer, review_round++)
  CHANGES_REQUESTED       (ENG fixes + quality gate)         ──submit(ENG)─────→ AWAITING_REVIEW        (turn=pm)
  APPROVED                (ENG creates branch [first M] + commits) ──advance(ENG)──→ COMMITTED
  COMMITTED               → next milestone (SPEC_PENDING, turn=pm)  |  last milestone → process DONE

  review_round      > max_review_rounds       ⇒  BLOCKED (delivery-gate stall — halt for the human)
  spec_review_round > max_spec_review_rounds  ⇒  BLOCKED (design-gate stall — three-agent flow)
```

- **`PLANNING` → `AWAITING_PLAN_APPROVAL`**: the PM produces the master plan (the milestone
  checklist) and stops for the one human gate. On approval the PM runs the **`approve-plan`** verb,
  which sets `process_status: RUNNING`, `total_milestones`, `current_milestone: 1`,
  `step: SPEC_PENDING`, `turn: pm` atomically.
- **Two independent, bounded loops** (three-agent flow), each with its own counter and cap: the
  **design-gate loop** (`SPEC_PENDING → ARCH_REVIEW → SPEC_CHANGES_REQUESTED → ARCH_REVIEW …`,
  counted by `spec_review_round`) and the **delivery-gate loop** (`AWAITING_REVIEW →
  CHANGES_REQUESTED → AWAITING_REVIEW …`, counted by `review_round`).
- **Hand-back** (three-agent flow) is a third, un-counted safety valve: when the engineer discovers
  the milestone is mis-scoped for its profile, it returns the milestone to `SPEC_PENDING/turn=pm`
  with a reason, and the PM re-scopes or re-profiles. A re-scope is a new spec, so the architect
  re-reviews it.
- **`BLOCKED`**: any agent may set this with a `blocked_reason` when it needs the human — an
  unresolved question, a stall (a review cap exceeded), or an **irreversible/outward-facing action**
  (push, PR, publish). The flow resumes when the human clears the block.
- **`DONE`**: terminal and **sticky**. Every agent, on its next poll, observes `DONE` and exits its
  loop. The file persists as the record until a new flow resets it.

## File layout per flow

Flat in `.stateful-spec/history/` (the proven layout — no per-milestone subfolders, no duplicated
milestone lists):

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

- The **master plan** (`NNN-<name>.md`) is the umbrella iteration and the **single source** for the
  milestone list. Detailed milestone specs **refine** an item; they never re-list the milestones.
- Each milestone spec uses [`create-technical-spec`](../prompts/operations/create-technical-spec.md)
  output; each review handoff uses
  [`templates/implementation/review-handoff.md`](../templates/implementation/review-handoff.md).
- **Milestone archiving (on advance).** When the flow advances past a milestone
  (`COMMITTED → SPEC_PENDING` for the next, via PM `advance`), run the idempotent archive operation
  in [`history-archiving.md`](history-archiving.md) for the **completed** milestones: move their
  `-mN-spec.md` + `handoff-…-mN-review*.md` into `history/.archived/`, keeping only the **current**
  milestone's auxiliaries and the umbrella's central `NNN-<name>.md`. The open milestone's files are
  never touched; an archived earlier-milestone file is still reachable by its exact indexed path if a
  later milestone needs it.

## Sessions and memory

One flow = one **umbrella iteration** (one `NNN`, one History Index row, one Engrama row). Within
it, **each milestone is its own session** (`SPEC_PENDING` → … → `COMMITTED`):

- The `pm` kickoff initializes the umbrella iteration, the Open Session, and `flow-state.md`.
- The `engineer` (and, in the three-agent flow, the `architect`) kickoff detects the existing flow
  and **attaches** — it never creates a competing iteration (per the Open Session rule in
  [`roles.md`](roles.md)).
- Each milestone's events (spec written, each review verdict, implementation, fixes, commit) are
  Session Log entries in the **master iteration file**, tagged by role.
- Because the protocol is **turn-based**, only the agent whose turn it is ever writes — the agents
  never collide editing `memory.md` or the iteration file.
- **History Index and Engramas stay per umbrella iteration** (one row each). On `DONE`, the Engrama
  is compiled from all milestone session logs via the usual map-reduce — no per-milestone Engrama
  rows. See the Engramas maintenance rule in `AGENTS.md`.
- **Process failures during the flow** (a spawned agent that ends without handing off, an orphaned
  worker, a commit on the wrong branch, a hand-back, a stall recovery) are logged as `[INCIDENT]`
  entries in the umbrella iteration's Session Log **at the moment they occur**, and swept into
  Engrama learnings at close — see the in-flow incident log in [`qa-phase.md`](qa-phase.md).

## How the agents run (the loop)

Each agent is started once via
[`start-multi-agent-flow`](../prompts/operations/start-multi-agent-flow.md) with its role and then
**polls** `flow-state.md`:

1. Read `flow-state.md`.
2. If `process_status` is `DONE` or `BLOCKED` → report and **exit** the loop.
3. If `turn` ≠ my role → **stand by on a blocking poll** that returns the instant the turn flips
   (not on a fixed timer). On a poll timeout, run the liveness / spawn-death watchdog and **re-poll**.
4. If `turn` == my role → perform the action for the current `step` (delegating to the right existing
   operation), then **run the matching verb** — it validates the precondition, flips `turn`/`step`,
   bumps the round counter and `seq`, sets `updated_by`/`updated_at`, and appends the turn-log line
   **atomically**. Then re-enter the blocking poll to monitor for the next turn.

The driver (PM) **spawns** the next agent on every flip to a spawned role and then blocks on the
poll. In the three-agent flow there is one extra spawn per milestone — the architect, spawned at
`ARCH_REVIEW`; everything else is identical.

The **primary** standby mechanism is the **blocking poll** (it wakes on the turn flip);
fixed-interval wake-ups (a scheduler, a `/loop`) are a fallback only. The **protocol is identical
regardless of trigger** — a human can also relay turns manually by invoking each agent in sequence,
since the file contract is the same.

## The coordination tool (tooling contract)

> **Optional reference implementations.** Implementations of this contract are published separately —
> `stateful-spec-flow` (crates.io, Rust) and `@stateful-spec/flow` (npm, Node/TypeScript); their source
> lives in the methodology repository's [`packages/`](../packages/). They are **not required**: the
> contract below is fully sufficient, and any tool — or a human relaying turns — satisfies it. An agent
> guided by this methodology that wants to use a published flow tool **must notify the user and ask
> permission first** (see [`roles.md`](roles.md)).

The flow's safety properties do not depend on any particular program — they depend on a **contract**
that whatever drives the state file must honor. The methodology specifies the contract, not an implementation
(optional published implementations are noted above). A project may implement it as a small CLI in any language, wire
it into its agent's scheduler, or **relay turns by hand** — all satisfy the same contract:

1. **Validated verbs are the only writer.** No agent hand-edits the frontmatter. Each transition is
   a verb with an explicit **precondition** (role + current `step`); an illegal verb is rejected and
   **leaves the state untouched**. This makes the "wrong turn flag → deadlock" class and the
   "read-only role makes a change" class impossible by construction, not by convention.
2. **Atomic writes + monotonic `seq`.** Every accepted verb writes atomically (e.g. temp file +
   rename), bumps `seq`, and appends one `#seq`-tagged turn-log line — so the trace is strictly
   chronological without parsing timestamps.
3. **A blocking poll.** "Wait until it is my turn" must be an event on the turn flip, not a fixed
   timer — and it should carry a **stall watchdog**: if `seq` is unchanged for too long, warn (with
   the manual-recovery protocol) and keep waiting.

The abstract verb set:

| Role | Verb | Precondition | Effect |
|------|------|--------------|--------|
| PM | `approve-plan --total <n>` | AWAITING_PLAN_APPROVAL · turn=pm | → RUNNING, milestone 1, step=SPEC_PENDING |
| PM | `submit-spec` *(3-agent)* | RUNNING · turn=pm · step ∈ {SPEC_PENDING, SPEC_CHANGES_REQUESTED} | → ARCH_REVIEW, turn=architect |
| ARCH | `approve-spec` *(3-agent)* | RUNNING · turn=architect · step=ARCH_REVIEW | → SPEC_APPROVED, turn=pm |
| ARCH | `request-spec-changes "<reason>"` *(3-agent)* | RUNNING · turn=architect · step=ARCH_REVIEW | → SPEC_CHANGES_REQUESTED, turn=pm, `spec_review_round++` (> max → BLOCKED) |
| PM | `spec-ready` | RUNNING · turn=pm · step ∈ {SPEC_PENDING *(2-agent)*, SPEC_APPROVED *(3-agent)*} | → SPEC_READY, turn=engineer |
| ENG | `submit [--gate "…"]` | RUNNING · turn=engineer · step ∈ {SPEC_READY, CHANGES_REQUESTED} · gate clean | → AWAITING_REVIEW, turn=pm; refuses an unformatted/failing tree (kills the "false green"); `--gate` records the real exit codes in the turn-log |
| ENG | `hand-back "<reason>"` *(3-agent)* | RUNNING · turn=engineer · step ∈ {SPEC_READY, CHANGES_REQUESTED} | → SPEC_PENDING, turn=pm (re-scope) |
| PM | `approve` | RUNNING · turn=pm · step=AWAITING_REVIEW | → APPROVED, turn=engineer |
| PM | `request-changes "<reason>"` | RUNNING · turn=pm · step=AWAITING_REVIEW | → CHANGES_REQUESTED, turn=engineer, `review_round++` (> max → BLOCKED) |
| ENG | `advance` | RUNNING · turn=engineer · step=APPROVED · **product tree committed** | next milestone SPEC_PENDING (resets the round counters) or DONE; **refuses on the default branch**; **archives** the just-completed milestone's auxiliaries |
| any | `block "<reason>"` | any | → BLOCKED (halt for the human) |
| PM | `spawn [--profile <domain>]` | state readable | launch the next agent for the current `step` — the payload (which persona/skills) is resolved **from the `step` (+ profile)** so it cannot be mis-spawned: at `ARCH_REVIEW` the read-only reviewer (born reading the draft spec); at `SPEC_READY`/`CHANGES_REQUESTED` the engineer profile (born with the review handoff in context, when present); at `APPROVED` a **commit-only** spawn (no engineering personas — the turn only commits + advances) |
| — | `status` / `poll --role <r> [--stall-after N]` | — | read state / **block until it is `<r>`'s turn** (the primary standby), with the stall watchdog |

> **Design-gate handoff, recovered by construction.** The architect's handoff back to the PM is the
> fragile step: a spawned reviewer may end without running its verb. Make it robust without depending
> on the reviewer running anything: instruct the architect to **end its final message with a
> parseable verdict block** (`verdict` / `milestone` / `reason`; scaffold
> [`templates/implementation/arch-verdict.md`](../templates/implementation/arch-verdict.md)); after
> the reviewer exits, the **PM recovers the verdict** and runs the matching verb itself. The verbs
> are **turn-gated, not identity-gated**, so the PM can run `approve-spec`/`request-spec-changes`
> while `turn=architect` — that is the turn flip. The reviewer running the verb is an optional
> fast-path.

> **Platform hardening (lesson-level, implementation-specific).** On a platform where a running build
> artifact holds an exclusive lock, release or terminate the stale artifact before a rebuild, or the
> next gate hits a lock error. Guard against **orphaned spawned agents** — confirm exactly one worker
> is alive per turn, and that a spawned agent has exited before the next spawn. Judge a worker's
> liveness by **activity** (e.g. CPU delta), never by a wall-clock timestamp (process clocks may be
> local, not UTC). These are concerns for whatever implements the spawn/poll; the methodology only
> requires that the contract above hold.

## Review discipline

The PM's delivery review is the gate that decides `approve` vs. `request-changes`. For UI software it
has **three gates, not two: technical · visual · functional.**

- The **technical** gate proves it compiles / tests / lints.
- The **visual** gate proves it *looks* right (the reviewer must actually be able to **see** the
  rendered output — a reviewer that cannot see makes the visual gate a no-op).
- The **functional** gate proves it *works* — the reviewer drives the live artifact and exercises the
  primary controls; every affordant control must do something real (or an integration test covers
  it). A screen that passes technical + visual but ships **inert** (dead clicks, hover-only
  affordances) is a blocking finding.

For non-UI milestones the visual/functional gates collapse to "it runs and does what the spec says."

**Review is also discovery.** While reviewing, the PM actively hunts evolutionary opportunities —
real functionality gaps, shared-system evolution needs — and records them as `O-NNN` entries in
`.stateful-spec/backlog.md` (see [`backlog.md`](backlog.md)), **never** folded into the current
milestone's scope (no scope creep). A real defect against the milestone's spec is **not** an
opportunity — it follows `request-changes`.

**Handoff discipline (what the verbs can't enforce alone).** Hand the turn back **only after
finishing all writes** — never a partial handoff. Pass the **real gate exit codes** at `submit`
(recorded in the turn-log); the PM still re-runs the full gate in review. Pin the toolchain so
fmt/lint verdicts are identical across agents (the gate must certify against one ruler). The standby
is the blocking poll, every time — never an implicit "wait." Every flip to a spawned role is a spawn,
including the commit-only `APPROVED` turn.

## Stall handling

`review_round` counts delivery-review cycles for the current milestone (one round = `AWAITING_REVIEW`
→ `CHANGES_REQUESTED` → fix → `AWAITING_REVIEW`); `spec_review_round` counts design-gate cycles in
the three-agent flow. When either exceeds its cap (default 3), the PM sets `process_status: BLOCKED`
with a `blocked_reason` and halts for the human instead of looping indefinitely. This encodes the
core lesson behind this mode: bound the work, never run unbounded.

## Branches, commits, and human gates

- **One feature branch** for the whole umbrella iteration, created by the PM at `approve-plan`
  (`feature/NNN-<name>`); the Engineer adds **one commit per approved milestone** onto it. Each commit
  leaves the repository green (the quality gate runs at the end of every milestone — never deferred to
  the end of the flow). The **`advance` verb enforces this**: it refuses to advance while product code
  (anything outside `.stateful-spec/`) is uncommitted, **refuses on the default branch**, and
  **archives the just-completed milestone's auxiliaries** by construction.
- **Commit to the local feature branch is pre-authorized** inside the flow.
- **Irreversible / outward-facing actions are NOT** pre-authorized: pushing, opening a PR on a shared
  remote, publishing, or releasing. When the flow reaches such a step, the agent sets `BLOCKED`
  (reason) and waits for explicit human authorization, consistent with [`roles.md`](roles.md) (the AI
  does not make irreversible changes without instruction).

## Completion

When the last milestone reaches `COMMITTED`, the PM sets `process_status: DONE` and `done: true`,
writes a final turn-log summary, runs `end-session` for the final milestone, and closes the umbrella
iteration (move to Recent Completions, set History Index status `done`, compile the Engrama).
Delivery (push/PR/publish) remains a human gate. `DONE` is sticky; the file persists as the record of
the completed flow until a new flow resets it.

## Relationship to the 5 phases and existing operations

This mode **reuses**, it does not replace:

| Flow step | Phase | Operation reused |
|-----------|-------|------------------|
| Master plan (milestones) | Analyze + Plan | the planning phases ([`phases/01-analyze.md`](phases/01-analyze.md), [`phases/02-plan.md`](phases/02-plan.md)) |
| Milestone spec | Specify | [`create-technical-spec`](../prompts/operations/create-technical-spec.md) |
| Spec review (design gate) | Specify | [`review-changes`](../prompts/operations/review-changes.md) (three-agent flow, read-only) |
| Milestone implementation + gate | Implement + Verify | [`phases/04-implement.md`](phases/04-implement.md), [`phases/05-verify.md`](phases/05-verify.md) |
| Code review (handoff) | Verify | [`review-changes`](../prompts/operations/review-changes.md) |
| Commit | Verify | [`write-commit-message`](../prompts/operations/write-commit-message.md) |
| Session bookkeeping | — | [`start-session`](../prompts/operations/start-session.md) / [`end-session`](../prompts/operations/end-session.md) (milestone-session variant) |

## Anti-patterns

- **Using it for small work** — overhead with no benefit; use the single-agent cycle.
- **Duplicating the milestone list** — the master plan is the only source; specs refine, they don't
  re-list.
- **Façade milestones** — marking a milestone done without a green quality gate and a real review;
  the gate runs every milestone, the review is mandatory.
- **Skipping the human gate on irreversible actions** — never push/PR/publish autonomously.
- **Two agents writing at once** — the turn flag is authoritative; act only on your turn.
- **Hand-editing the state file** — go through the validated verbs; a hand-edit can set an illegal
  state the preconditions exist to prevent.
- **Unbounded review loops** — respect the round caps; escalate via `BLOCKED`.
- **A blind visual gate** — a reviewer that cannot see the rendered output cannot run the visual gate;
  use a vision-capable reviewer or cover it with tests.
