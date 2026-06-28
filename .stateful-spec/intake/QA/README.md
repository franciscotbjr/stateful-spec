# intake / QA — defects & observations from testing

Defects and observations from human / post-delivery testing, captured before they are routed. One
file per item, `kebab-case.md`, with the READY frontmatter (see [`../README.md`](../README.md)).

At triage, each QA item is **routed by type** (the canonical QA phase in `methodology/qa-phase.md`):

- **visual / layout / interaction polish** → single-agent (applied directly), recorded with
  root cause + category **E/P/H** + lesson → skill + a commit per item;
- **substantial / logic / cross-cutting** → multi-agent (spec → review → implement).

A real defect against a milestone's spec is **not** an opportunity — it follows
`request-changes` / reopening, never the backlog. A QA item that reveals durable evolution is
promoted to [`../../backlog.md`](../../backlog.md) as an `O-NNN`.

## Example item

```markdown
---
status: ready
title: Long lists do not scroll when the data set is large
origin: human test (post-delivery QA)
category: E      # E = engineering defect, P = process gap, H = human/UX call
---

Repro: load a view with many items → the list clips, no scrollbar. Suspected class: a missing
scroll-wrapper on this container (recurring scroll class). Route: visual/layout → single-agent fix
+ lesson back to the relevant UI skill.
```

> `draft` while only observed; `ready` once it has a repro and a suspected class. The `category`
> (E/P/H) field is filled at triage, not at capture.
