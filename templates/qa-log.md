# NNN — Post-delivery QA (corrections)

> Log of **post-delivery corrections** for <artifact>, from defects found in **human / real-world
> testing** of the delivery. Complements the (DONE) iteration record `NNN-<name>.md`. The discipline
> is defined in `methodology/qa-phase.md`.
>
> **Directives for this cycle:**
> 1. **Route by type** — visual/layout/interaction polish → single-agent (applied directly);
>    substantial/logic/cross-cutting → multi-agent (spec → review → implement).
> 2. **Register + categorize** every item: **(E)** engineering · **(P)** process · **(H)** human
>    definition (primary + secondary when both apply).
> 3. **Defect = lesson** → compiled at the end as a skill-improvement (which skill, what changes).
> 4. **One commit per item**, gate-green.

## Item registry

| # | Item | Category | Status |
|---|------|----------|--------|
| 1 | <short description> | **E** (primary) + **P** (secondary) | Fixed — gate green + smoke; <how confirmed> |
| 2 | ... | ... | ... |

---

### Item #1 — <title>

**Symptom.** <what was observed>

**Root cause.** <the actual mechanism, not the surface>

**Category.** **(E)**/**(P)**/**(H)** — <why>, primary; <secondary if any>.

**Fix.** <what changed, which files>

**Verification.** <gates run + result; smoke/boot; what remains human-confirmed>

**Lesson → skill.** <which skill / knowledge file, what to add so the class is caught next time>
