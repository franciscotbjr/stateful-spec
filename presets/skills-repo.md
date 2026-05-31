# Project Definition — Agent Skills Repository

> Pre-filled preset for a **skills** project: a repository of Agent Skills (Markdown
> prompt extensions; zero-code). Customize for your project. See
> [`methodology/project-types.md`](../methodology/project-types.md).

---

## Project Identity

- **Project Name:** [your-skills-repo]
- **Description:** [One-sentence description — e.g., prose-quality skills for AI agents]
- **Project Type:** skills
- **Repository URL:** [https://github.com/user/repo]
- **License:** [e.g., MIT]

## Stack / Materials

- **Format:** Markdown + YAML frontmatter (`name`, `description`)
- **Target agents:** [e.g., Claude Code, OpenCode]
- **Tooling:** No build, package manager, lint, or CI — quality is manual review

## Repository Structure

```
project/
├── README.md                 # Bilingual index with a table of all skills
├── <skill-name>/
│   ├── SKILL.md              # YAML name/description + instructions
│   └── README.md             # Human-facing explanation
├── <another-skill>/
│   ├── SKILL.md
│   └── README.md
└── opencode.json             # Optional: skills.paths for consumption
```

### Key Directories

| Directory | Purpose |
|-----------|---------|
| `<skill-name>/` | One directory per skill: `SKILL.md` + `README.md` |

## Conventions

- **Skill structure:** Two files per skill — `SKILL.md` + `README.md`
- **Trigger:** `description` is the trigger condition — specific and actionable; it must cover the intended cases and exclude the rest
- **Language:** Skills are monolingual; the root README is bilingual (e.g., EN/PT-BR) with cross-references between language-paired skills
- **Content pattern:** anti-patterns → before/after pairs → self-check checklist → "When NOT to apply"
- **Prose:** sober prose — no rhetorical padding (the skills enforce this on agents, so the repo holds itself to it)
- **Naming:** `kebab-case` skill directory names (e.g., `sober-prose`, `verbosity-reduction`)

## Verification

- Trigger validation (the `description` matches the intended cases and excludes the rest)
- Self-check checklist present and satisfied
- Before/after examples demonstrate each rule (one rule per pair)
- Every normative claim is grounded in a source

## Quality Gates

> Checklist (no commands to run). All must hold before a skill is considered done.

- [ ] Directory has exactly `SKILL.md` + `README.md`
- [ ] `description` specific and actionable
- [ ] Skill monolingual; bilingual counterpart cross-referenced in root README (if any)
- [ ] Anti-patterns, before/after, "When NOT to apply" present
- [ ] Prose follows sober-prose (no rhetorical padding)
- [ ] Claims grounded in sources
- [ ] Root README skill table updated

## Documentation

### Required Documentation Files

| File | Purpose |
|------|---------|
| README.md (root) | Bilingual index; table of all skills with one-line descriptions |
| `<skill>/README.md` | Human-facing explanation and before/after examples for the skill |

### Documentation Style

- **Skill docs:** before/after examples per skill; sober, instructional register
- **Cross-references:** language-paired skills link to each other from the root README

## Delivery / Distribution

- Consumed directly from the repo (e.g., `opencode.json` `skills.paths`)
- Publish = merge to main via PR; no external pipeline

## Constraints & Non-Negotiables

- Every normative claim must be grounded in a source
- No rhetorical padding; skills hold themselves to the standard they enforce
- Each skill stays monolingual; provide a bilingual counterpart instead of mixing languages
- No build tooling, dependencies, or CI — the repo is Markdown-only
