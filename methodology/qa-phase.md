# Post-Delivery QA

**Goal:** Capture, route, fix, and learn from defects that surface when a **delivered**
artifact meets human / real-world testing — with a discipline that turns each defect into a
recorded lesson rather than a silent patch.

> **This is not a sixth phase.** The iteration cycle stays five phases (Analyze → Plan →
> Specify → Implement → Verify). Post-delivery QA is a **loop entered *after* Verify/delivery**,
> when testing the shipped artifact reveals problems the quality gates and reviews did not.

## Why this exists

Green quality gates and a clean delivery review are necessary, not sufficient. They prove the
artifact builds, passes its tests, and matches its spec — they do **not** prove it behaves
correctly under a real user. In a representative case, a delivery review approved every milestone
green, yet a large batch of visual / layout / interaction defects surfaced only when a human
exercised the running artifact. The cost migrated *past* delivery. Post-delivery QA is the gate
for that cost: a place where the defects are registered, categorized, root-caused, fixed, and —
critically — **fed back into the project's skills/knowledge** so the same class does not recur.

## When to Enter This Phase

- A **delivered / `DONE`** artifact is exercised by a human or in real-world use and a defect or
  observation appears.
- Feedback from a stakeholder/user reports behavior the gates did not catch.

**When NOT to enter it.** A defect found *before* `DONE` — i.e. a real defect against the current
milestone's **spec** — is **not** post-delivery QA. It follows the normal correction path
(`request-changes` / reopen the milestone), never this loop. Post-delivery QA begins only once the
work has been delivered.

## In-flow incident log (the pre-`DONE` sibling)

Post-delivery QA covers defects in the *delivered artifact*. A **different** class — **process /
methodology failures during the flow**, before `DONE` (a spawned reviewer that ends without handing
off, a zombie agent, a spawn death, a commit on the wrong branch, a hand-back, a takeover) — has its
own canonical home: the **in-flow incident log**, an `[INCIDENT]` entry in the umbrella iteration's
**Session Log**, written **at the moment the failure or recovery occurs**.

- **Format:** `[INCIDENT/<E|P|H>] <symptom> · <recovery>` with the timestamp — `P` (process) is the
  usual category here. These are not spec defects (those follow `request-changes`) and not
  post-delivery defects (those are this phase).
- **Swept at close.** `end-session` (and, lightly, `save-session`) **enumerate the `[INCIDENT]`
  entries before compiling the Engrama** and ensure each becomes a `Learning`, routed: durable
  behavioural → memory; recurring/opportunity → `O-NNN` in `backlog.md`; one-off → Engrama only.
- **Why forced:** a failure recorded only by the agent's discipline is lost when the discipline
  fails — and the evolutionary feedback with it. Logging at the moment + sweeping at close makes the
  capture by construction.

## Inputs

- The delivered artifact and the report(s) from testing it.
- The project's **Quality Gates** (`project-definition.md`) — re-run per fix.
- The project's skills / knowledge base — the destination of every lesson.
- The intake `QA/` inbox, if items were captured there first (see [`backlog.md`](backlog.md) for the
  intake convention).

## Activities

### 1. Register every item

Open (or append to) a **QA log** for the delivery (one file per delivery cycle). Record *every*
defect and observation, even small ones — the log is the evidence trail and the source of the
compiled lessons. Use the template at the end of this document.

### 2. Route by type

Each item is routed by **what kind of work the fix is**:

| Item type | Route | Who |
|-----------|-------|-----|
| **Visual / layout / interaction polish** (and small, localized fixes) | **single-agent** | The PM / agent applies it directly, item by item. |
| **Substantial / logic / cross-region / cross-cutting** | **multi-agent** (software) | A spec is written, reviewed, then implemented. See [`multi-agent-flow.md`](multi-agent-flow.md). |

When in doubt, prefer the lighter route for a genuinely localized fix and the heavier route once a
fix touches logic, multiple regions, or an interface.

### 3. Categorize (E / P / H)

Tag every item with its cause, **primary** plus a **secondary** when one applies:

- **(E) — Engineering** — a defect in the built artifact (wrong code/structure/asset).
- **(P) — Process** — a gap in the methodology/gates that let the defect through (e.g. a review or
  gate that could not observe the failure).
