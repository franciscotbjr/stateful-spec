# Project Definition

> This is the single source of technology-specific information that all prompts and AI interactions reference.

---

## Project Identity

- **Project Name:** Stateful Spec
- **Description:** A specification-driven development methodology for AI-assisted software projects
- **Project Type:** Documentation / Methodology
- **Repository URL:** https://github.com/franciscotbjr/stateful-spec
- **License:** MIT

## Technology Stack

### Language(s)

| Language | Version | Role |
|----------|---------|------|
| Markdown | N/A | Primary — all content is `.md` files |

### Framework(s)

None — this is a documentation-only project.

### Key Dependencies

None — no runtime or build-time dependencies.

### Build System & Package Manager

- **Package Manager:** None
- **Build Tool:** None
- **Task Runner:** None

## Repository Structure

```
stateful-spec/
├── methodology/            # Core process documentation
│   ├── overview.md         # Philosophy, principles, iteration cycle
│   ├── project-types.md    # Project Type registry (software/skills/studies)
│   ├── roles.md            # Human vs AI responsibilities (+ multi-agent flow)
│   ├── decision-framework.md
│   ├── backlog.md          # Intake + backlog (O-NNN) pipeline
│   ├── history-archiving.md # Context budget: history/.archived/ + RAW_HISTORY
│   ├── qa-phase.md         # Post-delivery QA loop (E/P/H)
│   ├── multi-agent-flow.md # Optional two/three-agent milestone flow (software)
│   └── phases/             # 5 phase guides (01-analyze through 05-verify)
├── prompts/                # LLM-ready prompts
│   ├── initialization/     # new-project, onboard-existing, update-project
│   ├── phase-transitions/  # start-analysis through start-verification
│   └── operations/         # operation prompts (lifecycle + per-type + multi-agent flow)
├── templates/              # Fill-in templates
│   ├── project/            # project-definition, memory, architecture-decision, agents-md
│   ├── specification/      # feature, endpoint, component, bugfix, refactor, skill, study
│   ├── implementation/     # implementation-plan, test-plan, iteration, verification-plan,
│   │                       #   flow-state, review-handoff, arch-verdict
│   ├── intake/             # Backlog/Discovery/QA intake scaffold (+ READMEs)
│   ├── backlog.md          # O-NNN backlog template
│   ├── intake-item.md      # Single intake-item template
│   └── qa-log.md           # Post-delivery QA log template
├── presets/                # Pre-filled project definitions (software/skills/studies)
├── packages/               # Optional reference implementations of the flow tooling contract
│   ├── flow-rs/            #   stateful-spec-flow (Rust → crates.io, zero-dep)
│   ├── flow-ts/            #   @stateful-spec/flow (Node/TS → npm, zero runtime deps)
│   └── conformance/        #   shared fixtures (Rust ≡ TS parity)
├── examples/               # Community examples (placeholder)
├── .stateful-spec/         # Project memory instance (this project's own Stateful Spec)
│   ├── memory.md           #   state, Engramas, History Index
│   ├── project-definition.md
│   ├── backlog.md          #   triaged opportunities (O-NNN)
│   ├── intake/             #   raw inbox: Backlog/, Discovery/, QA/
│   └── history/            #   one file per iteration (+ .archived/ cold store)
├── .cursor/rules/          # Cursor-native operation prompts
├── .claude/commands/       # Claude Code / OpenClaude operation commands
├── .opencode/commands/     # OpenCode operation commands
├── CLAUDE.md               # Claude Code entry point (imports AGENTS.md)
├── AGENTS.md
├── CHANGELOG.md
├── LICENSE
└── README.md
```

### Key Directories

| Directory | Purpose |
|-----------|---------|
| `methodology/` | Core process documentation — the source of truth for the methodology |
| `prompts/` | Ready-to-use prompts organized by category (initialization, phase-transitions, operations) |
| `templates/` | Fill-in templates for project setup, specs, implementation, and intake/backlog/QA |
| `presets/` | Pre-filled Project Definitions (software, skills, studies) |
| `packages/` | Optional, separately-published reference implementations of the multi-agent flow tooling contract (`stateful-spec-flow` / `@stateful-spec/flow`) — **not required** to use the methodology |
| `examples/` | Placeholder for community-contributed applied examples |
| `.stateful-spec/` | This project's own memory, project definition, backlog, intake inbox, and iteration history |

## Code Conventions

### Naming

| Item | Convention | Example |
|------|-----------|---------|
| Files | kebab-case.md | `project-definition.md` |
| Directories | lowercase | `phase-transitions/` |
| Phase files | NN-kebab-case.md | `01-analyze.md` |
| Iteration files | NNN-kebab-case.md | `001-feature-name.md` |

### Content Style

- **Formatter:** None
- **Max Line Length:** No formal limit
- **Indentation:** 2 spaces for nested lists, 4 spaces for code blocks
- **Import Order:** N/A

### Patterns & Conventions

- Some prompt files use YAML frontmatter with a `description:` field
- Templates use placeholder syntax: `[e.g., example value]` or `{{VARIABLE}}`
- Prompts use step-by-step wizard format with developer confirmation gates
- Section headers use ATX-style Markdown (`#`, `##`, `###`)
- Tables use GitHub Flavored Markdown pipe syntax

## Testing

Not applicable — documentation-only project.

## Quality Gates

> No automated quality gates. Quality is maintained through manual review.

```bash
# No linter, formatter, type checker, tests, or build commands
```

## Documentation

### Required Documentation Files

| File | Purpose |
|------|---------|
| `README.md` | Project overview, get-started guide, operation/preset listing |
| `CHANGELOG.md` | Version history (Keep a Changelog format) |
| `methodology/overview.md` | Philosophy, principles, iteration cycle |

### Documentation Style

- **Code Comments:** N/A
- **Doc Examples:** Inline in Markdown with fenced code blocks

## Deployment

- **Target Environment:** GitHub repository (public)
- **CI/CD:** None
- **Branch Strategy:** main + feature branches with PRs

## Constraints & Non-Negotiables

- The methodology content is Markdown — no application code in the repository **except** the optional `packages/` reference implementations of the multi-agent flow tool (separately published to crates.io / npm; not required to use the methodology)
- File naming must follow existing kebab-case convention
- CHANGELOG entries must follow Keep a Changelog format
- New presets must match the structure of `templates/project/project-definition.md`
- New prompts must include clear instructions, expected inputs, and expected outputs
- When modifying source prompts in `prompts/operations/`, the AI must also update the corresponding `.cursor/rules/<name>.mdc`, `.claude/commands/<name>.md`, and `.opencode/commands/<name>.md` files
- When modifying methodology source files in `methodology/`, no sync is needed — `.stateful-spec/methodology/` references the source directly
- Non-trivial work must use an iteration file under `.stateful-spec/history/` (see `AGENTS.md` Iteration tracking and `prompts/operations/resume-session.md`); update `memory.md` when starting or completing work
