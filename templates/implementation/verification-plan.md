# Verification Plan

> A type-aware entry point for planning how a unit of work will be verified. Keep the
> subsection for your Project Type; delete the others. What "verification" means per
> type is defined in [`methodology/project-types.md`](../../methodology/project-types.md).

---

## Metadata

- **Spec Reference:** [Link or name of the specification being verified]
- **Project Type:** software | skills | studies
- **Status:** Draft | Ready | In Progress | Complete

## Verification — software

> Applies to: software

Use the full [`test-plan.md`](test-plan.md) template for the testing strategy
(unit / integration / edge / error cases, fixtures, execution commands, coverage).
Summarize or link it here.

- **Test plan:** [link to the filled `test-plan.md`, or inline the key cases]

## Verification — skills

> Applies to: skills

### Trigger Validation

| # | Case | Should the skill fire? | Result |
|---|------|------------------------|--------|
| 1 | [intended case] | Yes | [ ] |
| 2 | [out-of-scope case] | No | [ ] |

### Before / After Coverage

- [ ] Every anti-pattern in the spec has at least one before/after example
- [ ] Each example demonstrates exactly one rule

### Self-Check Checklist

- [ ] Directory has exactly `SKILL.md` + `README.md`
- [ ] `description` is specific and actionable
- [ ] "When NOT to apply" present
- [ ] Prose follows sober-prose (no rhetorical padding)
- [ ] Every normative claim grounded in a source
- [ ] Root README skill table updated

## Verification — studies

> Applies to: studies

### Source & Claim Verification

| # | Claim | Cited source | Resolves? | Supports claim? |
|---|-------|--------------|-----------|-----------------|
| 1 | [claim] | [reference] | [ ] | [ ] |

### Reproducibility Checklist

- [ ] Data committed or referenced with stable access
- [ ] Scripts/notebooks run end to end from a clean environment
- [ ] Random seeds / parameters recorded
- [ ] Figures/tables regenerate from the committed pipeline

### Internal Consistency

- [ ] No contradictory statements across sections
- [ ] Terminology and notation used consistently
- [ ] Argument coherent end to end
