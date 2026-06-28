# Iteration: 012 — Reverse update from stand-in

> Umbrella iteration. One file per feature, bugfix, or refactor. Track progress and decisions here.

## Metadata

- **Type:** feature
- **Status:** done
- **Created:** 2026-06-24
- **Completed:** 2026-06-28
- **Author:** Francisco Tarcizo Bomfim Júnior

## Description

Reverse-update (back-port) the methodology evolutions that grew **inside the downstream consumer
repo** `D:\development\public\stand-in` back into this agnostic methodology source. The work is
**curation, not copy**: each evolved/new artifact was compared one-by-one; only the open, general,
foundational essence is back-ported, and all stand-in-specific content (Rust/PowerShell/Windows,
GPUI/MCP, `stand-in-*` names, PT-BR state names, app examples, internal IDs) is stripped. The
documentation-only constraint is honored — no code/scripts are ported; protocols are described as
abstract contracts.

Driven by the plan `prompt.md` (root) → curated plan. Executed in **4 waves** with an adversarial
self-review after each wave and a final holistic review.

## Acceptance Criteria

- [x] **Wave 1** — `methodology/{history-archiving,backlog,qa-phase}.md` created (agnostic, EN, Per-Type Meaning on the universal ones)
- [x] **Wave 2** — `overview.md`, `roles.md`, `phases/05-verify.md` integrated with the new concepts (5-phase identity preserved)
- [x] **Wave 3** — session operations (start/resume/end/save-session) + `review-changes` updated (source + 3 ports each), governance/memory templates, init prompts, and companion templates (intake/, intake-item, backlog, qa-log)
- [x] **Wave 4** — `multi-agent-flow.md` rewritten as a single two-mode doc (no v1/v2 naming); flow-state v2 fields; arch-verdict template; `start-multi-agent-flow` ports
- [x] No stand-in-specific leakage in any new/edited upstream file (grep clean)
- [x] Source↔ports parity for every edited operation; `.mdc` frontmatter preserved
- [x] Relative links in new/edited methodology docs resolve
- [x] Adversarial self-review performed after each wave + final holistic review
- [x] NOT committed (changes left visible in the working tree)

## Implementation Tasks

- [x] Wave 1: write 3 new methodology docs
- [x] Wave 1: adversarial self-review
- [x] Wave 2: edit overview / roles / 05-verify
- [x] Wave 2: adversarial self-review
- [x] Wave 3: operations + ports + governance/memory templates + init + companion templates
- [x] Wave 3: adversarial self-review
- [x] Wave 4: multi-agent-flow two-mode rewrite + flow templates + ports
- [x] Wave 4: adversarial self-review
- [x] Final holistic self-review + verification greps
- [x] CHANGELOG + memory.md (Engramas / Recent Completions) + close iteration

## Quality Checks

> This repo is documentation-only — quality gate is manual review.

- [x] No stand-in specifics leaked (grep: stand-in, cargo, clippy, gpui, powershell, .exe, .ps1, mcp, PT-BR states)
- [x] Source↔port parity for edited operations
- [x] Links resolve; methodology↔templates↔self-application consistent
- [x] 5-phase identity intact in overview.md
- [x] No debug/TODO/placeholder leftovers; English throughout

## Session Log

