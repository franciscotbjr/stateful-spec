# Iteration: 018 — Apply Preset Rust

> One file per feature, bugfix, or refactor. Track progress and decisions here.

## Metadata

- **Type:** feature
- **Status:** in-progress
- **Created:** 2026-06-28
- **Completed:** —
- **Author:** Francisco Tarcizo Bomfim Júnior

## Description

Apply the external **"new - 03 - preset rust"** product-requirements document (backlog **O-005**) —
shape and implement a Rust preset for the methodology.

The concrete requirements live in an external PRD in the developer's working area, **outside this
repo** (`…/stateful-spec/fluxo/backlog/new - 03 - preset rust/product requirements document.md`).
That document must be read and shaped during the **Analyze → Plan** phase before any implementation.

**Scope (settled with developer 2026-06-28):** O-005 is a **revision of the existing
`rust-library` preset** — the single file [`presets/rust-library.md`](../../presets/rust-library.md)
— **not** a new preset. The revisions come from learnings extracted from the
`D:\development\public\stand-in` project (its crates/codebase + memory & history docs + Rust-specific
`.claude/skills`).

The PRD's second ask — **propose other rust presets** (app/ui, design-system, …) — is handled as a
**written proposal + new `O-NNN` backlog opportunities**, NOT built in this iteration (decided with
the developer 2026-06-28).

## Acceptance Criteria

> Checkboxes for what "done" means. These come from the specification or user story.
> _Placeholders — to be filled from the external PRD during the Analyze/Plan phase._

- [x] External PRD read and its requirements shaped into concrete revisions to `presets/rust-library.md`
- [x] Learnings extracted from the `stand-in` project (codebase + memory/history + Rust skills) and mapped to the preset
- [ ] `presets/rust-library.md` revised with the applicable learnings (mirrors `templates/project/project-definition.md`) — drafted in `scratchpad/rust-library-final.md`, **pending developer review before apply**
- [x] Other rust presets (app/ui, design-system) proposed in writing and filed as new `O-NNN` backlog opportunities (O-006 `rust-gpui-app`, O-007 `rust-design-system`)

## Implementation Tasks

> Breakdown of work. Check off as you go.
> _To be expanded once the PRD scope is settled._

- [x] Read and analyze the external PRD; resolve the design decision tree (document alternatives)
- [x] Analyze the `stand-in` project: crates/codebase, memory & history docs, Rust-specific `.claude/skills`
- [x] Extract candidate learnings; map each onto the current `presets/rust-library.md` (what changes, what stays)
- [ ] Apply the revisions to `presets/rust-library.md` (mirror `templates/project/project-definition.md` structure) — **pending developer review** of `scratchpad/rust-library-final.md`
- [x] Draft the proposal for additional rust presets (app/ui, design-system); file each as a new `O-NNN` in `backlog.md` (O-006, O-007)
- [ ] Documentation / CHANGELOG update if applicable — with the apply step

## Quality Checks

> Standard checks from the Project Definition. Verify before marking done.

- [ ] All quality gates pass (manual review — methodology core is documentation-only)
- [ ] New/changed preset mirrors the structure of `templates/project/project-definition.md`
- [ ] Documentation updated (if applicable)
- [ ] No debug code or TODOs left behind

## Session Log

> Timestamped entries recording each operation performed during this session.
> Agents append entries automatically when an Open Session is active.

| Timestamp | Operation | Summary |
|-----------|-----------|---------|
| 2026-06-28 15:30 | start-session | Session opened for O-005 — apply the external "new - 03 - preset rust" PRD. Close triage promoted intake `prd.md` → O-005; this iteration takes it. |
| 2026-06-28 15:40 | analyze | Scope settled with developer: O-005 = **revise the existing `presets/rust-library.md`** (not a new preset). Awaiting developer permission to read the external PRD before planning. |
| 2026-06-28 15:50 | analyze | Read the external PRD (captured below). It directs: analyze the `D:\development\public\stand-in` project (codebase + memory/history + Rust `.claude/skills`), extract learnings for the `rust-library` preset, AND propose other rust presets (app/ui, design-system, …). Surfaced a scope tension between the PRD's two-part ask and the earlier "revision only" decision; asking the developer to reconcile. |
| 2026-06-28 15:55 | analyze | Scope tension reconciled with developer: revise `rust-library` now; the "propose other presets" half becomes a written proposal + new `O-NNN` backlog opportunities (not built in 018). Next prerequisite: read access to `D:\development\public\stand-in`. |
| 2026-06-28 16:05 | analyze | Access granted; surveyed `stand-in`: 11-crate Cargo workspace (MCP lib `stand-in`/`-macros`, client SDK `stand-in-client`/`-macros`, GPUI app `stand-in-mcp-explorer` + spike, egui app `stand-in-mcp-studio`, DS crate `stand-in-mcp-studio-ds` + prototype, `stand-in-reference`, `sync-crates`), 22 `.claude/skills` (rust-pro, gpui-*, design-system/frontend/ux, mcp), and a deep `.stateful-spec/` history (iters 035-038 + archived). Launched multi-agent extraction workflow (9 readers → 3 syntheses → adversarial verify). |
| 2026-06-28 16:25 | analyze | Workflow done (15 agents, ~1.4M tok): **338 learnings** (175 rust-library / 116 app-ui / 95 design-system). rust-library synthesis = 12 changes; adversarial verify: 0 drop / 4 keep / 8 revise, flagging bloat + 3 residual stand-in leaks. Produced a tightened final preset applying every verifier trim (dropped binary-only `[profile.release]`, demoted RPC two-error-plane note, dropped sibling-binary test recipe, genericized `exclude=`, deduped `env!`, trimmed multi-workspace asides + Constraints dup) → `scratchpad/rust-library-final.md`. Both new-preset proposals verified as well-justified (no `rust-library` duplication). |
| 2026-06-28 16:30 | plan | Developer decisions: (1) will **read the tightened preset first** before I apply it to `presets/rust-library.md` — apply pending review; (2) app preset slug = **`rust-gpui-app`** (framework-specific, over the neutral alternatives); (3) **file both** new presets — added **O-006** (`rust-gpui-app`) and **O-007** (`rust-design-system`) to `backlog.md`, status `new`, with verifier-refined descriptions. |

