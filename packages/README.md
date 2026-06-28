# packages/ — optional reference implementations

This directory holds **optional, separately-published** reference implementations of the Stateful
Spec multi-agent flow's *tooling contract* (defined in
[`../methodology/multi-agent-flow.md`](../methodology/multi-agent-flow.md)).

| Package | Language | Registry | Source |
|---------|----------|----------|--------|
| `stateful-spec-flow` | Rust (zero dependencies) | crates.io | [`flow-rs/`](flow-rs/) |
| `@stateful-spec/flow` | Node / TypeScript (zero runtime deps) | npm | [`flow-ts/`](flow-ts/) |

Both implement the **same** validated-verb CLI over `.stateful-spec/flow-state.md` — identical verbs,
state-file contract, and exit codes. Parity is enforced by the shared fixtures in
[`conformance/`](conformance/), which both test suites run.

## Not required; ask permission before use

- The methodology is **fully usable without these packages.** The documented contract is sufficient —
  any tool, or a human relaying turns, satisfies it.
- The **published packages** (from crates.io / npm), not the source here, are what a project installs.
  The sources merely live in this repository; they are not needed to use the methodology.
- An AI agent guided by the methodology must **notify the user and ask permission before using** a
  published flow tool (per [`../methodology/roles.md`](../methodology/roles.md)).

## Build & test

- Rust: `cargo test --manifest-path flow-rs/Cargo.toml`
- Node: `cd flow-ts && npm install && npm test`
- Conformance (Rust ≡ TS parity) fixtures: [`conformance/README.md`](conformance/README.md).

These are the **only** parts of this otherwise documentation-only repository that carry build/test
tooling — the sanctioned `packages/` exception noted in `CLAUDE.md` / `AGENTS.md`.