| Timestamp | Operation | Summary |
|-----------|-----------|---------|
| 2026-06-24 | start-session | Opened umbrella iteration 012 for the reverse-update back-port from stand-in. Plan curated (4 waves, doc-único two-mode flow, universal supporting concepts, documentation-only). |
| 2026-06-24 | wave-1 | Created `methodology/{history-archiving,backlog,qa-phase}.md` — curated agnostic/EN: backlog states PT→EN (new/triaged/promoted/discarded), generic filename examples, instance paths as prose (no `../history/` links since methodology/ is at repo root), single-doc flow refs (no v2). |
| 2026-06-24 | wave-1 self-review | Grep clean (no stand-in/Rust/PS/PT-state leakage). Sibling links resolve; no dangling v2 ref. Forward-refs to triage/archive wiring are intentional (canon precedes wiring) — Wave 3 must close them. `RAW_HISTORY=3` newly introduced → echo in overview (W2) + memory template (W3). No rework needed. |
| 2026-06-24 | wave-2 | Integrated into existing methodology: overview.md (+"Supporting Structures" table, +intake/backlog/.archived in both trees, +bounded-context note), roles.md (+agnostic rigorous-3-agent note + post-delivery-QA subsection — NOT the stand-in v2 dump), phases/05-verify.md (+post-delivery-QA exit). 5-phase identity preserved explicitly. |
| 2026-06-24 | wave-2 self-review | Grep clean (overview/roles); 05-verify `cargo` hits are pre-existing balanced multi-stack examples, not leakage. Links resolve (`../qa-phase.md` from phases/; siblings from methodology/). Forward-ref: roles.md names the 3-agent mode before multi-agent-flow.md documents it — resolved in Wave 4. No rework. |
| 2026-06-24 | wave-3 | Wired supporting structures into the operational surface: start/resume/end/save-session + review-changes (source + 3 ports each = 20 files) gained intake triage (kickoff/close), both-dirs next-NNN scan, idempotent archive step, `[INCIDENT]` failure sweep, Engrama fold-preservation. Added governance/memory template rules (agents-md rules 7-9 incl. Engramas; memory.md History Index/.archived + fold note). Wired intake/+backlog creation into new-project/onboard-existing init and update-project refresh. Created companion templates: `templates/intake/` (README + Backlog/Discovery/QA), `intake-item.md`, `backlog.md`, `qa-log.md`. Curated agnostic (PT state nova→new; dropped intake/README ref; bland examples). |
| 2026-06-24 | wave-3 self-review | Grep clean: zero stand-in/gpui/ps1/PT-state leakage in prompts/ + templates/ (only pre-existing multi-stack `cargo` example placeholders in review-handoff/test-plan/project-definition). Port parity verified: Incident=4, kickoff-triage=8, close-triage=4, failure-sweep-light=4, fold-preservation=14 (12 ops + agents-md + memory). Scenario: un-refreshed projects degrade gracefully (missing intake/.archived → no-op scans; archive creates .archived on first move); `methodology/X` path style matches existing op convention. No rework. |
| 2026-06-24 | wave-4 | Rewrote `methodology/multi-agent-flow.md` as a single TWO-MODE doc (two-agent light + three-agent rigorous: read-only architect design gate + profiles + hand-back + two bounded loops), with an abstract "tooling contract" (validated verbs / blocking poll / monotonic seq / atomic writes) — NO code shipped (documentation-only); three-gate review + review-is-discovery generalized. Stripped flow CLI/Rust/PowerShell/Windows, `stand-in-*` personas, GPUI/MCP, fe/lib→"profile", v2/"superseded"/decision-IDs/iteration-numbers. Updated `templates/implementation/flow-state.md` (3-agent fields + `seq`); created `arch-verdict.md`. Extended `start-multi-agent-flow` (source + 3 ports): architect role, blocking-poll standby, design-gate STEP 4 additions. |
| 2026-06-24 | wave-4 self-review | Grep clean: zero leakage in multi-agent-flow.md and all flow files; repo-wide unambiguous flow-leak terms hit ONLY `.stateful-spec/` self-application history (legit precedent refs), zero in product. Parity: design-gate additions=4 (source+3 ports). Wave-2 forward-ref (roles.md 3-agent mode) now RESOLVED. Scenario: two-agent path unchanged; three-agent path coherent; mode disambiguated by the "additions" framing. No rework. |
| 2026-06-24 | final-review | Holistic end-to-end review: repo-wide leakage sweep clean — all 11 broad-pattern hits are pre-existing legit content (Rust preset, multi-stack example placeholders, init detection signals) or `.stateful-spec/` self-app history; **zero in any back-ported file**. All new template targets exist; methodology cross-refs resolve; the 4 supporting structures interlock; 5-phase identity preserved. CHANGELOG `[Unreleased]` updated. Status → review; **NOT committed**. The 012 Engrama compaction and this repo's own self-adoption (scaffolding `intake/`, the history-archiving migration of 001–009) are **deferred to the true `end-session` close after developer review**, to avoid an unreviewed structural migration. |
| 2026-06-28 | review-changes | Independent developer-requested review (via resume-session) — **verified, not trusted**, the wave self-reviews. Stand-in leakage CLEAN across methodology/prompts/templates/ports (hits only in self-app history + balanced multi-stack examples); all relative links in new/edited methodology docs resolve; source↔port parity SATISFIED for the 5 session/review ops × 3 ports (kickoff/close triage, both-dirs next-NNN, idempotent archiving, `[INCIDENT]` capture+sweep, Engrama fold-preservation present in every port). No rework — stays parked in review, uncommitted. Side cleanup: removed stray root `package.json`/`package-lock.json` (`headroom-ai`; unrelated to 012/013). |
| 2026-06-28 | end-session | Session closed (full close, developer-approved). Status → **done**, Completed 2026-06-28. Finalized alongside 013 (the Open Session): both committed earlier as `b4bdd10`; Engrama compiled into `memory.md` (the N=10 bound folded 001+002 into the `0-archived` row, preserved verbatim to `history/.archived/memory.md`); history-archiving migration ran (`git mv` 001–010 centrals into `history/.archived/`, History Index `File` cells repointed); moved to Recent Completions. No `[INCIDENT]` entries to sweep. |

## Decisions Made

| Decision | Rationale | Date |
|----------|-----------|------|
| Scope = complete, in 4 waves | Cover the full back-port surface while keeping implementation reviewable per wave | 2026-06-24 |
| Multi-agent flow = single doc, two modes (no v1/v2 naming) | "v1/v2" + "superseded since iteration NNN" is stand-in historical baggage; a general methodology offers a light (2-agent) and a rigorous (3-agent) mode | 2026-06-24 |
| Supporting concepts (archiving/intake-backlog/QA) = universal across all project types | Disciplines are agnostic; only the meaning of opportunity/defect/deliver shifts (Per-Type Meaning). multi-agent-flow stays software-only | 2026-06-24 |
| Documentation-only: no code/scripts ported | Hard constraint of this repo; the flow's `flow` CLI/PowerShell are described as an abstract tooling contract instead | 2026-06-24 |
| English; backlog states new/triaged/promoted/discarded | Source is EN; strip PT-BR state names from stand-in | 2026-06-24 |
| Do not commit this session | Developer wants changes visible/uncommitted for review next day | 2026-06-24 |

## Blockers & Notes

- Sources to curate from live under `D:\development\public\stand-in\.stateful-spec\` (methodology copy, intake/, scripts/, evolved ports, AGENTS.md). Do **not** copy instance data (real backlog/history/flow-state).
- Adversarial self-review protocol is mandatory per wave (hypothetical usage scenario → usage implications) and once holistically at the end.

## References

- **Specification:** `prompt.md` (repo root) → plan `sprightly-painting-babbage.md`
- **PR/MR:** —
- **Commits:** — (intentionally uncommitted this session)
- **Related Issues:** —
