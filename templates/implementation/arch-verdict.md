---
verdict: APPROVED            # APPROVED | CHANGES_REQUESTED
milestone: <k>               # the current_milestone from flow-state.md
reason: <one line>           # required iff CHANGES_REQUESTED; omit when APPROVED
---

<!--
Scaffold for the three-agent flow's design-gate handoff (see `methodology/multi-agent-flow.md`).

The read-only Architect reviewer ends its final message with the parseable block above; the PM
recovers the verdict and runs the matching verb (`approve-spec` / `request-spec-changes "<reason>"`).
The verbs are turn-gated, not identity-gated, so the PM runs them while `turn=architect` — that is
the turn flip. Writing this file is an optional fast-path; the verdict is recoverable from the
reviewer's final message regardless.
-->

Optional human-readable notes from the spec review: what was checked, the failure classes
considered, and why the spec was approved or what must change.
