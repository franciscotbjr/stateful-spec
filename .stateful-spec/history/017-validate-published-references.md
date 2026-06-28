# Iteration: 017 — Validate Published References

> One file per feature, bugfix, or refactor. Track progress and decisions here.

## Metadata

- **Type:** chore
- **Status:** in-progress
- **Created:** 2026-06-28
- **Completed:** —
- **Author:** Francisco Tarcizo Bomfim Júnior

## Description

Validate the published references now live in their registries — the two **optional** reference
implementations of the multi-agent flow contract published in iterations 015–016:

- `stateful-spec-flow` v0.1.0 on **crates.io** (O-002 / iteration 015)
- `@stateful-spec/flow` v0.1.0 on **npm** (O-003 / iteration 016)

Confirm the published artifacts install/resolve as advertised, and that every place the repo
points at them (docs, `packages/` READMEs, `project-definition.md`, CHANGELOG) cites the correct
published **name** and **version** and matches what was actually published.

Drives backlog opportunity **O-004** (intake `prd.md`, idea).

## Acceptance Criteria

> Checkboxes for what "done" means. These come from the specification or user story.

- [x] `stateful-spec-flow` v0.1.0 confirmed live on crates.io and resolvable
- [x] `@stateful-spec/flow` v0.1.0 confirmed live on npm and resolvable
- [x] Every in-repo reference (name + version) to both packages audited against what was published
- [x] Any drift (wrong name, stale version, broken link) corrected with a minimal diff — none found
- [x] O-004 routed to a destination (promoted/closed) in `backlog.md`

## Implementation Tasks

> Breakdown of work. Check off as you go.

- [x] Inventory in-repo references to both packages (grep name/version across docs + packages)
- [x] Verify crates.io publication of `stateful-spec-flow` v0.1.0 (name, version, resolves)
- [x] Verify npm publication of `@stateful-spec/flow` v0.1.0 (name, version, resolves)
- [x] Reconcile each reference against the published truth; fix drift — no drift found
- [x] Update CHANGELOG / project-definition if anything changed — no changes needed
- [x] Close out O-004 in backlog.md (promoted → iteration 017)

## Quality Checks

> Standard checks from the Project Definition. Verify before marking done.

- [x] All quality gates pass (manual review — methodology core is documentation-only)
- [x] References match published name + version exactly
- [x] Documentation updated (if applicable) — none required; references already correct
- [x] No debug code or TODOs left behind

## Session Log

> Timestamped entries recording each operation performed during this session.
> Agents append entries automatically when an Open Session is active.

| Timestamp | Operation | Summary |
|-----------|-----------|---------|
| 2026-06-28 14:50 | start-session | Session opened for O-004 — validate published references (crates.io + npm). |
| 2026-06-28 15:05 | implement | Validated both registries live: crates.io sparse index + API confirm `stateful-spec-flow@0.1.0` (yanked:false, MSRV 1.96, keywords/categories as published); npm confirms `@stateful-spec/flow@0.1.0` (dist-tag latest, MIT, repo→packages/flow-ts). Audited all in-repo refs (manifests, README, packages/README, AGENTS, project-definition, multi-agent-flow, 4 start-multi-agent-flow ports) against published truth — name/version/desc/repo all match, **no drift**. Promoted O-004 → iteration 017 in backlog.md. |

> **Timestamp format:** `YYYY-MM-DD HH:MM` (local time). Example: `2026-05-03 14:30 | start-session | Session opened for feature work.`
>
> **Note:** Iterations created prior to the session management feature may lack this section. This is expected and does not require migration.

## Decisions Made

> Decisions made during this iteration. Include rationale.

| Decision | Rationale | Date |
|----------|-----------|------|
| Treat O-004 as a chore (validation), not a feature | No new behavior is added — the work confirms published artifacts and reconciles references | 2026-06-28 |

## Blockers & Notes

> Anything that blocked progress or is worth remembering.

- Verifying live registry state may require network access (crates.io / npm). If unavailable, validate in-repo reference consistency and flag the live check as pending.
- Live checks succeeded. crates.io rejects requests without a `User-Agent` (API data-access policy) — use the sparse index `https://index.crates.io/st/at/stateful-spec-flow` or send a UA. npm registry read API: `https://registry.npmjs.org/@stateful-spec%2Fflow`.
- Both packages publish identical-description, zero-dep CLIs; published metadata matches the committed `Cargo.toml` / `package.json` exactly (incl. MSRV 1.96 and node `>=24`). No reference drift anywhere in the repo — zero corrective edits needed.

## References

- **Specification:** —
- **PR/MR:** —
- **Commits:** —
- **Related Issues:** backlog `O-004`; intake `.stateful-spec/intake/Backlog/prd.md`
