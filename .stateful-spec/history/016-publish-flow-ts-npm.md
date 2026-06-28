# Iteration: 016 — Publish flow-ts to npm

> One file per feature, bugfix, or refactor. Track progress and decisions here.

## Metadata

- **Type:** chore
- **Status:** done
- **Created:** 2026-06-28
- **Completed:** 2026-06-28
- **Author:** Francisco Tarcizo Bomfim Júnior

## Description

Publish the optional reference package `@stateful-spec/flow` (`packages/flow-ts/`) to npm — the
npm counterpart to iteration 015's crates.io release of the Rust twin `stateful-spec-flow`.
Starting point is updating `package.json` with the metadata npm requires for a publish
(description, license, repository, keywords, files, exports, etc.) and preparing the release.

Promoted from the intake inbox (`intake/Backlog/prd.md`, origin: idea) as opportunity **O-003**.
The intake item is titled "publish flow-rs to npm"; since `flow-rs` → crates.io is already done
(015), npm targets the Node/TS twin.

## PRD Properties (binding)

> From the intake PRD `…/new - 02 - publish flow-rs to npm/product requirements document.md`.
> Applied verbatim (as 015 did with edition 2024 / MSRV 1.96).

- **typescript (dev) version:** `6.0.3` → `devDependencies.typescript`
- **@types/node (dev) version:** `26.0.1` → `devDependencies.@types/node`
- **node-version (engine):** `">=24"` → `engines.node`
- **Publish token:** provided in the PRD frontmatter — **secret**; used only at `npm publish` time via env (`NODE_AUTH_TOKEN` / `npm config`), never committed to the repo, git history, or any file. Flag for rotation after use.

## Acceptance Criteria

> Checkboxes for what "done" means. Derived from the external PRD.

- [x] `packages/flow-ts/package.json`: `devDependencies.typescript` = `6.0.3`, `@types/node` = `26.0.1`, `engines.node` = `">=24"` (per PRD properties)
- [x] Publish metadata complete (description, license, repository, files, bin — verified; `repository.url` prefixed `git+`)
- [x] All tests pass (`npm test` in `packages/flow-ts/`) — 16 tests, 0 fail
- [x] `npm publish --dry-run` succeeds — 10 files, 12.4 kB, clean
- [x] Package published to npm — `@stateful-spec/flow` v0.1.0 live at https://www.npmjs.com/package/@stateful-spec/flow

## Implementation Tasks

> Breakdown of work. Check off as you go.

- [x] Update `package.json` per PRD: `typescript` → `6.0.3`, `@types/node` → `26.0.1`, `engines.node` → `">=24"`
- [x] Verify publish metadata (description, license, repository, files, bin) is complete
- [x] Install bumped devDependencies and run all tests (`npm test`) — 16 pass
- [x] Add `"types": ["node"]` to `tsconfig.json` — required by TS 6.0's changed default type inclusion
- [x] Run `npm publish --dry-run` — clean
- [x] Publish (`npm publish --access public`) — published with an automation token after creating the `@stateful-spec` org; token via env, never stored

## Quality Checks

> Standard checks from the Project Definition. Verify before marking done.

- [x] `npm test` passes in `packages/flow-ts/` — 16 tests, 0 fail
- [x] `npm publish --dry-run` clean
- [x] Documentation updated (README ships in the tarball; versions per PRD)
- [x] No debug code or TODOs left behind

## Session Log

> Timestamped entries recording each operation performed during this session.
> Agents append entries automatically when an Open Session is active.

