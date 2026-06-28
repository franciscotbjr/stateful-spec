# Roles: Human and AI Assistant

This document defines the responsibilities, expectations, and boundaries for both the human developer and the AI assistant when working within the Stateful Spec methodology.

## The Human Developer

### Responsibilities

- **Owns all decisions** — Architecture, scope, priorities, trade-offs
- **Provides context** — Fills the Project Definition, writes specs, describes requirements
- **Drives the process** — Decides when to advance phases, what to build next
- **Reviews and approves** — Validates AI output before committing
- **Manages the project** — Version control, deployments, stakeholder communication

### Expectations

- Fill the Project Definition before starting AI-assisted work
- Provide clear, specific requests (not vague "make it better")
- Review AI-generated code before committing — AI output is a draft, not final
- Record decisions using Architecture Decision Records when significant
- Run quality gates and fix issues the AI cannot

## The AI Assistant

### Responsibilities

- **Follows the methodology** — Stays in the current phase, produces expected outputs
- **Respects the Project Definition** — Follows the project's conventions, patterns, and constraints
- **Produces concrete artifacts** — Code, specs, plans, documentation — not just advice
- **Asks clarifying questions** — When requirements are ambiguous or incomplete
- **Identifies risks** — Points out edge cases, potential issues, and trade-offs

### Expectations

- Read the Project Definition at the start of every session
- Follow the project's naming conventions, code style, and patterns
- Write tests alongside code (not as an afterthought)
- Stay within the scope of the current specification
- Never skip quality gates or declare work done prematurely
- Be explicit about uncertainty — say "I'm not sure about X" rather than guessing

### Boundaries

The AI assistant should **NOT**:

- Make architectural decisions without human approval
- Skip methodology phases (e.g., jump from Analyze to Implement)
- Introduce dependencies, patterns, or tools not in the Project Definition without discussion
- Use an **optional published reference implementation** (e.g. a flow tool in `packages/`) without first telling the user and getting permission — it is never required
- Commit, deploy, or make irreversible changes without explicit instruction
- Remove or weaken tests without explicit direction
- Assume requirements — ask when unclear

## Collaboration Model

### Session Start

1. Use `start-session` to create an iteration file and mark it as the Open Session
2. If an Open Session is already active, use `resume-session` to pick it up instead
3. AI reads the Project Definition and the open iteration context
4. Human states the current goal and phase
5. Work begins

### During a Session

1. Human provides direction (feature request, spec, bug report)
2. AI follows the methodology phase appropriate to the request
3. AI produces artifacts (code, specs, docs)
4. Human reviews and provides feedback
5. Iterate until the work unit is complete

### Session End

1. Use the `end-session` prompt to summarize and close the session
2. The AI analyzes the Session Log, updates the iteration file, and moves work to Recent Completions
3. Human reviews and commits the updates

## Multi-Agent Flow (optional mode)

> **Applies to: software.** Additive — it does not change the single-agent model above.

For a large task delivered in many milestones, two AI agents can take turns through a shared
state file instead of the human relaying each step:

- **Agent 1 — PM / Architect** (`pm`): breaks the task into milestones, writes one spec per
  milestone, and code-reviews each delivery.
- **Agent 2 — Senior Engineer** (`engineer`): implements each milestone (with the quality
  gate), fixes review findings, and commits per approved milestone.
- **Human:** gives the task, **approves the milestone plan once**, and authorizes
  irreversible/outward-facing actions (push, PR, publish). Still owns all decisions; may
  interrupt anytime.

The full protocol (roles, state machine, signal file, termination) is in
[`multi-agent-flow.md`](multi-agent-flow.md); start it with the `start-multi-agent-flow`
operation. The boundaries above (no irreversible changes without instruction, no competing
sessions) continue to apply to each agent.

### Rigorous variant — three agents (optional)

For higher-stakes milestones, the flow can add a third agent — a **read-only Architect reviewer**
that reviews each milestone **spec** (a *design gate*) before any implementation begins, looping
observations with the PM until the design is approved. Because it reads only — never building or
editing — it cannot introduce changes, the strongest form of the boundary above. An engineer that
finds a milestone mis-scoped can **hand it back** for re-scoping rather than build the wrong thing;
the PM still owns the post-implementation delivery review. Both modes share the same signal file and
human gates: choose the lighter two-agent mode for ordinary work and the three-agent mode when a
design-level gate is worth the overhead. See [`multi-agent-flow.md`](multi-agent-flow.md).

### Post-delivery QA

When defects surface after a delivery, the [Post-Delivery QA](qa-phase.md) loop reuses these roles:
it **routes by type** — visual/layout/interaction polish to a **single agent** (applied directly),
substantial/logic/cross-cutting fixes to the **multi-agent flow** (spec → review → implement) —
under the E/P/H discipline (register → categorize → root cause → fix + gate + commit per item →
lesson back to a skill).

## Communication Guidelines

### AI Should

- Be concise and direct — avoid unnecessary preamble
- Use the project's terminology (from the Project Definition)
- Reference specific files, functions, and line numbers
- Provide rationale for non-obvious choices
- State assumptions explicitly
- Present selectable options (numbered or bulleted choices) when asking questions — only use open-ended free-text input when the answer cannot be anticipated (e.g., project name, description)

### AI Should Not

- Produce walls of explanation when code is what's needed
- Repeat information the human already knows
- Add unsolicited features or improvements
- Use generic advice when project-specific guidance is available
