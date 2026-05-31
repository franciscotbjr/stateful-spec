# Skill Specification

> **Applies to:** skills. See [`methodology/project-types.md`](../../methodology/project-types.md).

> Use this template to specify a single Agent Skill before authoring it. A skill is a
> directory with `SKILL.md` (YAML `name`/`description` + instructions) and `README.md`.

---

## Metadata

- **Skill Name:** [kebab-case, matches the directory name — e.g., sober-prose]
- **Language:** [e.g., en, pt-br — skills are monolingual]
- **Status:** Draft | Ready | In Progress | Complete

## Target Bias / Phenomenon

[What behavior, bias, or phenomenon does this skill correct or enable? Describe the
problem in the agent's output that this skill addresses, in 2-3 sentences.]

## Trigger (`description` draft)

> The `description` is the trigger condition — it decides when the agent loads the
> skill. Make it specific and actionable: it must cover the intended cases and
> exclude the rest.

```
[Draft the YAML description value here.]
```

## Anti-Patterns to Cover

> The concrete behaviors the skill should catch and correct.

- [ ] [Anti-pattern 1 — e.g., rhetorical padding ("it's important to note that…")]
- [ ] [Anti-pattern 2]
- [ ] [Anti-pattern 3]

## Before / After Examples

> Each example demonstrates one rule: the "before" exhibits the anti-pattern; the
> "after" shows the corrected form.

| # | Before | After | Rule demonstrated |
|---|--------|-------|-------------------|
| 1 | [text] | [text] | [rule] |
| 2 | [text] | [text] | [rule] |

## When NOT to Apply

[Explicitly list the cases where this skill should not fire or should be overridden,
to keep the trigger from over-matching.]

## Source Grounding

> Every normative claim in the skill must be grounded in a source.

| Claim | Source |
|-------|--------|
| [claim] | [reference / link] |

## Bilingual Counterpart

[If there is a sibling skill in another language, name it and note the cross-reference
in the root README. Otherwise: none.]

## Out of Scope

[What this skill explicitly does NOT cover, to prevent scope creep.]

## References

- [Link to related skill, issue, PR, or source material]
