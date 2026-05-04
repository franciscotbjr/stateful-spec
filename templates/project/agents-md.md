# [Project Name] — AI Agent Instructions

This project uses the **Stateful Spec** methodology for AI-assisted development.

## Getting Started

1. **Read `.stateful-spec/memory.md`** — your entry point for current project state
2. **Read `.stateful-spec/project-definition.md`** — technology stack, conventions, quality gates
3. Read the methodology digest below. For deeper reference see `.stateful-spec/methodology/`.

To **refresh** methodology, prompts, or agent rules from upstream in an already configured repo, use [`prompts/initialization/update-project.md`](prompts/initialization/update-project.md).

## Methodology

Follow the 5-phase iteration cycle: **Analyze → Plan → Specify → Implement → Verify**

### Core Principles

1. **Specification before implementation** — Write down what you're building before writing code
2. **Incremental delivery** — Ship working software in small, verifiable iterations
3. **Decisions are permanent artifacts** — Record architectural choices with rationale
4. **The Project Definition is the source of truth** — All technology-specific details in one place
5. **Quality gates are non-negotiable** — Every iteration passes the project's defined checks
6. **AI is a collaborator, not an oracle** — Human owns decisions; AI accelerates execution

### Phase Summary

| Phase | Goal | Key Output | Skip When |
|-------|------|------------|-----------|
| 1. Analyze | Understand requirements, break down complexity, identify unknowns | Requirements summary, complexity breakdown, dependency list, open questions | — |
| 2. Plan | Define architecture, milestones, identify blockers | Architecture sketch, milestone list, blocker list, ADRs if significant | Small bugfixes with obvious scope |
| 3. Specify | Write detailed technical specs so implementation has no ambiguity | Filled specification documents with measurable acceptance criteria | Trivial changes (typos, config, dependency bumps) |
| 4. Implement | Build, test, integrate following the specification and project conventions | Working code, tests, inline docs, clean incremental commits | — |
| 5. Verify | Confirm quality standards, update docs, prepare for delivery | All quality gates passing, updated documentation, clean delivery artifact | — |

**Implementation order:** Data/Types → Core Logic → Integration → Tests → Documentation

### Phase Anti-Patterns

| Phase | Avoid |
|-------|-------|
| 1. Analyze | Jumping to code before understanding; assuming requirements; analysis paralysis |
| 2. Plan | Over-planning simple tasks; big-bang delivery; ignoring unresolved blockers |
| 3. Specify | Writing spec after code; over-specifying trivial work; only specifying the happy path |
| 4. Implement | Implementing without a spec; big-bang commits; tests as afterthought; gold plating |
| 5. Verify | Skipping quality gates; forgetting documentation; scope creep during review |

### Roles & Boundaries

**Human developer** owns all decisions (architecture, scope, priorities). Reviews and approves AI output before committing. Fills the Project Definition, writes specs, drives the process.

**AI assistant** follows the methodology, stays in the current phase, produces concrete artifacts. Follows the Project Definition's conventions. Asks when requirements are unclear. Identifies risks and edge cases.

**AI must NOT:**
- Make architectural decisions without human approval
- Skip methodology phases
- Introduce dependencies or patterns not in the Project Definition without discussion
- Commit, deploy, or make irreversible changes without explicit instruction
- Assume requirements — ask when unclear

### Collaboration Model

- **Start:** Use `start-session`. AI reads Project Definition + iteration context. Human states goal and phase.
- **During:** Human provides direction. AI follows the phase, produces artifacts, human reviews.
- **End:** Use `end-session`. AI summarizes, updates iteration file, moves work to Recent Completions.

### Decision Framework

For significant choices (architecture, patterns, tools):

1. **Identify** — State what and why
2. **List alternatives** — Pros/cons for each
3. **Apply criteria** (priority order): consistency > simplicity > reversibility > convention > team familiarity
4. **Decide and record** — Use the ADR template (`templates/project/architecture-decision.md`)
5. **Communicate** — Reference ADR in PRs, update Project Definition if conventions change

When in doubt: ask the human. When time-pressured: make the reversible choice and create a follow-up task. Revisit decisions when new information emerges — create a new ADR that supersedes the old one; never delete old ADRs.

### Quality Gates

All quality gates defined in the **Project Definition** must pass before work is considered complete. Typical gates: linter, formatter, type checker, tests, build. Never skip a gate or declare work done prematurely.

> Full phase guides, detailed role descriptions, and extended examples are in `.stateful-spec/methodology/`.

## Operation Prompts

The following operations are available as native commands in supported tools:

- **Cursor:** invoke with `@name` (rules in `.cursor/rules/`)
- **Claude Code / OpenClaude:** invoke with `/name` (commands in `.claude/commands/`)
- **OpenCode:** invoke with `/name` (commands in `.opencode/commands/`)

| Operation | Purpose |
|-----------|---------|
| `start-session` | Start a new session — creates iteration file and marks it as open |
| `end-session` | End the current session — summarizes work and closes the iteration |
| `resume-session` | Resume work — loads project context and picks up where you left off |
| `save-session` | Save session progress — updates memory.md and iteration files |
| `create-technical-spec` | Write a technical specification for new work |
| `write-tests` | Generate tests for existing or new code |
| `debug-issue` | Diagnose and fix a bug with structured root cause analysis |
| `refactor-code` | Safely restructure code without changing behavior |
| `review-changes` | Self-review code changes before committing |
| `write-commit-message` | Generate a well-structured commit message |
| `update-documentation` | Update docs after implementing a change |

Source prompts live in `prompts/operations/`. The tool-specific files (`.cursor/rules/`, `.claude/commands/`, `.opencode/commands/`) mirror these sources.

## Rules for All Work

1. All content must follow the conventions in the Project Definition
2. All quality gates must pass before work is considered complete
3. Do not introduce patterns or structures not in the Project Definition without discussing first
4. When modifying existing content, minimize the diff — prefer targeted changes over rewrites
5. Make small, logical commits that leave the repository in a working state
6. Track iterations in `.stateful-spec/history/` and keep `.stateful-spec/memory.md` current

### Iteration tracking

For every **non-trivial** work unit (feature, bugfix, refactor, methodology or documentation change driven by a plan or spec), **before** substantive implementation:

- Create `.stateful-spec/history/NNN-[kebab-name].md` using `templates/implementation/iteration.md`, where `NNN` is the next number after existing files in `history/`.
- Update **Active Work** and **History Index** in `.stateful-spec/memory.md` when starting; move completed work to **Recent Completions** when done.

**Trivial** edits may skip a new iteration file (e.g. typo fix, obvious one-line correction with no acceptance criteria). When in doubt, create an iteration file — it keeps audits straightforward.

If the session **starts with a direct task** (e.g. "implement this plan") instead of a full session-resume dialog, still create or attach to an iteration file **before** implementing.

### Session lifecycle

Use `start-session` at the beginning of an implementation cycle to create an iteration file and mark it as the **Open Session** in `memory.md`. While a session is open, every operation prompt (spec writing, code review, documentation, debugging, etc.) registers its contributions to the Session Log of the open iteration file. Use `end-session` to summarize all work, close the iteration, and clear the Open Session flag.

If an agent instance detects an Open Session in `memory.md`, it should maintain that session — appending entries and not creating competing iterations. If asked to start a new session while one is open, the agent must ask for approval to close the existing one first.
