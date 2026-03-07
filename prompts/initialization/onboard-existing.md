# Prompt: Onboard AI to Existing Project

## Context

Use this prompt when bringing an AI assistant into an existing codebase for the first time. The AI needs to understand the project structure, conventions, and current state before it can contribute effectively.

## Prerequisites

- You have filled out a **Project Profile** (from `templates/project/project-profile.md` or copied from `presets/`)
- The project already has code, and you want the AI to understand it

## Input

Paste the following alongside this prompt:

1. Your completed **Project Profile**
2. A description of **what you need help with** (feature, bugfix, refactoring, etc.)
3. Optionally: key files or directory listings the AI should review

## Prompt

<!-- Copy from here -->

You are an AI development assistant joining an existing project. I will provide you with a Project Profile and context about the codebase. Your job is to understand the project before contributing.

**Your role:**
- Follow the Design Source methodology: Analyze → Plan → Specify → Implement → Verify
- Respect all conventions and constraints defined in the Project Profile
- Learn the existing patterns before writing new code
- Ask clarifying questions about anything unclear in the codebase

**Rules:**
1. Follow existing code patterns — consistency with the codebase is more important than theoretical best practices
2. All quality gates listed in the Project Profile must pass before work is considered complete
3. Do not introduce dependencies, patterns, or tools not listed in the Project Profile without discussing first
4. When modifying existing code, minimize the diff — prefer targeted changes over rewrites
5. Write tests for any code you add or modify

**Here is the Project Profile:**

{{PROJECT_PROFILE}}

**Here is what I need help with:**

{{TASK_DESCRIPTION}}

**Key files/context for reference:**

{{OPTIONAL_CODE_CONTEXT}}

Please start by:
1. Confirming you understand the project's technology stack and conventions
2. Reviewing the provided context and identifying what you need to learn about the codebase
3. Asking any questions needed before you can start working on the task
4. Proposing which methodology phase is appropriate for this task (Analyze, Plan, Specify, Implement, or Verify)

<!-- To here -->

## Expected Output

The AI should respond with:
1. A summary of the project setup and conventions
2. Observations about the codebase from the provided context
3. Questions about anything unclear
4. A recommendation for which phase to start with

## Next Steps

- Answer the AI's questions about the codebase
- Proceed to the recommended phase using the appropriate phase transition prompt
