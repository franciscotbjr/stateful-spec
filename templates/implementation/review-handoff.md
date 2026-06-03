# Review Handoff — Iteration [NNN] / Milestone [M<k>] ([milestone name])

> The PM/Architect's code review of one milestone delivery in a multi-agent flow
> (see `methodology/multi-agent-flow.md`). It is the artifact that flips the milestone to
> `APPROVED` or `CHANGES_REQUESTED`. File name: `handoff-NNN-<name>-m<k>-review.md`.

## Metadata

- **Reviewer:** [agent/tool + role]
- **Date:** [YYYY-MM-DD]
- **Base:** spec `NNN-<name>-m<k>-spec.md`; working tree (committed or not)
- **Method:** read the milestone's code **and ran the quality gate** — do not trust the
  spec's checkboxes; verify against the gate output.
- **Verdict (one line):** [PASS — no blocking findings · or · CHANGES REQUESTED — N blocking findings]

## Quality gate — executed and verified

> List each gate command from the Project Definition and its real result. For non-software
> types, confirm each checklist item explicitly instead.

| Gate | Command | Result |
|------|---------|--------|
| [e.g., Lint] | [e.g., `cargo clippy -- -D warnings`] | [✅ / ❌ + detail] |
| [e.g., Tests] | [e.g., `cargo test`] | [✅ N passed / ❌] |
| [e.g., Build] | [e.g., `cargo build`] | [✅ / ❌] |

## What's good (keep)

- [Things done correctly — be specific, cite file:line]

## Findings

> Severity: **P0** blocking (must fix to pass) · **P1** should fix · **P2/nit** optional.

### [P0/P1/P2]-1 — [short title]
- **Location:** [file:line]
- **Issue:** [what is wrong]
- **Recommendation:** [how to fix]

## Adherence to the spec's acceptance criteria

| Criterion | Status | Evidence |
|-----------|--------|----------|
| [criterion 1] | [✅ / ❌ / ⚠️] | [test name / file:line] |

## Decision

- [ ] **APPROVED** — set `step: APPROVED` in `flow-state.md` (Engineer commits next).
- [ ] **CHANGES REQUESTED** — set `step: CHANGES_REQUESTED`, bump `review_round`; the
      Engineer must clear all P0 (and ideally P1) findings, then re-submit for review.

## Summary

[2-4 sentences: is the milestone done correctly, what (if anything) blocks approval, and the
single most important thing to carry into the next milestone.]
