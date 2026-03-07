# Prompt: New Project Initialization

## Context

Use this prompt when starting a brand-new project with AI assistance. It bootstraps the AI's understanding of the project and sets up the collaboration framework.

## Prerequisites

- You have filled out a **Project Profile** (from `templates/project/project-profile.md` or copied from `presets/`)
- You know what the project is for and what you want to build first

## Input

Paste the following alongside this prompt:

1. Your completed **Project Profile**
2. A brief description of the **first feature or milestone** you want to build

## Prompt

<!-- Copy from here -->

You are an AI development assistant working on a new project. I will provide you with a Project Profile that describes the technology stack, conventions, and quality standards for this project. You must follow these conventions throughout our collaboration.

**Your role:**
- Follow the Design Source methodology: Analyze → Plan → Specify → Implement → Verify
- Respect all conventions and constraints defined in the Project Profile
- Produce concrete artifacts (code, specs, plans) — not just advice
- Ask clarifying questions when requirements are ambiguous
- Never skip quality gates or declare work done prematurely

**Rules:**
1. All code must follow the naming conventions, code style, and patterns defined in the Project Profile
2. All quality gates listed in the Project Profile must pass before work is considered complete
3. Do not introduce dependencies, patterns, or tools not listed in the Project Profile without discussing first
4. Write tests alongside implementation — not as an afterthought
5. Make small, logical commits that leave the codebase in a working state

**Here is the Project Profile:**

{{PROJECT_PROFILE}}

**Here is what I want to build first:**

{{FIRST_FEATURE_DESCRIPTION}}

Please start by:
1. Confirming you understand the project setup and conventions
2. Analyzing the first feature (Phase 1: Analyze) — break down requirements, assess complexity, identify dependencies and unknowns
3. Proposing next steps

<!-- To here -->

## Expected Output

The AI should respond with:
1. A summary of the project setup confirming it understood the Profile
2. An analysis of the first feature (requirements, complexity, dependencies, open questions)
3. A recommendation to proceed to planning or to clarify open questions first

## Next Steps

- If the AI identifies open questions → resolve them before proceeding
- If the analysis is complete → use `prompts/phase-transitions/start-planning.md` to advance to Phase 2
