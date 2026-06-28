---
status: ready | draft
title: <short title>
origin: <where it came from — review, human test, idea, discovery>
# set at triage when the item is routed (not at capture):
# destination: <O-NNN → slot, or discard reason>
# QA items also carry, filled at triage:
# category: E | P | H   # E = engineering defect, P = process gap, H = human/UX call
---

<!--
One intake item per file, kebab-case.md, in the matching `.stateful-spec/intake/` folder
(Backlog/, Discovery/, QA/). See `methodology/backlog.md` for the pipeline and the READY gate.

`draft` until shaped; flip to `ready` once it is concrete enough to triage; triage then flips it to
`triaged` (+ a `destination:` line). Do NOT assign an `O-NNN` here — numbering belongs to
`.stateful-spec/backlog.md`.
-->

<The idea / question / defect, in a few lines. For an idea: enough to shape the scope before
promoting. For a discovery: the question + findings so far + the next step. For a QA defect: a
repro + the suspected class + the proposed route (single- vs multi-agent).>
