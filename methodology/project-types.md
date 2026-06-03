# Project Types

> Stateful Spec supports multiple project types. The Project Type is recorded in
> `project-definition.md` (Project Identity) and determines which conventions,
> templates, and operations apply. `software` is the default — if the Project Type
> is absent or `software`, the methodology behaves exactly as it always has.

This file is the single source that the initialization wizards, phase guides, and
operation prompts reference. Templates and prompts *point here* instead of
duplicating per-type logic.

## Registry

| Type | One-liner | Default? |
|------|-----------|----------|
| software | Code projects (libraries, services, apps). | Yes |
| skills | Repositories of Agent Skills (Markdown prompt extensions; zero-code). | No |
| studies | Research/study projects (reviews, analyses, reports, papers). | No |

The registry is designed to accept new types later. To add one, append a row here,
then add its detection signals, terminology row, and a "Per-type detail" section
below — and provide the matching conditional subsections in the templates.

## Detection signals (used by the initialization wizards)

| Type | Signals |
|------|---------|
| software | Manifests: `package.json`, `Cargo.toml`, `go.mod`, `pyproject.toml`, `setup.py`; `src/`, `tests/`; CI configs (`.github/workflows/`, `.gitlab-ci.yml`). |
| skills | Directories containing `SKILL.md`; `opencode.json` with `skills.paths`; skill `description:` frontmatter; references to agentskills.io. |
| studies | `.bib` / `.tex`; `references/`, `papers/`, `notes/`; Jupyter/R notebooks used for analysis; `data/` + `figures/`; `.csl` / Zotero files. |

When signals overlap (e.g. a study with analysis notebooks that also has a
`pyproject.toml`), pick the type that matches the **deliverable** — what the project
ships — and confirm with the developer.

## Terminology mapping (what each phase word means per type)

| Concept | software | skills | studies |
|---------|----------|--------|---------|
| Artifact | code module | a skill (SKILL.md + README.md) | a research deliverable (review/analysis/report/paper) |
| "Implement" (Phase 4) | write code | author the skill prose, before/after pairs, self-check checklist | gather sources, run analysis, draft sections |
| "Tests" | unit/integration tests | trigger validation + self-check + before/after examples | source-checking, reproducibility, internal-consistency review |
| "Quality gates" | lint/format/type/test/build | structure + trigger + sober-prose + grounding (see skills detail) | citations resolve + claims sourced + no plagiarism + reproducible |
| "Deliver" (Phase 5) | deploy / release / PR | consumed from repo; publish = merge via PR | publish/submit report or paper; share dataset |

## Per-type detail

### software

- **Applicable project-definition sections:** all current sections (unchanged).
- **Spec templates:** `feature-spec`, `endpoint-spec`, `component-spec`,
  `bugfix-spec`, `refactor-spec`.
- **Operations:** lifecycle (`start-session`, `end-session`, `resume-session`,
  `save-session`) + `create-technical-spec`, `write-tests`, `debug-issue`,
  `refactor-code`, `review-changes`, `update-documentation`, `write-commit-message`.
  Optional multi-agent mode: `start-multi-agent-flow` (two-agent milestone loop — see
  [`multi-agent-flow.md`](multi-agent-flow.md)).
- **Verification template:** `templates/implementation/test-plan.md` (via
  `verification-plan.md`).
- **Iteration `Type` values:** feature | bugfix | refactor | chore.

### skills

A repository of Agent Skills: Markdown prompt extensions, each a directory with
`SKILL.md` (YAML `name`/`description` + instructions) and `README.md`. Zero-code.

- **Detection signals.** Directories containing `SKILL.md`; `opencode.json` with
  `skills.paths`; `description:` frontmatter; references to agentskills.io.
- **Terminology.** *Artifact* = a skill. *Implement* = author the skill prose,
  before/after pairs, self-check checklist. *Tests* = trigger validation +
  self-check + before/after examples. *Deliver* = consumed directly from the repo;
  publish = merge to main via PR.

**project-definition skeleton (skills subsections):**

```markdown
- Project Type: skills
## Stack / Materials — skills
- Format: Markdown + YAML frontmatter (name, description)
- Target agents: [e.g., Claude Code, OpenCode]
- No build, package manager, lint, or CI
## Conventions — skills
- Two files per skill: SKILL.md + README.md
- `description` is the trigger condition — specific and actionable
- Skills are monolingual; root README is bilingual with cross-references
- Content pattern: anti-patterns, before/after pairs, self-check checklist, "When NOT to apply"
## Verification — skills
- Trigger validation (description covers intended cases, excludes the rest)
- Self-check checklist present and satisfied
- Before/after examples demonstrate the rule
- Every normative claim is grounded in a source
## Quality Gates — skills
- [ ] Directory has exactly SKILL.md + README.md
- [ ] `description` specific and actionable
- [ ] Skill monolingual; bilingual counterpart cross-referenced in root README (if any)
- [ ] Anti-patterns, before/after, "When NOT to apply" present
- [ ] Prose follows sober-prose (no rhetorical padding)
- [ ] Claims grounded in sources
- [ ] Root README skill table updated
## Delivery / Distribution — skills
- Consumed directly from the repo (e.g., opencode.json skills.paths)
- Publish = merge to main via PR; no external pipeline
```

