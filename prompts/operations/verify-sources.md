# Prompt: Verify Sources

> **Applies to:** studies. The studies analog of `write-tests` — it verifies the work. See [`methodology/project-types.md`](../../methodology/project-types.md).

## Context

Use this prompt to verify a study: confirm that every claim is backed by a citation that resolves and actually supports it, and that any analysis is reproducible. This is the studies "verification" activity.

## Prerequisites

- AI has access to the **Project Definition** (Project Type: studies)
- The draft (or section) with claims and citations exists
- The bibliography and any analysis scripts/data are available

## Input

Paste the following alongside this prompt:

1. Your **Project Definition** (if the AI doesn't already have it in context)
2. The **draft text** with its claims and in-text citations
3. The **bibliography** (`.bib` or reference list) and, if relevant, analysis scripts/data

## Prompt

<!-- Copy from here -->

Please verify the sources and reproducibility for the following study material. Follow the conventions from the Project Definition (citation style, reproducibility expectations).

**Draft text:**

{{DRAFT_TEXT_OR_REFERENCE}}

**Bibliography / scripts / data:**

{{SOURCES_AND_ARTIFACTS}}

**Instructions:**

**Before starting:** Check `.stateful-spec/memory.md` for an Open Session section. If found, you are working within an open implementation cycle — all contributions must be registered under that iteration.

1. For **each normative claim**, identify its citation and check: does the reference exist, resolve, and actually support the claim?
2. Flag any **unsourced** claim or any citation that does not support the statement
3. Check the bibliography for **completeness** (no missing fields) and **no duplicates**
4. If there is an analysis, confirm it is **reproducible**: data available, scripts run end to end, seeds/parameters recorded, figures/tables regenerate
5. Note any **plagiarism risk** (uncited close paraphrase or quotation)
6. Produce a table of claim → source → status (resolves / supports / issue)

Do NOT rewrite the argument here — only verify and report. Use `resolve-inconsistency` or `restructure-argument` for changes.

**Session tracking:** If an Open Session was detected, append a timestamped entry to the **Session Log** in the referenced iteration file summarizing the verification results.

<!-- To here -->

## Expected Output

A claim → source → status table, a list of unsourced/unsupported claims, bibliography issues, and a reproducibility note.

## Next Steps

- Source or remove unsupported claims
- Fix bibliography issues
- Re-run the analysis if reproducibility gaps were found
