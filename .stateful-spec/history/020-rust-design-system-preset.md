# Iteration: 020 — rust-design-system-preset

> One file per feature, bugfix, or refactor. Track progress and decisions here.

## Metadata

- **Type:** feature
- **Status:** done
- **Created:** 2026-06-28
- **Completed:** 2026-06-28
- **Author:** Francisco Tarcizo Bomfim Júnior

## Description

Add a toolkit-agnostic `rust-design-system` preset (`presets/rust-design-system.md`; Project Type: software, Category: design system) — a pre-filled Project Definition for a native Rust design-system crate, **distinct** from `rust-library` (logic crate) and from `rust-gpui-app` (composition layer). This is backlog opportunity **O-007**, the second of the two presets distilled from the iteration 018 `stand-in` analysis (O-006 `rust-gpui-app` shipped in iteration 019).

The preset captures a design-system crate's defining concerns: a single token canon (raw ramp → theme aliases → OKLCH semantic states, dark/light sharing one primary, runtime primary override), density + color-mode as typed enums driving five variables, embedded two-font typography, an icon-as-shape closed catalog, builder component contracts (stateless `Widget` vs `DsStatefulWidget<State>` + a shared `DsWidget` helper), one-file-per-component structure, centralized motion timings, a single `apply_theme` + context-stashed tokens accessor, the anti-facade clippy boundary with one sanctioned escape hatch, WCAG contrast + accessibility gates with shipped contrast math, per-component render-without-panic + differential geometry tests, `publish = false` path-dep packaging (with a documented publishable switch), a committed root `rust-toolchain.toml`, and an optional companion gallery/storybook.

**Authoring constraints:** mirror `templates/project/project-definition.md` as a terse preset (not an architecture essay); use bracketed `[e.g., egui | GPUI]` toolkit-agnostic placeholders; add "design system" as a free-text Category example; reuse the standard software operations (no DS-specific ops); add a presets-index row. Documentation-only — no application code, no build tooling.

## Acceptance Criteria

> Checkboxes for what "done" means. These come from the specification or user story.

- [x] `presets/rust-design-system.md` exists, mirrors the structure of `templates/project/project-definition.md`, and is terse (a filled Project Definition, not an architecture essay).
- [x] Project Type is `software`; Category is the free-text example "design system"; preset is distinct from both `rust-library` and `rust-gpui-app` (no duplication of their content; cross-reference where it clarifies the boundary).
- [x] Toolkit-agnostic: UI-toolkit specifics use bracketed `[e.g., egui | GPUI]` placeholders rather than hard-coding one framework.
- [x] Covers the design-system concerns from O-007: token canon, density/color-mode enums, typography, icon catalog, component contracts, file-per-component, motion timings, `apply_theme`/tokens accessor, anti-facade clippy boundary, accessibility/contrast gates, component tests, packaging (`publish = false` + publishable switch), `rust-toolchain.toml`, optional gallery.
- [x] Reuses the standard software operations (no DS-specific operations introduced).
- [x] A row for `rust-design-system` is added to the presets index, and CHANGELOG `[Unreleased]` records the addition (plus "design system" added as a Category example in the template).
- [x] Documentation-only: no application code, build tooling, dependencies, or CI introduced outside `packages/`.

## Implementation Tasks

> Breakdown of work. Check off as you go.

- [x] Read the O-007 source evidence: iteration 018/019 history, the external `stand-in` PRD lineage as captured in the backlog, and the existing `presets/rust-library.md` + `presets/rust-gpui-app.md` for parity and boundary. _(workflow Study phase)_
- [x] Confirm the canonical preset structure from `templates/project/project-definition.md` and the presets index location/format. _(workflow Study phase)_
- [x] Draft `presets/rust-design-system.md` directly under `presets/` (not in scratchpad — lesson from 018).
- [x] Generalize stand-in/MCP/toolkit-specific evidence to framework-neutral guidance with `[e.g., …]` placeholders.
- [x] Add the presets-index row and "design system" Category example where the registry documents Categories.
- [x] Update CHANGELOG `[Unreleased]`.
- [x] Self-review (adversarial): refute the preset against a hypothetical "I'm starting a Rust design-system crate" usage scenario; check for leakage, bloat, and `rust-library`/`rust-gpui-app` overlap. _(5-lens review + holistic read)_

## Quality Checks

