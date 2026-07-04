# Iteration: 022 — SDD Persona Adoption

> One file per feature, bugfix, or refactor. Track progress and decisions here.

## Metadata

- **Type:** feature
- **Status:** in-progress
- **Created:** 2026-07-04
- **Completed:** —
- **Author:** Francisco Tarcizo Bomfim Júnior

## Description

Adopt the externally-authored SDD-specialist persona — *Especialista em Pesquisa, Design e
Curadoria de Metodologias SDD* (authored 2026-07-03) — as **this repository's own agent persona**.

Scope clarification (developer, 2026-07-04): this is **not** a persona shipped to downstream
projects that use Stateful Spec; it is the persona of Stateful Spec itself — the agent that
researches, designs, and curates the methodology in this repo. Target = the self-application /
instruction layer only (root `AGENTS.md`, `.stateful-spec/`); **no shipped artifact**
(`methodology/`, `prompts/`, `templates/`, `presets/`) changes.

External sources (imported verbatim during this iteration):

- System prompt (operative): `D:\franciscotbjr\Documents\Explorações\Jandi\Agente - System Prompt - Especialista em Pesquisa, Design e Curadoria de Metodologias SDD.md`
- Reference doc (rationale + sources): `D:\franciscotbjr\Documents\Explorações\Jandi\Agente - Persona Especialista em Pesquisa, Design e Curadoria de Metodologias SDD.md`

Backlog: **O-009**. Source: `intake/Backlog/prd.md` ("Evolução do System Prompt da Persona para a
Statefull Spec"). Design resolved via exhaustive one-question-at-a-time Q&A (decisions D0–D9
recorded in **Decisions Made** below).

## Acceptance Criteria

> Checkboxes for what "done" means. These come from the specification or user story.

- [x] `.stateful-spec/persona.md` exists — verbatim copy of the external system prompt, differing
      only by one italic provenance line at top (origin path, authored/imported dates, companion,
      refresh trigger). Verified: `git diff --no-index` = 2 insertions, 0 deletions.
- [x] `.stateful-spec/persona-reference.md` exists — verbatim copy of the reference doc under the
      same provenance-line pattern, marked consult-on-demand (never bulk-read). Verified: 2
      insertions, 0 deletions.
- [x] Root `AGENTS.md` has a short self-only persona section: ambient binding, carve-outs
      (`packages/` → engineer posture; craft-protocol weight scales with task size), the §6
      checklist Sensor, and a self-declared intentional-divergence note vs
      `templates/project/agents-md.md`.
- [x] `.stateful-spec/project-definition.md` has one constraint line recording the binding and the
      intentional divergence.
