# Prompt: Resolve Inconsistency

> **Applies to:** studies. The studies analog of `debug-issue`. See [`methodology/project-types.md`](../../methodology/project-types.md).

## Context

Use this prompt when a study contains an internal contradiction — conflicting statements, a result that disagrees with the text, inconsistent terminology or notation, or a figure that doesn't match its description. The AI locates the contradiction and proposes a minimal resolution.

## Prerequisites

- AI has access to the **Project Definition** (Project Type: studies)
- You can point to the contradiction (or suspect there is one)

## Input

Paste the following alongside this prompt:

1. Your **Project Definition** (if the AI doesn't already have it in context)
2. The **conflicting passages** (or the sections to scan)
3. Any **data/result** that the text should agree with

## Prompt

<!-- Copy from here -->

I need help resolving an internal inconsistency in a study. Please locate the contradiction and propose a minimal resolution.

**Conflicting passages / sections:**

{{PASSAGES_OR_SECTIONS}}

**Relevant data / results (if any):**

{{DATA_OR_NONE}}

**Instructions:**

**Before starting:** Check `.stateful-spec/memory.md` for an Open Session section. If found, you are working within an open implementation cycle — all contributions must be registered under that iteration.

1. State the contradiction precisely — quote the conflicting statements and explain why they cannot both hold
2. Determine which statement is correct by checking it against the sources and any data/results
3. If it cannot be resolved from the material given, list what evidence would settle it
4. Propose a **minimal** correction that removes the contradiction without changing conclusions that are still supported
5. Check whether the same inconsistency appears elsewhere (terminology, notation, repeated claim) and list those occurrences

Do NOT restructure the whole argument here — fix the specific inconsistency. Use `restructure-argument` for larger reorganization.

**Session tracking:** If an Open Session was detected, append a timestamped entry to the **Session Log** in the referenced iteration file summarizing the inconsistency and resolution.

<!-- To here -->

## Expected Output

1. Precise statement of the contradiction
2. Which side is correct, with evidence (or what evidence is needed)
3. Minimal correction
4. Other occurrences of the same issue

## Next Steps

- Apply the correction consistently across the document
- Re-run `verify-sources` if the correction touched a cited claim
