# Prompt: Diagnose Skill

> **Applies to:** skills. The skills analog of `debug-issue`. See [`methodology/project-types.md`](../../methodology/project-types.md).

## Context

Use this prompt when a skill misbehaves — it fires when it shouldn't (over-matching), fails to fire when it should (under-matching), gives advice that isn't grounded, or its instructions conflict. The AI diagnoses the root cause and proposes a minimal fix.

## Prerequisites

- AI has access to the **Project Definition** (Project Type: skills)
- You can describe the misbehavior (what fired or didn't, and the expected behavior)

## Input

Paste the following alongside this prompt:

1. Your **Project Definition** (if the AI doesn't already have it in context)
2. The **skill** (`SKILL.md` + `README.md`)
3. The **observed behavior** (the input that triggered it, or the case it missed)
4. What you **expected** to happen

## Prompt

<!-- Copy from here -->

I need help diagnosing a skill that isn't behaving as intended. Please find the root cause and propose a minimal fix.

**Skill:**

{{SKILL_CONTENT_OR_REFERENCE}}

**Observed vs. expected behavior:**

{{OBSERVED_AND_EXPECTED}}

**Instructions:**

**Before starting:** Check `.stateful-spec/memory.md` for an Open Session section. If found, you are working within an open implementation cycle — all contributions must be registered under that iteration.

1. Classify the problem: trigger over-matching, trigger under-matching, ungrounded/incorrect guidance, or internal contradiction
2. Identify the most likely root cause — explain your reasoning (point to the `description`, a specific instruction, or a missing "When NOT to apply")
3. If uncertain, list the top 2-3 causes ranked by likelihood, each with a check (a test case that confirms or eliminates it)
4. Propose a **minimal** fix that addresses the root cause — prefer tightening the `description` or adding a single rule over rewriting the skill
5. Suggest a before/after example or trigger case that would catch this regression in the future

Do NOT broadly rewrite the skill during diagnosis. Focus on the specific misbehavior.

**Session tracking:** If an Open Session was detected, append a timestamped entry to the **Session Log** in the referenced iteration file summarizing the root cause and fix.

<!-- To here -->

## Expected Output

1. Problem classification and root-cause analysis
2. Checks (if the cause is uncertain)
3. Minimal fix
4. A regression example/trigger case

## Next Steps

- Apply the fix with `revise-skill` if it's more than a one-line change
- Add the regression example via `write-examples`
- Re-validate the trigger
