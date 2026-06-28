# Backlog

> Triaged **opportunities** — work beyond the current milestone's contract. The canonical format is
> defined in `methodology/backlog.md` (at the repo root — this repo IS the methodology source). This
> file (`.stateful-spec/backlog.md`) is the project's instance: one row per opportunity, `O-NNN`
> numbering owned here (sequential, stable, never reused).
>
> A real defect against the current milestone's spec does **not** belong here — it follows the
> correction path (`request-changes` / reopen). Only durable opportunities are recorded.

<!-- O-NNN allocated at triage. States: new → triaged → promoted | discarded. -->

| ID | Description | Origin | Status | Destination |
|------|-------------|--------|--------|-------------|
| O-001 | The repo is the Stateful Spec methodology source yet had not adopted the structures it ships — its own `intake/` inbox + `backlog.md`, documented `RAW_HISTORY`, and an up-to-date `project-definition.md` structure. Make the repo practice its own methodology. | iteration 012/013 review (self-adoption audit) | promoted | Iteration 014 — self-adoption |

<!--
States:
- new       — promoted into the backlog, not yet evaluated.
- triaged   — evaluated; awaiting a destination (roadmap slot / iteration) or a discard.
- promoted  — turned into roadmap/iteration work (Destination says which).
- discarded — closed without doing it; Destination records why.
-->
