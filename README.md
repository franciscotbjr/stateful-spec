# Stateful Spec

A Specification Driven Development methodology for AI-assisted projects of any type (software, skills, studies, and more).

## Get Started

Pick one file below and give it to your AI assistant. You can do this by:
- **Copy & paste** the file content into your AI chat, or
- **Point your AI** to the file URL on GitHub, or
- **Reference the local file** if you cloned this repository

| Your situation | Give your AI this file |
|---|---|
| **Starting a new project** | [`prompts/initialization/new-project.md`](prompts/initialization/new-project.md) |
| **Adding AI to an existing project** | [`prompts/initialization/onboard-existing.md`](prompts/initialization/onboard-existing.md) |
| **Updating Stateful Spec in an existing project** | [`prompts/initialization/update-project.md`](prompts/initialization/update-project.md) |
| **Resuming previous work** | [`prompts/operations/resume-session.md`](prompts/operations/resume-session.md) |

**That's it.** The AI reads the instructions and walks you through everything else — project setup, conventions, and your first task.

---

## Example

Here's what it looks like in practice. You point your AI to an operation prompt and a business document:

> **You:** Read `create-technical-spec.md` and use it to write a spec based on `requirements/user-notifications.md`.

> **AI:** I've analyzed the requirements. Here's the technical specification:
>
> **Feature:** User Notification System
> **Complexity:** Medium — requires a new service, database table, and WebSocket integration
>
> **Acceptance Criteria:**
> 1. Users receive real-time notifications for mentions and replies
> 2. Notifications persist and can be marked as read
> 3. API supports pagination with a maximum of 50 notifications per page
>
> *[... full spec follows the project's template and conventions]*

That's the pattern — **you provide context, the AI follows the methodology.**

---

## How It Works

Stateful Spec gives your AI assistant a structured process to follow:

```
Analyze → Plan → Specify → Implement → Verify
```

When you initialize a project, the AI creates a `.stateful-spec/` folder in your project with:
- **Project Definition** — Your tech stack, conventions, and quality gates
- **Memory** — What's in progress, what was decided, what's next
- **Methodology** — The full process the AI follows
- **Operation prompts** — Ready-made prompts for common tasks (debug, refactor, write tests, etc.)

This makes your project **self-contained** — any AI assistant can pick up where you (or another AI) left off.

## Philosophy

**If you give an AI structured context and memory, it becomes a reliable collaborator instead of a stateless tool.**

## Project Types

Stateful Spec supports multiple **project types** — `software` (the default), `skills` (repositories of Agent Skills), and `studies` (research/study projects). The Project Type is recorded in your Project Definition and drives which conventions, templates, and operations apply. The 5-phase cycle is the same for every type; only what "implement", "tests", and "deliver" mean changes. See [`methodology/project-types.md`](methodology/project-types.md).

## Operation Prompts

Once set up, these prompts are available in your project as native agent commands:

- **Cursor:** `@resume-session`, `@save-session`, etc. (via `.cursor/rules/`)
- **Claude Code / OpenClaude:** `/resume-session`, `/save-session`, etc. (via `.claude/commands/`)
- **OpenCode:** `/resume-session`, `/save-session`, etc. (via `.opencode/commands/`)
- **Other tools:** point your AI at the source files in `prompts/operations/`

> The table below is the **software** operation set. The set varies by Project Type:
> `skills` projects get `create-skill-spec`, `write-examples`, `diagnose-skill`,
> `revise-skill`; `studies` projects get `create-study-spec`, `verify-sources`,
> `resolve-inconsistency`, `restructure-argument`. The lifecycle, review, docs, and
> commit-message ops are shared by all types. See [`methodology/project-types.md`](methodology/project-types.md).

| Prompt | Purpose |
|--------|---------|
| `start-session.md` | Start a new implementation session |
| `end-session.md` | End the current session and close the iteration |
| `resume-session.md` | Pick up where you left off |
| `save-session.md` | Save progress before ending a session |
| `create-technical-spec.md` | Write a spec for new work |
| `write-tests.md` | Generate tests for existing code |
| `debug-issue.md` | Diagnose and fix a bug |
| `refactor-code.md` | Safely restructure code |
| `review-changes.md` | Self-review code before committing |
| `write-commit-message.md` | Generate a structured commit message |
| `update-documentation.md` | Update docs after a change |
| `start-multi-agent-flow.md` | Optional: run a two-agent autonomous milestone loop (software) |

> **Multi-agent flow (optional, software).** For a large task delivered in many milestones,
> two agents can take turns through a shared state file — a PM/Architect that plans, specs,
> and reviews, and a Senior Engineer that implements, fixes, and commits. Start it with
> `start-multi-agent-flow pm` / `start-multi-agent-flow engineer`. See
> [`methodology/multi-agent-flow.md`](methodology/multi-agent-flow.md).

## Available Presets

Pre-filled Project Definitions — the AI will suggest one if it matches your project:

| Preset | Type | Stack / Materials |
|--------|------|-------------------|
| [`rust-library.md`](presets/rust-library.md) | software | Rust + Cargo + clippy/fmt |
| [`node-express-api.md`](presets/node-express-api.md) | software | Node.js + Express + Jest + ESLint |
| [`python-fastapi.md`](presets/python-fastapi.md) | software | Python + FastAPI + pytest + ruff |
| [`react-webapp.md`](presets/react-webapp.md) | software | React + TypeScript + Vite + Vitest |
| [`go-service.md`](presets/go-service.md) | software | Go + stdlib + go vet/test |
| [`skills-repo.md`](presets/skills-repo.md) | skills | Markdown + YAML frontmatter; no build |
| [`studies-project.md`](presets/studies-project.md) | studies | LaTeX / Zotero / Python / R |

## Learn More

- [`methodology/overview.md`](methodology/overview.md) — Philosophy, principles, and the iteration cycle
- [`methodology/project-types.md`](methodology/project-types.md) — Project Types registry (software, skills, studies)
- [`methodology/phases/`](methodology/phases/) — Detailed guide for each of the 5 phases
- [`methodology/roles.md`](methodology/roles.md) — Human and AI responsibilities
- [`methodology/decision-framework.md`](methodology/decision-framework.md) — How to make and record technical decisions
- [`templates/`](templates/) — All templates (project setup, specs, implementation plans)
- [`presets/`](presets/) — Pre-filled Project Definitions

## License

MIT