- [x] `.stateful-spec/intake/Discovery/persona-ex-post-evaluation.md` exists with `status: draft` —
      the light ex-post evaluation hook (revisit after ~5 iterations with the persona's §8 signals).
- [x] `CHANGELOG.md` `[Unreleased]` records the adoption (Added).
- [x] The persona's §6 output checklist ran against this deliverable in a separate critique step and
      its outcome is logged here (first observable firing of the Sensor) — see Session Log
      2026-07-04 09:50.
- [x] No shipped artifact modified — `methodology/`, `prompts/`, `templates/`, `presets/`,
      `packages/`, `.cursor/`, `.claude/`, `.opencode/` untouched by this iteration. Verified:
      `git status --short` lists only `AGENTS.md`, `CHANGELOG.md`, and `.stateful-spec/**`.

## Implementation Tasks

> Breakdown of work. Check off as you go.

- [x] Import system prompt → `.stateful-spec/persona.md` (verbatim + provenance line)
- [x] Import reference doc → `.stateful-spec/persona-reference.md` (verbatim + provenance line)
- [x] Add self-only persona section to root `AGENTS.md`
- [x] Add constraint line to `.stateful-spec/project-definition.md`
- [x] Add Key Decisions line to `.stateful-spec/memory.md`
- [x] Seed `intake/Discovery/persona-ex-post-evaluation.md` (draft)
- [x] Update `CHANGELOG.md`
- [x] Run §6 checklist (separate critique step) and log outcome

## Quality Checks

> Standard checks from the Project Definition. Verify before marking done.

- [ ] All quality gates pass (manual review — documentation-only project; **pending developer review**)
- [x] Verbatim check: imported files differ from external sources only by the provenance line
      (`git diff --no-index`: +2/−0 each)
- [x] Documentation updated (CHANGELOG, memory.md)
- [x] No debug code or TODOs left behind
- [x] Persona §6 output checklist run in a separate critique step; outcome logged in Session Log

## Session Log

> Timestamped entries recording each operation performed during this session.
> Agents append entries automatically when an Open Session is active.

| Timestamp | Operation | Summary |
|-----------|-----------|---------|
| 2026-07-04 09:14 | start-session | Session opened for feature work (O-009 — adopt the SDD-specialist persona as this repo's own agent persona). Kickoff triage promoted `intake/Backlog/prd.md` → O-009; design tree resolved via exhaustive Q&A (D0–D9). |
| 2026-07-04 09:45 | implement | Adoption delivered per D0–D9: `persona.md` + `persona-reference.md` imported verbatim (UTF-8-safe programmatic copy; `git diff --no-index` = +2/−0 each — provenance line only); self-only Persona section in root `AGENTS.md`; constraint in `project-definition.md`; Key Decisions line in `memory.md`; ex-post hook seeded in `intake/Discovery/` (draft); CHANGELOG `[Unreleased]/Added` entry. Engram fold 012 → cold store executed at session open (rules 7/9). |
| 2026-07-04 09:50 | persona §6 checklist (separate critique step) | **First observable firing of the D4 Sensor. All 7 items PASS**: (1) anchors resolve (021/006/cold-store claims → files; empirical claims travel with imported `# Fontes`); (2) contrary evidence in body (Zheng / Playing Pretend / AGENTbench live inside the imported docs §2; rejected alternatives in Decisions Made); (3) method note = Blockers & Notes declares INCONCLUSIVO + resolution path; (4) removals/obsolescence declared (nothing shipped; template divergence declared in 3 places); (5) every mechanism has trigger + observable output (binding/Sensor/refresh/ex-post); (6) no improvement claim made — INCONCLUSIVO by abstention; (7) minimal diff — every clause traces to a decision. **Refutation found & accepted as declared risk:** ambient binding is Guide-level (a pointer, not a forced read) — an agent could skip `persona.md`; monitored by the ex-post hook ("a checklist that never fails is suspect"). |

> **Timestamp format:** `YYYY-MM-DD HH:MM` (local time). Example: `2026-05-03 14:30 | start-session | Session opened for feature work.`
>
> **Note:** Iterations created prior to the session management feature may lack this section. This is expected and does not require migration.

## Decisions Made

> Decisions made during this iteration. Include rationale.

| Decision | Rationale | Date |
|----------|-----------|------|
| **D0 — Target = this repo's own agent** (self-application), not downstream projects | Developer clarification: the persona belongs to Stateful Spec itself, not to projects that use it; downstream-facing adoption explicitly rejected | 2026-07-04 |
| **D1 — Binding = ambient with carve-outs** (persona governs every session via the AGENTS.md entry chain; `packages/` code → engineer posture per persona Boundary 5.1; craft-protocol weight scales with task size per rule 3.1) | Nearly all work here IS research/design/curation; scoped activation would need a trigger/Sensor and risks never activating; on-demand mode is a Guide without Sensor — optional in practice | 2026-07-04 |
| **D2 — Import both docs, verbatim** (`persona.md` operative + `persona-reference.md` rationale/sources, consult-on-demand) | Closes the harness hole the persona itself condemns (rationale on a personal external path, invisible to other clones); prompt-only import leaves the why-hole (rules 1.3/3.5 structurally violated); weaving rules into living docs destroys provenance | 2026-07-04 |
| **D2a — Placement = `.stateful-spec/`**, kebab-case rename, one italic provenance line per file | Only self-only non-shipped home fitting conventions; original filenames have spaces/accents; the provenance line is a minimal declared deviation needed because in-doc cross-references cite external filenames | 2026-07-04 |
| **D3 — Binding point = self-only section in root `AGENTS.md`**, divergence self-declared + constraint in project-definition.md | Iteration 021 made template↔copy drift a failure mode, so the divergence must be deliberate and declared; a persona slot in the shipped template fails the persona's own subtraction test downstream; project-definition-only binding is a weaker Guide (AGENTS.md is the canonical entry since 006) | 2026-07-04 |
| **D4 — Sensor = §6 checklist in a separate critique step, outcome logged in the open iteration** | Guide without Sensor is instruction without enforcement (persona D4); iteration files are self-only instances → auditable in `history/` with zero shipped changes; generic conditional wiring in shipped ops was rejected as scope leak | 2026-07-04 |
| **D5 — Conflicts reconciled without edits**: 5.1 vs `packages/` → D1 carve-out; 5.4 ≡ existing no-push policy; epistemic rules are cheap on trivial tasks (the diff is the anchor); persona formats govern craft deliverables while roles.md concise style governs session conversation; rule 3.4 requires a separate step, not a separate agent (`review-changes` is that step); in multi-agent flow, editing methodology Markdown IS the persona's craft — only `packages/`-touching milestones trigger the carve-out | All resolved by reading the persona against `methodology/roles.md`, root `AGENTS.md`, and repo policy — no contradiction requiring artifact changes | 2026-07-04 |
| **D6 — Language**: imported docs stay PT-BR (verbatim); AGENTS.md section + constraint in English | Verbatim import settles doc language; host-file convention settles the binding text | 2026-07-04 |
| **D7 — Refresh trigger = the intake funnel** (new external version → `intake/` → triage → iteration), declared in the provenance lines | Answers persona rule C2's "when does this lie?" with the repo's existing mechanism — no new machinery | 2026-07-04 |
| **D8 — Ex-post hook = light** (`intake/Discovery/` draft item; revisit after ~5 iterations with §8 signals) + efficacy declared INCONCLUSIVO | Persona §8: it is an ex ante hypothesis until naturalistic evaluation; declare-only leaves a silent permanent INCONCLUSIVO; a formal metrics scaffold fails the persona's own fragment test (D1/C1) | 2026-07-04 |
| **D9 — Iteration identity**: `022-sdd-persona-adoption`, type `feature` | Developer chose feature framing (new methodology capability for this repo's agents) | 2026-07-04 |

## Blockers & Notes

- The persona's efficacy, by its own ruler (§8), is **INCONCLUSIVO — ex ante hypothesis with cited
  grounding, awaiting naturalistic evaluation**. The ex-post hook in `intake/Discovery/` is the
  resolution path.
- The self-only `AGENTS.md` section is an intentional, declared divergence from
  `templates/project/agents-md.md`. Future `update-project` refreshes (template → copy) must
  preserve it — the section says so itself.
- §6-checklist refutation, accepted as declared risk: the ambient binding is Guide-level — the
  AGENTS.md section points to `persona.md` but cannot force the read. Non-compliance would surface
  as missing §6 log entries in future iterations; the ex-post hook (`intake/Discovery/`) watches
  exactly this signal.

## References

- **Specification:** Decision log D0–D9 (above) + plan file of session 2026-07-04
- **PR/MR:** —
- **Commits:** —
- **Related Issues:** O-009 (`.stateful-spec/backlog.md`); source intake item `intake/Backlog/prd.md`
