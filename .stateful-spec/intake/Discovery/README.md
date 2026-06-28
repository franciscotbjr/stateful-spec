# intake / Discovery — open questions & spikes

Research notes, open questions, and spike results that precede a decision. One file per question,
`kebab-case.md`, with the READY frontmatter (see [`../README.md`](../README.md)). A resolved
discovery either feeds a decision (recorded in the relevant iteration / ADR) or is promoted to
[`../../backlog.md`](../../backlog.md) as an `O-NNN` if it surfaces durable work.

## Example item

```markdown
---
status: ready
title: Can the test suite run in a headless environment?
origin: discovery (spike)
---

Question: can the quality gate run without a display? Findings so far: most checks are headless,
but one visual check needs a display surface. Next step: a time-boxed spike on an off-screen
alternative. Collects the evidence before re-triaging.
```

> Use `draft` while still gathering; `ready` once the question is well-posed and the next step is
> clear. Long-lived opportunities go to [`../../backlog.md`](../../backlog.md); one-off answers go to
> the iteration record.
