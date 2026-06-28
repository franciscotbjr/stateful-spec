# Backlog & Intake

**Goal:** Give a project one durable, triaged home for the **opportunities** it discovers — work
beyond the current milestone's contract — and a disciplined pipeline that carries a raw note all
the way to the roadmap without letting half-formed ideas leak into planning.

> **This doc is the canon (the *format*).** In a project that adopts the methodology, the triaged
> list lives at `.stateful-spec/backlog.md` (the *instance*); the raw inbox lives at
> `.stateful-spec/intake/`; the active roadmap lives in `.stateful-spec/memory.md`. This file
> defines how those work — it is not itself a project's backlog.

## What belongs in the backlog

A backlog entry is an **opportunity**: evolution beyond the current milestone's contract. For
example:

- a real functionality gap discovered in use,
- a needed evolution of a shared system (a missing component / token / capability → a proposed
  **extension**, never a one-off),
- UX polish,
- engineering debt.

**What does NOT belong:** a **real defect against the current milestone's spec**. That follows the
correction path (`request-changes` / reopen), never the backlog. A defect found in post-delivery
testing is handled by the [QA phase](qa-phase.md); only the *durable opportunity* it may reveal is
promoted here.

## The artifact: `backlog.md`

A single Markdown table, one row per opportunity:

| Column | Meaning |
|--------|---------|
| **ID** | `O-NNN` — sequential, **stable**, never reused. |
| **Description** | The opportunity, stated without scope-creep into the current milestone. |
| **Origin** | Where it came from (iteration / review / human test / idea). |
| **Status** | `new` → `triaged` → `promoted` \| `discarded`. |
| **Destination** | Where it went once resolved (the iteration/milestone that took it, or the discard reason). |

**Numbering.** `O-NNN` is owned solely by `backlog.md` and is allocated **at triage**, never by a
raw intake note. Numbers are stable and never reused, so cross-references from iterations and
session logs stay valid forever.

**States.**

- **`new`** — promoted into the backlog, not yet evaluated.
- **`triaged`** — evaluated; awaiting a destination (roadmap slot / iteration) or a discard.
- **`promoted`** — turned into roadmap/iteration work (the Destination column says which).
- **`discarded`** — closed without doing it; the Destination column records **why**.

Recording an opportunity is done **without scope-creep**: the entry is born `new` and the current
milestone does not change.

## Intake: the raw inbox

Before an idea is shaped enough to earn an `O-NNN`, it lives in the project's `.stateful-spec/intake/`
folders — a raw inbox mirroring the human's working area:

- **`Backlog/`** — inbound feature/improvement ideas not yet shaped into an `O-NNN`.
- **`Discovery/`** — open questions, spikes, research notes that precede a decision.
- **`QA/`** — defects/observations from human / post-delivery testing, before they are routed by
  the [QA phase](qa-phase.md).

### The READY convention

Every intake item carries YAML frontmatter with a `status` gate:

- **`draft`** — captured but not yet shaped; **not** eligible to become a milestone, iteration, or
  `O-NNN`.
- **`ready`** — shaped enough to be triaged.
- **`triaged`** — already routed (promoted to an `O-NNN` or discarded). The item stays as the raw
  record but is **not** re-surfaced at later triages. A `destination:` line records where it went
  (the `O-NNN` + slot, or the discard reason).
- **Missing or unrecognized `status`** — treated as `draft` (not triaged); the triage step **warns**
  so the item gets a valid `status`.

**Only `ready` items are picked up at triage.** This single gate keeps half-formed notes out of
planning; `draft` items wait until someone shapes them (or they are discarded), and `triaged` items
are skipped (the scan stays idempotent across kickoffs). A missing/unrecognized `status` defaults to
`draft` with a warning, so an unshaped note never reaches planning by omission.

## The pipeline

```
raw intake (Backlog/, Discovery/, QA/)  ──triage──→  backlog.md (O-NNN, triaged)  ──promote──→  roadmap (memory.md) / iteration
```

- **Triage** happens at **iteration close** and at the **next kickoff**: each `ready` raw item is
  reviewed and either **promoted** into `backlog.md` as a stable `O-NNN`, or **discarded** with a
  reason. A resolved `Discovery/` note may instead simply feed a decision recorded in the relevant
  iteration / ADR. After routing, the raw item is flipped to `status: triaged` (with a `destination:`)
  so it is not re-triaged.
- **Promotion** moves a `triaged` opportunity onto the roadmap (in `memory.md`) or into a concrete
  iteration; its Destination is recorded.

### Enforcement points (where triage executes)

The two triage moments are wired into the session commands:

| Moment | Command | Where |
|--------|---------|-------|
| **Next kickoff** | `start-session` | STEP 1.5 — *Triage the Intake Inbox* |
| **Resume / direct-task** | `resume-session` | STEP 2.5 + the direct-task entry |
| **Iteration close** | `end-session` | the close-side triage step |

The commands point here for the routing rules; this table names the enforcement points.

## Orchestration = manual (no daemon)

There is **no orchestrator agent and no real-time watcher**. A live daemon would need an always-on
process, which is fragile in a session-based environment. Triage is performed **by a human or the
PM at the explicit triage moments** above — the intake folders are a shared convention, not an
automated queue.

## Per-Type Meaning

Opportunities, intake, and triage apply to **every** Project Type (see
[`project-types.md`](project-types.md)). What an opportunity *is* shifts with the type — a code
capability for `software`, a new/expanded skill for `skills`, a follow-up question or dataset for
`studies` — but the artifact (`O-NNN`, the states), the READY gate, and the pipeline are identical.

## Template: an intake item

One file per item, `kebab-case.md`, in the matching intake folder, with the READY frontmatter:

```markdown
---
status: ready | draft
title: <short title>
origin: <where it came from — review, human test, idea, discovery>
# QA items also carry, filled at triage (not at capture):
# category: E | P | H   # E = engineering defect, P = process gap, H = human/UX call
---

<The idea / question / defect, in a few lines. For an idea: enough to shape the scope before
promoting. For a discovery: the question + findings so far + the next step. For a QA defect: a
repro + the suspected class + the proposed route (single- vs multi-agent).>
```

> `draft` until shaped; flip to `ready` once it is concrete enough to triage; triage then flips it to
> `triaged` (+ a `destination:` line). Do **not** assign an `O-NNN` in an intake item — numbering
> belongs to `backlog.md`.

## Completion Criteria (when adopting this in a project)

- [ ] `.stateful-spec/backlog.md` exists with the `O-NNN` table (ID / Description / Origin / Status
      / Destination) and the "defect ≠ opportunity" rule stated.
- [ ] `.stateful-spec/intake/` has `Backlog/`, `Discovery/`, `QA/` and documents the READY
      convention.
- [ ] Triage moments (iteration close + kickoff) are part of the project's working rhythm.
- [ ] `O-NNN` numbering is sequential, stable, and never reused.
