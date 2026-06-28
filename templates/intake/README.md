# Intake — raw inbound work (Stateful Spec, project-side)

> Template for a project's `.stateful-spec/intake/` folder. This is the **raw inbox** for work that
> has not yet been triaged into a numbered opportunity. The canonical, triaged backlog stays in
> `.stateful-spec/backlog.md`; the roadmap stays in `.stateful-spec/memory.md`.
>
> This README documents the **project's** intake folders and the READY convention only. The
> methodology canon lives in `methodology/backlog.md` (intake + backlog pipeline) and
> `methodology/qa-phase.md` (post-delivery QA, routed by type). Do not duplicate that canon here.

## The pipeline

```
raw intake (Backlog/, Discovery/, QA/)  ──triage──→  backlog.md (O-NNN, triaged)  ──promote──→  roadmap (memory.md) / iteration
```

- **`Backlog/`** — inbound feature/improvement ideas not yet shaped into an `O-NNN`.
- **`Discovery/`** — open questions, spikes, research notes that precede a decision.
- **`QA/`** — defects and observations from human/post-delivery testing, before they are routed
  (visual/layout/interaction polish → single-agent; substantial/logic/cross-cutting → multi-agent;
  see the canonical QA phase in `methodology/qa-phase.md`, which routes by type and applies the
  E/P/H discipline).

**Triage** happens at iteration close and at the next kickoff: a raw item is reviewed, and either
**promoted** into `.stateful-spec/backlog.md` as a stable `O-NNN` entry (then possibly onto the
roadmap), or **discarded** with a reason. Numbering (`O-NNN`) is owned by `backlog.md`, never by the
raw items here. After routing, the item is flipped to `status: triaged` so it is not re-promoted.

These two moments are wired into the session commands: the **next kickoff** is `start-session`
(STEP 1.5 — *Triage the Intake Inbox*); **iteration close** is `end-session`. See the methodology
canon in `methodology/backlog.md` ("Enforcement points").

## The READY convention

Every intake item carries YAML frontmatter with a `status`:

```yaml
---
status: ready | draft | triaged
title: <short title>
origin: <where it came from — review, human test, idea>
# set at triage when the item is routed:
# destination: <O-NNN → slot, or discard reason>
---
```

- **`draft`** — captured but not yet shaped; **not** eligible to become a milestone or iteration.
- **`ready`** — shaped enough to be triaged into `O-NNN` and considered for the roadmap.
- **`triaged`** — already routed (promoted to an `O-NNN` or discarded); kept as the raw
  capture/shaping record but **not** re-surfaced at later triages. The `destination:` line says where
  it went.
- **Missing or unrecognized `status`** — treated as `draft` (not triaged); triage warns so the item
  gets a valid `status`.

Only `ready` items are picked up at triage. `draft` items wait until someone shapes them (or they are
discarded); `triaged` items are skipped (the scan stays idempotent across kickoffs); a missing or
unrecognized `status` defaults to `draft` with a warning. This is the single gate that keeps
half-formed notes from leaking into planning.

## Orchestration = manual (no daemon)

There is **no orchestrator agent and no real-time watcher**. A live daemon would need an always-on
process, which is fragile in a session-based environment. Triage is performed **by a human or the PM
at the explicit triage moments** above — the folders are a shared convention, not an automated queue.

## Folder index

- [`Backlog/`](Backlog/README.md) — inbound ideas.
- [`Discovery/`](Discovery/README.md) — open questions / spikes.
- [`QA/`](QA/README.md) — defects & observations from testing.
