# Iteration: 013 — Flow packages (`packages/`): Rust crate + Node/TS

> Umbrella iteration. Track progress and decisions here.

## Metadata

- **Type:** feature
- **Status:** review
- **Created:** 2026-06-26
- **Completed:** — (implementation complete; awaiting developer review)
- **Author:** Francisco Tarcizo Bomfim Júnior

## Description

Bring an **agnostic reference implementation** of the multi-agent flow's "tooling contract" (defined
in `methodology/multi-agent-flow.md`, iteration 012) into a new root `packages/` folder — a **Rust
crate** `stateful-spec-flow` (→ crates.io) and a **Node/TypeScript** twin `@stateful-spec/flow`
(→ npm), curated from the `stand-in` `flow` crate by stripping project-specific content.

**Non-binding by design:** the methodology stays fully usable **without** the packages; their sources
are **not required** (published to registries, merely living in the repo). But an agent guided by the
methodology must **know they exist** and **ask the user's permission before using** one. This requires
amending this repo's "documentation-only / no application code" constraint to carve out `packages/`.

Depends on iteration **012** (in review): the packages conform to 012's `flow-state.md` schema +
verb set. Same curation discipline as 012 (agnostic, adversarial self-review per wave + final).
Plan: `sprightly-painting-babbage.md`.

## Acceptance Criteria

- [x] **Wave A** — `packages/flow-rs/` (`stateful-spec-flow`, zero-dep, MIT): agnostic modules ported (frontmatter/transition/poll/io/git), config-driven `spawn` (no `stand-in-*` bundles), fmt-gate enforcement dropped (`--gate` evidence kept), `--ui-visual` removed, EN strings; 13 verbs + exit-codes preserved
- [x] **Wave B** — `packages/flow-ts/` (`@stateful-spec/flow`, Node 18+, bin `flow`): same contract, same CLI surface, same exit-codes
- [x] **Wave C** — shared conformance fixtures (Rust≡TS parity); methodology awareness + ask-permission wiring (multi-agent-flow.md, start-multi-agent-flow + 3 ports, roles.md, overview.md, README.md); documentation-only amendment (CLAUDE.md, AGENTS.md, project-definition.md); packages/README.md; .gitignore
- [x] No stand-in leakage in the packages (grep clean: stand-in/opencode/cargo-fmt/gpui/mcp/fe/lib hardcoded/PT)
- [x] Rust ≡ TS parity verified via shared fixtures; both test suites green (where cargo/npm available)
- [x] Methodology usable without the packages; ask-permission present; documentation-only amendment consistent
- [x] Adversarial self-review after each wave + final holistic review
- [x] NOT committed (changes left visible in the working tree)

## Implementation Tasks

- [x] Setup: iteration 013 + memory.md
- [x] Wave A: Rust crate (port + curate)
- [x] Wave A: self-review + cargo build/test
- [x] Wave B: Node/TS twin
- [x] Wave B: self-review + npm build/test
- [x] Wave C: fixtures + methodology wiring + constraint amendment + packages/README + .gitignore
- [x] Wave C: self-review
- [x] Final holistic self-review + verification
- [x] CHANGELOG + memory.md + iteration → review

## Quality Checks

> Packages add code (the `packages/` exception); methodology core stays documentation-only.

- [x] No stand-in specifics leaked into the packages (config-driven, no hardcoded bundles)
- [x] Rust ≡ TS parity (shared conformance fixtures pass identically)
- [x] Source↔port parity for the edited `start-multi-agent-flow` operation
- [x] documentation-only amendment consistent across CLAUDE.md / AGENTS.md / project-definition.md
- [x] Methodology works without the packages; ask-permission rule present

## Session Log