| Timestamp | Operation | Summary |
|-----------|-----------|---------|
| 2026-06-28 | start-session | Session opened. Triaged intake `prd.md` → promoted as O-003; iteration 016 created for publishing `@stateful-spec/flow` (flow-ts) to npm. |
| 2026-06-28 | start-session | Loaded external PRD; recorded binding properties (typescript 6.0.3, @types/node 26.0.1, node >=24) and token-as-secret note in iteration. |
| 2026-06-28 | implement | Verified PRD versions are current locally (node v24.18.0; ts 6.0.3 & @types/node 26.0.1 are latest). Bumped `package.json` (3 props + `git+` repo url). TS 6.0 dropped auto type inclusion → added `"types": ["node"]` to tsconfig. `npm test` 16/16 green; `npm publish --dry-run` clean (10 files, 12.4 kB). Awaiting publish confirmation. |
| 2026-06-28 | implement | `npm publish --access public` attempted (token via env, not stored). Token authenticated but npm returned **403** — account requires 2FA/granular bypass-2FA token to publish. Not published. Blocked on OTP or a bypass-2FA token. |
| 2026-06-28 | implement | Re-published with an automation token (bypasses 2FA) — cleared the 403. New error **404 Scope not found**: the `@stateful-spec` npm scope/org does not exist. Blocked on creating the `stateful-spec` org on npm (or rescoping the package). Prep remains green. |
| 2026-06-28 | implement | Developer created the `@stateful-spec` org. `npm publish --access public` (automation token via env) **succeeded** — `@stateful-spec/flow@0.1.0` live at https://www.npmjs.com/package/@stateful-spec/flow (web view confirmed; registry read API lagged briefly). All acceptance criteria met. |
| 2026-06-28 | end-session | Session closed. Published `@stateful-spec/flow@0.1.0` to npm (O-003) per the external PRD: bumped typescript 6.0.3 / @types/node 26.0.1 / node >=24, added `"types": ["node"]` for TS 6.0, 16/16 tests + clean dry-run. Publish path crossed 403 (2FA → automation token) and 404 (created `@stateful-spec` org). All criteria met. Flagged rotation of the 4 chat-pasted tokens + the plaintext token in the PRD frontmatter. |

> **Timestamp format:** `YYYY-MM-DD HH:MM` (local time). Example: `2026-05-03 14:30 | start-session | Session opened for feature work.`
>
> **Note:** Iterations created prior to the session management feature may lack this section. This is expected and does not require migration.

## Decisions Made

> Decisions made during this iteration. Include rationale.

| Decision | Rationale | Date |
|----------|-----------|------|
| Promote `prd.md` to O-003 and take it as this session's work | Concrete, durable release opportunity for the existing `packages/flow-ts/` package; npm counterpart to O-002 | 2026-06-28 |
| Interpret "flow-rs to npm" as flow-ts → npm | `flow-rs` is the Rust crate (→ crates.io, done in 015); npm is the registry for the Node/TS twin `@stateful-spec/flow` | 2026-06-28 |
| Add `"types": ["node"]` to `tsconfig.json` | TypeScript 6.0 changed the default `types` behavior — `@types/node` is no longer auto-included, so `node:*` imports failed to compile. Minimal config adaptation required by the PRD-bound TS 6.0 bump. | 2026-06-28 |
| Prefix `repository.url` with `git+` | Clears npm's publish normalization warning so the published manifest is clean. | 2026-06-28 |

## Blockers & Notes

> Anything that blocked progress or is worth remembering.

- External PRD loaded — binding properties captured in **PRD Properties** above (typescript 6.0.3, @types/node 26.0.1, node >=24).
- **Security:** the PRD frontmatter stores an npm token in plaintext. Use it only via env (`NODE_AUTH_TOKEN`) at publish time; never write it into this repo, git, or any committed file. Recommend revoking/rotating it at https://www.npmjs.com/settings/~/tokens after publishing.
- Publishing to npm is an irreversible/outward-facing action — confirm with the developer before the actual `npm publish`.
- **Blocker (2026-06-28):** publish returned **403** — the PRD's classic token authenticates but the account enforces 2FA for publishing. Resolve via either an OTP (`npm publish --otp=<code>`) or a granular access token with "bypass 2FA" enabled. All prep (versions, tests, dry-run) is green; only the authenticated publish remains.
- **Blocker (2026-06-28), resolved 403 → new 404:** an automation token cleared the 2FA gate. Publish then failed **404 "Scope not found"** — the `@stateful-spec` org/scope does not exist on npm. Resolution: create the `stateful-spec` organization on npmjs.com (Free plan, public packages) with the token's account as a member, then re-run `npm publish --access public`. (Alternative: rescope the package to a personal scope / unscoped name.)
- **Resolved (2026-06-28):** both blockers cleared — automation token (bypass-2FA) + the new `@stateful-spec` org. `@stateful-spec/flow@0.1.0` published public.
- **Token hygiene:** four npm tokens were pasted into the chat during the 403/404 debugging, plus the original token sits in the external PRD frontmatter. Developer confirmed the tokens were removed/revoked. Remaining action: blank the `token:` line in the external PRD file if not already done. Lesson: never debug publish auth by pasting live tokens into chat — use a single env-stored automation token.
- npm package versions are effectively permanent (unpublish is restricted) — future changes require a version bump.

## References

- **Specification:** intake/Backlog/prd.md (external PRD)
- **PR/MR:** —
- **Commits:** —
- **Related Issues:** Backlog O-003; intake/Backlog/prd.md
- **Published:** https://www.npmjs.com/package/@stateful-spec/flow (v0.1.0)