> **Timestamp format:** `YYYY-MM-DD HH:MM` (local time). Example: `2026-05-03 14:30 | start-session | Session opened for feature work.`
>
> **Note:** Iterations created prior to the session management feature may lack this section. This is expected and does not require migration.

## Decisions Made

> Decisions made during this iteration. Include rationale.

| Decision | Rationale | Date |
|----------|-----------|------|
| O-005 scope = revise the existing `presets/rust-library.md`, not create a new preset | Developer confirmed; the repo already ships a `rust-library` preset, so the PRD's requirements are applied as revisions to it | 2026-06-28 |
| Handle the PRD's "propose other presets" half as a written proposal + new `O-NNN` backlog opportunities; build only the rust-library revision in 018 | Reconciles the PRD's two-part ask with the "revision only" scope — proposals become tracked opportunities for later iterations without scope-creeping 018 | 2026-06-28 |
| Tighten the synthesis draft per the adversarial verifier before proposing (drop binary-only `[profile.release]`, demote RPC two-error-plane note, drop sibling-binary test recipe, genericize `exclude=`, dedupe `env!`, trim multi-workspace asides + Constraints dup) | Verifier flagged bloat + 3 residual stand-in leaks; the preset must stay terse and generic per preset philosophy | 2026-06-28 |
| App/ui preset slug = `rust-gpui-app` (framework-specific), over the verifier's framework-neutral lean (`rust-ui-app`/`rust-desktop-app`) | Developer preference — most evidence-true to the GPUI source; GPUI-only mechanics still called out, egui noted as a variant | 2026-06-28 |
| File both new-preset proposals now as O-006 (`rust-gpui-app`) + O-007 (`rust-design-system`), status `new` | Developer approved; records the opportunities without scope-creeping 018 | 2026-06-28 |

## Blockers & Notes

> Anything that blocked progress or is worth remembering.

- The PRD is **external** to this repo and may not be readable from the agent's working directory; if
  inaccessible, the developer must supply its contents before the Plan phase can proceed.
- The intake item carries a curator **persona + exhaustive-questioning protocol**: investigate
  hypotheses, ask one question at a time, walk every branch of the design decision tree resolving
  dependencies one by one, and document all alternatives considered with the rationale for each
  choice. Honor this protocol during Analyze/Plan. Questions answerable from the repo should be
  resolved without asking.

- **External PRD captured verbatim (source lives outside this repo, may change there):**
  > Since the latest methodology updates were extracted from `D:\development\public\stand-in` — a
  > project with several crates heavily modified/evolved, including several skills at
  > `D:\development\public\stand-in\.claude\skills` — you should: (1) analyze the entire codebase,
  > (2) analyze all memory and history documents, (3) analyze the Rust-specific skills. **Extract
  > learnings:** extract all learnings applicable to the `rust-library` preset; **propose other rust
  > presets** (e.g. app/ui, design system, etc.).

- **Open scope tension:** the PRD asks for BOTH (a) revise the `rust-library` preset AND (b) propose
  other rust presets. The earlier decision settled "revision only." Reconciliation pending with the
  developer (see Decisions).
- **Second access prerequisite:** extracting learnings requires **read access to
  `D:\development\public\stand-in`** (codebase, memory/history, `.claude/skills`) — another external
  path. Permission still to be requested.

## References

- **Specification:** — (to be created via `create-technical-spec` once the PRD is shaped)
- **PR/MR:** —
- **Commits:** —
- **Related Issues:** backlog `O-005`; intake `.stateful-spec/intake/Backlog/prd.md`