- **(H) — Human** — a missing or unclear human definition/decision (ambiguous requirement, a
  judgment call that was never made).

The category is what drives the lesson: an **E** sharpens an engineering skill, a **P** hardens
the process/gate, an **H** clarifies how requirements get stated.

### 4. Find the root cause

For each item, write the **symptom** and the **root cause** — the actual mechanism, not the
surface. A symptom logged without its mechanism produces a patch, not a lesson; the next instance
of the same class will not be recognized.

### 5. Fix, gate, and commit — per item

- Apply the fix for the **one** item.
- Run the project's **quality gates** (and any smoke/boot check) — green before moving on.
- **Commit that item by itself.** One commit per QA item keeps the cause→fix mapping legible and
  the history bisectable. Do not batch several QA fixes into one commit.

### 6. Turn each item into a lesson → skill

This is the step that makes QA compounding rather than repetitive. For each item, name the
**skill / knowledge file** that should change and **what** to add, so the class is caught by
construction (or by an earlier gate) next time. "What is not forced by construction or measured by
automation tends to fail again." Compile the lessons at the end of the cycle.

### 7. Promote durable opportunities

If a QA item reveals **durable evolution** (a missing component, a recurring need, real debt) — as
opposed to a one-off fix — promote it to the backlog as an `O-NNN`, per [`backlog.md`](backlog.md).
The one-off fix stays in the QA log; only the lasting opportunity goes to the backlog. (A real
defect against a spec is neither — it is reopened, not backlogged.)

## Per-Type Meaning

Like the phase guides, this document is read in place; the **Project Type** sets what the words
mean (see [`project-types.md`](project-types.md)). For `software`, "defect", "fix", and "gate" are
literal. For `skills`, an item is a skill that mis-fires (wrong trigger, missing anti-pattern) and
the "gate" is the skills checklist; for `studies`, an item is a claim/source/consistency problem
caught on review of the delivered report. The discipline (register → route → categorize →
root-cause → fix+gate+commit per item → lesson) is identical for every type.

## Outputs

1. A **QA log** for the delivery, every item registered, categorized, and root-caused.
2. **One commit per fixed item**, each leaving the artifact gate-green.
3. **Lessons compiled into skills / knowledge** — the concrete edits, per item.
4. **Backlog entries** (`O-NNN`) for any durable opportunities surfaced.

## Completion Criteria

- [ ] Every reported item is in the QA log with a category (E/P/H) and a root cause.
- [ ] Each fix passes the project's quality gates and is committed on its own.
- [ ] Each item maps to ≥1 lesson recorded in a skill / knowledge file.
- [ ] Durable opportunities are promoted to the backlog; one-off fixes are not.
- [ ] No spec-defect was smuggled through this loop instead of being reopened.

## Anti-Patterns

- **Spec defect as QA** — routing a pre-`DONE` defect against the spec through this loop instead of
  reopening the milestone.
- **Patch without root cause** — fixing the symptom, skipping the mechanism, so the class recurs.
- **Silent fix** — correcting a defect without recording the lesson into a skill; the gate that
  missed it stays blind.
- **Batched commits** — several QA fixes in one commit; the cause→fix trail is lost.
- **Backlogging a one-off** — inflating the backlog with throwaway fixes that belong only in the QA
  log.

## Template: QA log

One file per delivery cycle (e.g. `history/NNN-qa-corrections.md`). Header directives → a summary
table → one section per item.

```markdown
# NNN — Post-delivery QA (corrections)

> Log of **post-delivery corrections** for <artifact>, from defects found in **human / real-world
> testing** of the delivery. Complements the (DONE) iteration record NNN-<name>.md.
>
> **Directives for this cycle:**
> 1. **Route by type** — visual/layout/interaction polish → single-agent (applied directly);
>    substantial/logic/cross-region → multi-agent (spec → review → implement).
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

**Root cause.** <the actual mechanism>

**Category.** **(E)**/**(P)**/**(H)** — <why>, primary; <secondary if any>.

**Fix.** <what changed, which files>

**Verification.** <gates run + result; smoke/boot; what remains human-confirmed>

**Lesson → skill.** <which skill / knowledge file, what to add so the class is caught next time>
```