> Standard checks from the Project Definition. Verify before marking done.

- [x] All quality gates pass (manual review — this repo is documentation-only Markdown)
- [x] Preset covers the acceptance criteria
- [x] Documentation updated (presets index + CHANGELOG + template Category example)
- [x] No debug code or TODOs left behind

## Session Log

> Timestamped entries recording each operation performed during this session.
> Agents append entries automatically when an Open Session is active.

| Timestamp | Operation | Summary |
|-----------|-----------|---------|
| 2026-06-28 | start-session | Session opened for O-007 — author the `rust-design-system` preset. Iteration 020 created and marked as the open session. |
| 2026-06-28 | author-preset (workflow) | Authored `presets/rust-design-system.md` via a 5-phase ultracode workflow (Study → dual-Draft → Judge-merge → 5-lens adversarial Review → Revise; 13 agents, ~517k tok). Wrote the final file (UTF-8, no BOM), added the `presets/README.md` index row, the CHANGELOG `[Unreleased]` entry, and "design system" as a Category example in `templates/project/project-definition.md`. Holistic adversarial read passed. Not committed / no PR. |
| 2026-06-28 | review-changes (polish) | Applied two reviewer notes: aligned the UI-framework placeholder order to `[e.g., egui \| GPUI]` (matching the CHANGELOG, acceptance criteria, and O-007 wording); changed the `presets/README.md` Stack column to "Rust + design tokens + GPUI/egui" to lead with the DS substance over the interchangeable toolkit. Cosmetic; no separate commit. |
| 2026-06-28 | write-commit-message / commit | Created branch `feature/020-rust-design-system-preset` and committed all session changes as `3173517` (7 files, +332/−5). Single-line message ≤300 chars per convention. No PR opened. |
| 2026-06-28 | end-session | Session closed. O-007 delivered (`presets/rust-design-system.md` + presets index, CHANGELOG, and template Category example); all acceptance criteria met. Iteration → done; Engrama 020 compiled; iteration 017 central `git mv`'d to `.archived/`; O-007 promoted in `backlog.md`. Close committed on `chore/020-end-session`. |

> **Timestamp format:** `YYYY-MM-DD HH:MM` (local time). Example: `2026-05-03 14:30 | start-session | Session opened for feature work.`
>
> **Note:** Iterations created prior to the session management feature may lack this section. This is expected and does not require migration.

## Decisions Made

> Decisions made during this iteration. Include rationale.

| Decision | Rationale | Date |
|----------|-----------|------|
| Take O-007 (`rust-design-system`) this session. | O-006 shipped in 019; O-007 was explicitly left `new` per the "one preset per session" decision. It is the natural next slot. | 2026-06-28 |
| Author via a 5-phase multi-agent workflow (Study → dual-Draft → Judge → 5-lens adversarial Review → Revise) rather than a single-pass draft. | Ultracode mode; the O-007 spec is dense and the quality bar (terse-but-complete, toolkit-agnostic, zero overlap with the two siblings) benefits from independent drafts + adversarial verification. 22 findings surfaced; the major ones (apply_theme inputs, `raw::` escape-hatch consistency, headless render tests) were fixed in the Revise pass. | 2026-06-28 |
| Make `rust-design-system` inherit generic crate hygiene from `rust-library` and restate only design-system deltas. | Keeps the preset terse and sharply distinct from the siblings; avoids duplicating logic-crate hygiene (rust-library) and app composition (rust-gpui-app). | 2026-06-28 |
| Edit the template's Category line to add "design system" (not only set it in the preset). | O-007 asks to add "design system" as a free-text Category example "where the registry documents Categories" — that is `templates/project/project-definition.md:18`; the siblings carry no Category field. | 2026-06-28 |

## Blockers & Notes

> Anything that blocked progress or is worth remembering.

- Source of truth for the preset's substance is the O-007 backlog entry (distilled in 018 from the `stand-in` analysis). Mirror `presets/rust-gpui-app.md`'s tone/length; keep it toolkit-agnostic.

## References

- **Specification:** Backlog O-007 (`.stateful-spec/backlog.md`); lineage in `history/018-apply-preset-rust.md` and `history/019-rust-gpui-app-preset.md`.
- **PR/MR:** _(none yet)_
- **Commits:** `3173517` on branch `feature/020-rust-design-system-preset`
- **Related Issues:** O-007
