# Prompt: Create Skill Specification

> **Applies to:** skills. The skills analog of `create-technical-spec`. See [`methodology/project-types.md`](../../methodology/project-types.md).

## Context

Use this prompt to produce a specification for a new Agent Skill before authoring it. It combines analysis and specification into a single interaction using the `skill-spec.md` template.

## Prerequisites

- AI has access to the **Project Definition** (Project Type: skills)
- You can describe the bias, phenomenon, or behavior the skill should address
- You have the `skill-spec.md` template available

## Input

Paste the following alongside this prompt:

1. Your **Project Definition** (if the AI doesn't already have it in context)
2. A **description of the skill** (what behavior it corrects or enables)
3. The **`skill-spec.md` template** (copy from `templates/specification/`)

## Prompt

<!-- Copy from here -->

I need a specification for a new Agent Skill. Please analyze the request and produce a complete specification using the template provided.

**Skill description:**

{{SKILL_DESCRIPTION}}

**Specification template to follow:**

{{PASTE_THE_TEMPLATE_HERE}}

**Instructions:**

**Before starting:** Check `.stateful-spec/memory.md` for an Open Session section. If found, you are working within an open implementation cycle — all contributions must be registered under that iteration.

1. Identify the target bias/phenomenon precisely — what does the agent do wrong (or fail to do) that this skill fixes?
2. Draft the `description` trigger so it covers the intended cases and excludes the rest — specific and actionable
3. List the concrete anti-patterns the skill must catch
4. Propose at least two before/after example pairs, each demonstrating one rule
5. State "When NOT to apply" to keep the trigger from over-matching
6. Ground every normative claim in a source; flag any claim you cannot ground
7. Note a bilingual counterpart if one should exist

The specification should be detailed enough to author the skill without further clarification.

**Session tracking:** If an Open Session was detected, append a timestamped entry to the **Session Log** in the referenced iteration file summarizing what this specification covers.

<!-- To here -->

## Expected Output

A complete `skill-spec.md` document, ready for review and authoring.

## Next Steps

- Review the specification for completeness and grounding
- Resolve any open questions
- Use `write-examples` to author the before/after pairs and self-check checklist