- **Spec template:** `skill-spec.md` — Metadata (skill name, language, status);
  Target bias/phenomenon; Trigger (`description` draft); Anti-patterns to cover;
  Before/After examples; "When NOT to apply"; Source grounding; Bilingual
  counterpart (if any); Out of scope.
- **Operations:** lifecycle (`start-session`, `end-session`, `resume-session`,
  `save-session`) + `create-skill-spec`, `write-examples`, `diagnose-skill`,
  `revise-skill`, `review-changes`, `update-documentation`, `write-commit-message`.
- **Verification template:** `verification-plan.md` (before/after coverage +
  self-check).
- **Iteration `Type` values:** new-skill | skill-revision | chore.
- **Generic activity → operation:** *create spec* → `create-skill-spec`;
  *implement/examples* → `write-examples`; *debug* → `diagnose-skill`;
  *refactor/revise* → `revise-skill`; *write tests* → `write-examples` +
  `diagnose-skill` (trigger validation).
- **Preset:** `presets/skills-repo.md`.

### studies

A research/study project whose deliverables are documents, figures, and datasets —
literature reviews, analyses, reports, papers, study notes. May include analysis
notebooks, but the product is research, not shipped code.

- **Detection signals.** `.bib` / `.tex`; `references/`, `papers/`, `notes/`;
  Jupyter/R notebooks used for analysis; `data/` + `figures/`; `.csl` / Zotero files.
- **Terminology.** *Artifact* = a research deliverable. *Implement* = gather
  sources, run analysis, draft sections. *Tests* = verification (sources resolve
  and support claims, analysis reproducible, internal consistency). *Deliver* =
  publish/submit a report or paper; share dataset.

**project-definition skeleton (studies subsections):**

```markdown
- Project Type: studies
## Stack / Materials — studies
- Subject area: [e.g., NLP, economic history]
- Sources / corpus: [e.g., bibliography, datasets, primary sources]
- Tooling: [e.g., LaTeX, Zotero, Python/R, Jupyter]
## Conventions — studies
- Citation style: [e.g., APA, ABNT, IEEE]
- Notation and symbols: [describe]
- Figure / table / section numbering: [describe]
- Writing language and register: [e.g., academic English]
## Verification — studies
- Every claim traceable to a cited, resolvable source
- Analysis is reproducible from data + scripts
- Internal consistency (no contradictory statements)
## Quality Gates — studies
- [ ] All citations complete and resolvable
- [ ] No unsourced normative claims; no plagiarism
- [ ] Figures/tables referenced in text and numbered
- [ ] Analysis reproducible (seeds, scripts, environment noted)
- [ ] Argument coherent end to end
## Delivery / Distribution — studies
- Target venue/format: [e.g., internal report, arXiv, journal]
- Dataset sharing: [e.g., repository, DOI, none]
```

- **Spec template:** `study-spec.md` — Research question; Scope & boundaries;
  Sources/corpus; Method; Deliverable type (review/analysis/report/paper);
  Acceptance (claims grounded, reproducible); Out of scope; References.
- **Operations:** lifecycle (`start-session`, `end-session`, `resume-session`,
  `save-session`) + `create-study-spec`, `verify-sources`, `resolve-inconsistency`,
  `restructure-argument`, `review-changes`, `update-documentation`,
  `write-commit-message`.
- **Verification template:** `verification-plan.md` (source/claim verification +
  reproducibility).
- **Iteration `Type` values:** research | analysis | writing | revision | chore.
- **Generic activity → operation:** *create spec* → `create-study-spec`;
  *implement/verify sources* → `verify-sources`; *debug/inconsistency* →
  `resolve-inconsistency`; *refactor/restructure* → `restructure-argument`;
  *write tests* → `verify-sources` (source + reproducibility checks).
- **Preset:** `presets/studies-project.md`.

## How the layers use this registry

- **`project-definition.md`** keeps a single section per concept with per-type
  subsections (Stack / Materials, Conventions, Verification, Quality Gates,
  Delivery). During onboarding the wizard keeps only the active type's subsection.
- **Phase guides** (`phases/04-implement.md`, `phases/05-verify.md`) are read in
  place, not copied per project, so they keep all types and point here for the
  per-type meaning of "Implement", "Tests", "Quality gates", and "Deliver".
- **Operation prompts** are tagged with the type they apply to. Software-only
  prompts say *"Applies to: software"*; skills/studies have their own operations.
- **Initialization wizards** detect or ask the Project Type, then generate the
  active type's sections, emit the active type's operation set + native commands,
  and fill the `AGENTS.md` operation table accordingly.
