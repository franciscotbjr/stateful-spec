# Design Source Methodology

A structured, technology-agnostic framework for designing and building software with AI assistance.

## Philosophy

Software development with AI assistants works best when both human and AI operate within a shared, explicit process. Ad-hoc prompting produces inconsistent results. A methodology provides:

- **Predictability** — Both parties know what phase they're in and what's expected
- **Quality** — Defined gates prevent shipping incomplete or untested work
- **Continuity** — Session boundaries don't erase context; decisions are recorded
- **Scalability** — The same process works for a CLI tool, a web app, or a distributed system

## Core Principles

1. **Specification before implementation** — Write down what you're building before writing code
2. **Incremental delivery** — Ship working software in small, verifiable iterations
3. **Decisions are permanent artifacts** — Record architectural choices with rationale
4. **The Project Profile is the source of truth** — All technology-specific details live in one place
5. **Quality gates are non-negotiable** — Every iteration passes the project's defined checks
6. **AI is a collaborator, not an oracle** — The human owns decisions; the AI accelerates execution

## The Iteration Cycle

Every unit of work follows 5 phases:

```
┌─────────────────────────────────────────┐
│              1. ANALYZE                  │
│  Understand requirements, break down    │
│  complexity, identify unknowns          │
└────────────────────┬────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────┐
│               2. PLAN                    │
│  Define architecture, structure,        │
│  milestones, and blockers               │
└────────────────────┬────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────┐
│              3. SPECIFY                  │
│  Write technical refinement docs for    │
│  each unit of work                      │
└────────────────────┬────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────┐
│             4. IMPLEMENT                 │
│  Build, test, integrate following       │
│  the specification and plan             │
└────────────────────┬────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────┐
│              5. VERIFY                   │
│  Run quality gates, update docs,        │
│  prepare for delivery                   │
└─────────────────────────────────────────┘

         Repeat for each work unit.
```

## How It Works

### For the Human

1. **Fill the Project Profile** — Once per project, describe your tech stack, conventions, and quality gates using the `templates/project/project-profile.md` template (or copy a preset from `presets/`)
2. **Use prompts to drive the AI** — Copy prompts from the `prompts/` directory and paste them into your AI assistant alongside your Project Profile
3. **Write specs for each work unit** — Use the specification templates in `templates/specification/` to describe what needs to be built
4. **Follow the phases** — Move through Analyze → Plan → Specify → Implement → Verify for each feature, bugfix, or refactoring

### For the AI Assistant

1. **Read the Project Profile** — Understand the technology stack, conventions, and constraints
2. **Follow the current phase** — Don't skip ahead; each phase has defined inputs and outputs
3. **Reference the methodology** — Use the phase guides in `methodology/phases/` to stay on track
4. **Produce artifacts** — Each phase produces concrete deliverables (specs, plans, code, docs)
5. **Respect quality gates** — Never declare work complete until the project's quality checks pass

## Granularity

The methodology adapts to work of any size:

| Work Size | How to Apply |
|-----------|-------------|
| **Small** (bugfix, typo, config change) | Analyze + Implement + Verify (skip Plan and Specify) |
| **Medium** (new feature, API endpoint) | All 5 phases, single iteration |
| **Large** (new module, major refactor) | All 5 phases, multiple iterations with sub-tasks |
| **Project bootstrap** | Use the `new-project` initialization prompt, then iterate |

## Directory Structure

```
design-source/
├── methodology/          # Core process documentation
│   ├── overview.md       # This file
│   ├── phases/           # Detailed guide for each phase
│   ├── roles.md          # AI assistant role and expectations
│   └── decision-framework.md
├── prompts/              # LLM-ready prompts (copy-paste into any AI)
│   ├── initialization/   # Start/resume projects
│   ├── phase-transitions/# Move between phases
│   └── operations/       # Common tasks (commit, docs, review, etc.)
├── templates/            # User-fillable templates
│   ├── project/          # Project Profile, ADR
│   ├── specification/    # Feature, endpoint, component, bugfix, refactor specs
│   └── implementation/   # Implementation plan, test plan
├── presets/              # Pre-filled Project Profiles for common stacks
└── examples/             # Community-contributed applied examples
```

## Getting Started

1. Read the [phase guides](phases/) to understand the workflow
2. Copy a [preset](../presets/) or fill the [Project Profile template](../templates/project/project-profile.md)
3. Use the [new-project prompt](../prompts/initialization/new-project.md) to bootstrap your project with an AI assistant
4. Follow the iteration cycle for each unit of work
