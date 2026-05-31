# Iteration: 007 — Multi-Project-Type Support

> One file per feature, bugfix, or refactor. Track progress and decisions here.

## Metadata

- **Type:** feature
- **Status:** review
- **Created:** 2026-05-31
- **Completed:** —
- **Author:** Francisco Tarcizo Bomfim Júnior

## Description

Make `Project Type` a first-class, behavior-driving dimension so the methodology
natively supports non-software projects. Adds two new types beyond `software`:

- **skills** — repositories of Agent Skills (Markdown prompt extensions; zero-code).
- **studies** — research/study projects (literature reviews, analyses, reports, papers).

`software` remains the default; existing projects behave exactly as before. Driven
by a central registry (`methodology/project-types.md`) plus conditional sections in
shared templates/prompts. Implements `EVOLUTION-PLAN-multi-project-type.md`.

## Acceptance Criteria

> From the EVOLUTION-PLAN §11 verification checks.

- [x] **Skills dry-run:** `onboard-existing.md` pointed at a skills repo detects `skills`, generates a project-definition with the skills subsections, emits the skills operation set + native commands + AGENTS table. *(Verified by structural walk-through of the wizard path.)*
- [x] **Studies dry-run:** pointed at a repo with `.bib`/`.tex`/notebooks detects `studies`; correct sections/ops. *(Verified by structural walk-through.)*
- [x] **Software regression:** a repo with `package.json`/`Cargo.toml` produces the same output as before this change — software subsections carry the original content verbatim.
- [x] **Sync rule:** each new operation has `.claude/`, `.opencode/`, `.cursor/` ports (19 files in each command dir).
- [x] **Consistency:** `methodology/project-types.md` matches the conditional subsections in the templates; all internal links resolve.

## Implementation Tasks

> From the EVOLUTION-PLAN §12 suggested execution order.

- [x] Create `methodology/project-types.md` (registry with full skills/studies detail).
- [x] Edit `templates/project/project-definition.md` (load-bearing Project Type + branched sections).
- [x] Edit `methodology/overview.md`, `phases/04-implement.md`, `phases/05-verify.md`.
- [x] Create `templates/specification/skill-spec.md` and `study-spec.md`; tag existing five specs.
- [x] Update `templates/implementation/iteration.md`; tag `test-plan.md`; create `verification-plan.md`.
- [x] Create skills + studies operations and their three native ports each; tag software-only ops.
- [x] Edit the three initialization wizards for Project Type detection/selection + type-aware generation.
- [x] Create `presets/skills-repo.md` and `presets/studies-project.md`; update `presets/README.md`, root `README.md`, `templates/project/agents-md.md`, root `AGENTS.md`/`CLAUDE.md`.
- [x] Run §11 verification (dry-runs + software regression + sync + links).

## Quality Checks

> Standard checks from the Project Definition. Verify before marking done.

- [x] Conventions followed (English, kebab-case, ATX headings, GFM tables, placeholder syntax)
- [x] No application code, build tooling, or CI introduced (documentation-only)
- [x] `prompts/operations/*` kept in sync with `.claude/`, `.opencode/`, `.cursor/` ports
- [x] All internal links resolve
- [x] Backward compatibility preserved (software subsections carry current content verbatim)

## Session Log

> Timestamped entries recording each operation performed during this session.
> Agents append entries automatically when an Open Session is active.

| Timestamp | Operation | Summary |
|-----------|-----------|---------|
| 2026-05-31 | start | Opened iteration to execute EVOLUTION-PLAN-multi-project-type.md. |
| 2026-05-31 | implement | Created registry + 2 presets + 2 spec templates + verification-plan + 8 operations (×4 ports) ; branched project-definition; made overview/phases/wizards/READMEs type-aware. |
| 2026-05-31 | verify | §11 checks passed (sync rule 19×4, registry/template consistency, software content verbatim, links resolve). Status → review (awaiting commit/PR). |

## Decisions Made

> Decisions made during this iteration. Include rationale.

| Decision | Rationale | Date |
|----------|-----------|------|
| Mechanism = load-bearing `Project Type` field + conditional sections + central registry | Single source of truth; one artifact serves all types (no N copies); extensible | 2026-05-31 |
| `software` stays default; its subsection carries current content verbatim | Guarantees backward compatibility — existing projects unchanged | 2026-05-31 |
| Closed stale Open Session [005] (merged via PR #23) when opening this iteration | memory.md pointer was stale; 005 already merged | 2026-05-31 |

## Blockers & Notes

> Anything that blocked progress or is worth remembering.

- This repo's own `.stateful-spec/` stays `software`/documentation-only — not reclassified (EVOLUTION-PLAN §10).
- `impl.md` and `plan.md` at the root are unrelated personal scratch drafts — left untouched.

## References

- **Specification:** `EVOLUTION-PLAN-multi-project-type.md`
- **PR/MR:** [Link when available]
- **Commits:** [Key commit hashes or range]
- **Related Issues:** —