| Timestamp | Operation | Summary |
|-----------|-----------|---------|
| 2026-06-26 | start-session | Opened umbrella iteration 013 for the `packages/` flow reference implementations (Rust + Node/TS). Plan approved (3 waves: Rust crate, TS twin, parity+methodology+constraints; core+configurable-spawn; stateful-spec-flow / @stateful-spec/flow; sources+tests, manual publish; no CI). Layers on 012 (in review). |
| 2026-06-26 | wave-A | Built `packages/flow-rs/` (`stateful-spec-flow`, zero-dep Rust, edition 2021 / MSRV 1.74). Ported frontmatter/transition(EN action strings)/poll near-verbatim; split util→`io.rs`+`git.rs` (configurable default branches); NEW `config.rs` (INI-lite parser); REWROTE `spawn.rs` config-driven (no Profile enum / SpawnMode bundles / opencode / cargo-fmt — project supplies `[spawn]` command templates with `{role}{profile}{spec}{handoff}{verdict_directive}`); `main.rs` dropped the fmt-gate + `--ui-visual`/`--fmt-manifest`, configurable branch guard, let-chains rewritten for MSRV. 13 verbs + exit-codes preserved. |
| 2026-06-26 | wave-A self-review | `cargo test` green: 54 unit + 7 integration. Leakage grep clean — only a generic `"fe"` example profile in tokenizer tests (no Fe/Lib enum, no stand-in bundles/opencode/gpui/mcp/cargo-fmt). State machine + frontmatter fields match the 012 `flow-state.md` contract. `spawn` degrades gracefully without `flow.conf` (clear error; every other verb still works). No rework. |
| 2026-06-26 | wave-B | Built `packages/flow-ts/` (`@stateful-spec/flow`, Node ≥18, TypeScript, zero runtime deps): reimplemented the identical contract module-by-module (frontmatter/transition[same EN action strings]/poll/io/git/config/spawn/cli) — same 13 verbs, exit codes, `flow-state.md` schema, atomic write, `seq`, turn-log format, and config-driven spawn. `tsc` clean; `node:test` suite: 4 unit + 7 CLI integration. |
| 2026-06-26 | wave-B self-review | `npm test` green (11). Zero leakage in TS src. Parity: the TS CLI integration asserts identical outcomes to the Rust integration (full cycle → seq=3, illegal → exit 1, missing `--total` → exit 2, gate evidence, design-gate cycle, poll `--once` → 5/0). Launch cross-platform (shell on Windows for `.cmd` shims); sync poll via `Atomics.wait`. Formal Rust≡TS parity via shared fixtures comes in Wave C. No rework. |
| 2026-06-26 | wave-C | Parity: `packages/conformance/cases/*.txt` (5 cases: two-agent cycle, three-agent design-gate, delivery-stall→BLOCKED, illegal-rejected, hand-back) + zero-dep runners in both packages (`tests/conformance.rs`, `test/conformance.test.ts`) — both pass identically → **Rust ≡ TS**. Methodology awareness + ask-permission wired: `multi-agent-flow.md` tooling-contract pointer (+softened "documentation-only"), `start-multi-agent-flow` + 3 ports (STEP 1 note), `roles.md` boundary, `README.md` "Optional Reference Implementations", root `AGENTS.md` note. documentation-only amended (`CLAUDE.md` ×3, `project-definition.md`) to carve out `packages/`; `templates/project/agents-md.md` commented note; `packages/README.md` + `packages/.gitignore`. |
| 2026-06-26 | wave-C self-review | ask-permission parity = 4 (source + 3 ports). `packages/` structure complete; all `packages/` link targets exist. grep CLEAN — no stand-in/gpui in any product file (the OpenCode refs are legitimate). documentation-only amendment consistent across CLAUDE/AGENTS/project-definition; methodology usable without packages; ask-permission rule anchored in `roles.md` and referenced from the flow doc, the op (+ports), AGENTS, README, packages/README. No rework. |
| 2026-06-26 | final-review | Both suites green together — Rust: 54 unit + 7 cli + 1 conformance (5 shared cases); Node: 16 (4 unit + 7 cli + 5 conformance). Holistic e2e: packages implement exactly the 012 `flow-state.md` schema + 13-verb contract; methodology fully usable WITHOUT the packages (contract suffices); an agent must ask permission to use one; documentation-only amendment coherent. Build artifacts (target/node_modules/dist) are `.gitignore`d. Status → review; **NOT committed**. Final Engrama compaction deferred to the true `end-session` close after developer review. |
| 2026-06-28 | review-changes | Independent developer-requested review (via resume-session). **Re-ran the gates from a clean slate**: `cargo test` → 54 unit + 7 integration + 1 conformance (0 failed); `npm ci && npm test` → 16 (4 unit + 7 CLI + 5 conformance, 0 failed); zero-dep confirmed; Rust≡TS parity genuine (both suites load the same 5 `conformance/cases/*.txt`). Leakage CLEAN (spawn config-driven; no bundles/fmt-gate); documentation-only carve-out consistent across CLAUDE/AGENTS/project-definition/README/agents-md; ask-permission wired in all 8 locations w/ 4-way parity. No rework — stays parked, uncommitted. Removed unrelated stray root `package.json`/`package-lock.json` (`headroom-ai`). |
| 2026-06-28 | write-commit-message | Generated the combined single-line commit message (279 chars, ASCII pt-BR per repo convention) for the whole 012+013 uncommitted delivery (the two iterations are intermingled across shared files — `start-multi-agent-flow` ports, CHANGELOG, README — so a clean split is impractical): `feat: back-port agnostico das evolucoes de metodologia do stand-in (012) + implementacao de referencia opcional do fluxo em packages/ (013) - novos docs history-archiving, backlog/intake e qa-phase; fluxo multi-agente em 2 modos + contrato de tooling; crate Rust + pacote Node/TS`. **Not committed** — message provided for the developer; both iterations stay parked. |

## Decisions Made

| Decision | Rationale | Date |
|----------|-----------|------|
| Names: `stateful-spec-flow` (crates.io) + `@stateful-spec/flow` (npm) | Match the methodology name; scoped npm under `stateful-spec` | 2026-06-26 |
| Core + configurable spawn (no hardcoded bundles) | Agnostic: package knows HOW to spawn; project supplies WHAT via a config command template | 2026-06-26 |
| Sources + tests; manual publish (no CI) | Keep the repo's no-CI character; CI is an explicit follow-up | 2026-06-26 |
| Packages are optional/non-binding; ask-permission before use | Methodology must stay usable without them; agent must know + ask | 2026-06-26 |
| Amend documentation-only constraint to carve out `packages/` | Packages are real code; the core stays documentation-only | 2026-06-26 |

## Blockers & Notes

- Source crate: `D:\development\public\stand-in\.stateful-spec\scripts\flow\` (zero-dep Rust, ~2200 LOC). Curate; do not copy stand-in specifics (spawn bundles, cargo-fmt gate, opencode, PT strings).
- Conform to 012's `flow-state.md` schema + 13-verb set. If 012 changes in review, reconcile.
- Do NOT commit; leave changes visible for developer review.

## References

- **Specification:** plan `sprightly-painting-babbage.md`; contract in `methodology/multi-agent-flow.md` (012)
- **Source crate:** `D:\development\public\stand-in\.stateful-spec\scripts\flow\`
- **PR/MR:** —
- **Commits:** — (intentionally uncommitted)
