# Project Definition

> Fill this template once per project. It is the single source of technology-specific information that all prompts and AI interactions reference.

> **Project Type drives this file.** The `Project Type` field below selects which
> variable sections apply. Sections that vary by type carry one subsection per type
> (software / skills / studies) following the conditional-section convention: keep the
> subsection for your Project Type and delete the others. See
> [`methodology/project-types.md`](../../methodology/project-types.md) for the registry.

---

## Project Identity

- **Project Name:** [e.g., my-awesome-app]
- **Description:** [One-sentence description of what the project does]
- **Project Type:** software | skills | studies   <!-- determines which sections below apply; `software` is the default -->
- **Category (software only, optional):** [e.g., library | web app | CLI | API service | mobile app | data pipeline]
- **Repository URL:** [e.g., https://github.com/user/repo]
- **License:** [e.g., MIT, Apache-2.0, proprietary]

## Stack / Materials

> **Applies to all types.** Keep the subsection for your Project Type; delete the others.

### Stack / Materials — software

> Applies to: software

#### Language(s)

| Language | Version | Role |
|----------|---------|------|
| [e.g., TypeScript] | [e.g., 5.x] | [Primary / Secondary / Scripting] |

#### Framework(s)

| Framework | Version | Purpose |
|-----------|---------|---------|
| [e.g., Next.js] | [e.g., 14.x] | [Web framework] |

#### Key Dependencies

| Dependency | Version | Purpose |
|------------|---------|---------|
| [e.g., Prisma] | [e.g., 5.x] | [ORM / database access] |

#### Build System & Package Manager

- **Package Manager:** [e.g., npm, pnpm, cargo, pip, go modules]
- **Build Tool:** [e.g., Vite, webpack, cargo, setuptools, go build]
- **Task Runner:** [e.g., npm scripts, Makefile, just, none]

### Stack / Materials — skills

> Applies to: skills

- **Format:** Markdown + YAML frontmatter (name, description)
- **Target agents:** [e.g., Claude Code, OpenCode]
- **Tooling:** No build, package manager, lint, or CI

### Stack / Materials — studies

> Applies to: studies

- **Subject area:** [e.g., NLP, economic history]
- **Sources / corpus:** [e.g., bibliography, datasets, primary sources]
- **Tooling:** [e.g., LaTeX, Zotero, Python/R, Jupyter]

## Repository Structure

```
[Describe or paste the top-level directory structure]
project/
├── src/
├── tests/
├── docs/
└── ...
```

### Key Directories

| Directory | Purpose |
|-----------|---------|
| [e.g., src/] | [Source code] |
| [e.g., tests/] | [Test files] |
| [skills, e.g., <skill-name>/] | [A skill directory: SKILL.md + README.md] |
| [studies, e.g., references/, papers/, data/, figures/] | [Bibliography, deliverables, datasets, figures] |

## Conventions

> **Applies to all types.** Keep the subsection for your Project Type; delete the others.

### Conventions — software

> Applies to: software

#### Naming

| Item | Convention | Example |
|------|-----------|---------|
| Files | [e.g., kebab-case] | [e.g., user-service.ts] |
| Functions/Methods | [e.g., camelCase] | [e.g., getUserById] |
| Classes/Types | [e.g., PascalCase] | [e.g., UserService] |
| Constants | [e.g., SCREAMING_SNAKE_CASE] | [e.g., MAX_RETRIES] |
| Variables | [e.g., camelCase] | [e.g., currentUser] |

#### Code Style

- **Formatter:** [e.g., prettier, black, rustfmt]
- **Formatter Config:** [e.g., .prettierrc, pyproject.toml, default]
- **Max Line Length:** [e.g., 100, 120, no limit]
- **Indentation:** [e.g., 2 spaces, 4 spaces, tabs]
- **Import Order:** [Describe convention, e.g., stdlib → external → internal]

#### Patterns & Conventions

- **Error Handling:** [Describe pattern, e.g., Result types, try/catch with custom errors, error boundaries]
- **Logging:** [e.g., structured logging with pino, console.log, log crate]
- **Configuration:** [e.g., environment variables, config files, dotenv]
- **API Design:** [e.g., RESTful, GraphQL, RPC — describe patterns used]
- **State Management:** [e.g., Redux, Zustand, Context API — for frontend projects]
- **Other:** [Any project-specific patterns worth noting]

### Conventions — skills

> Applies to: skills

- **Skill structure:** Two files per skill — `SKILL.md` + `README.md`
- **Trigger:** `description` is the trigger condition — specific and actionable
- **Language:** Skills are monolingual; the root README is bilingual with cross-references
- **Content pattern:** anti-patterns, before/after pairs, self-check checklist, "When NOT to apply"

### Conventions — studies

> Applies to: studies

- **Citation style:** [e.g., APA, ABNT, IEEE]
- **Notation and symbols:** [describe]
- **Figure / table / section numbering:** [describe]
- **Writing language and register:** [e.g., academic English]

## Verification

> **Applies to all types.** Keep the subsection for your Project Type; delete the others.
> What "verification" means per type is defined in
> [`methodology/project-types.md`](../../methodology/project-types.md) (Terminology mapping).

### Verification — software

> Applies to: software

#### Strategy

- **Unit Tests:** [e.g., co-located with source, in tests/ dir, both]
- **Integration Tests:** [e.g., separate directory, docker-compose, test database]
- **E2E Tests:** [e.g., Playwright, Cypress, none]
- **Test Framework:** [e.g., Jest, pytest, cargo test, go test]
- **Mocking:** [e.g., jest.mock, mockito, unittest.mock, none]
- **Coverage Target:** [e.g., 80%, no formal target]

#### Test Naming Convention

[e.g., `test_should_return_user_when_id_exists`, `it('should return 404 for missing user')`]

### Verification — skills

> Applies to: skills

- Trigger validation (description covers intended cases, excludes the rest)
- Self-check checklist present and satisfied
- Before/after examples demonstrate the rule
- Every normative claim is grounded in a source

### Verification — studies

> Applies to: studies

- Every claim traceable to a cited, resolvable source
- Analysis is reproducible from data + scripts
- Internal consistency (no contradictory statements)

## Quality Gates

> **Applies to all types.** These must ALL pass before work is considered complete.
> Keep the subsection for your Project Type; delete the others.

### Quality Gates — software

> Applies to: software — commands that must pass.

```bash
# Linter
[e.g., npm run lint]

# Formatter check
[e.g., npm run format:check]

# Type check
[e.g., npx tsc --noEmit]

# Tests
[e.g., npm test]

# Build
[e.g., npm run build]
```

### Quality Gates — skills

> Applies to: skills — checklist (no commands to run).

- [ ] Directory has exactly SKILL.md + README.md
- [ ] `description` specific and actionable
- [ ] Skill monolingual; bilingual counterpart cross-referenced in root README (if any)
- [ ] Anti-patterns, before/after, "When NOT to apply" present
- [ ] Prose follows sober-prose (no rhetorical padding)
- [ ] Claims grounded in sources
- [ ] Root README skill table updated

### Quality Gates — studies

> Applies to: studies — checklist (no commands to run).

- [ ] All citations complete and resolvable
- [ ] No unsourced normative claims; no plagiarism
- [ ] Figures/tables referenced in text and numbered
- [ ] Analysis reproducible (seeds, scripts, environment noted)
- [ ] Argument coherent end to end

## Documentation

### Required Documentation Files

| File | Purpose |
|------|---------|
| [e.g., README.md] | [Project overview, setup, usage] |
| [e.g., CHANGELOG.md] | [Version history] |
| [e.g., API.md] | [API documentation] |
| [skills, e.g., root README.md] | [Skill index / table; bilingual cross-references] |
| [studies, e.g., references.bib, methodology.md] | [Bibliography; method and reproducibility notes] |

### Documentation Style

- **Code Comments:** [e.g., JSDoc, docstrings, rustdoc, minimal]
- **Doc Examples:** [e.g., runnable, no_run, pseudocode]
- **Non-software:** [skills, e.g., before/after examples per skill; studies, e.g., citation style and figure captions]

## Delivery / Distribution

> **Applies to all types.** Keep the subsection for your Project Type; delete the others.

### Delivery / Distribution — software

> Applies to: software

- **Target Environment:** [e.g., Vercel, AWS Lambda, Docker, crates.io, npm]
- **CI/CD:** [e.g., GitHub Actions, GitLab CI, none]
- **Branch Strategy:** [e.g., main + feature branches, trunk-based, gitflow]

### Delivery / Distribution — skills

> Applies to: skills

- Consumed directly from the repo (e.g., opencode.json skills.paths)
- Publish = merge to main via PR; no external pipeline
- **Branch Strategy:** [e.g., main + feature branches]

### Delivery / Distribution — studies

> Applies to: studies

- **Target venue/format:** [e.g., internal report, arXiv, journal]
- **Dataset sharing:** [e.g., repository, DOI, none]
- **Branch Strategy:** [e.g., main + feature branches]

## Constraints & Non-Negotiables

> List any hard constraints the AI must always respect.

- [e.g., No unsafe code without justification]
- [e.g., All public APIs must have documentation]
- [e.g., No new dependencies without discussion]
- [e.g., Must support Node 18+]
- [e.g., Must remain backward compatible with v2.x API]
- [skills, e.g., Every normative claim must be grounded in a source; no rhetorical padding]
- [studies, e.g., No unsourced claims; analyses must be reproducible from committed data + scripts]
