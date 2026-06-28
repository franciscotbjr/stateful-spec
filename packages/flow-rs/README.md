# stateful-spec-flow

A small, **cross-platform, zero-dependency** Rust CLI that is the single writer of
`.stateful-spec/flow-state.md` for the [Stateful Spec](https://github.com/franciscotbjr/stateful-spec)
**multi-agent flow**. Instead of agents hand-editing YAML, they call **validated verbs**: each checks
its precondition, mutates the state atomically, bumps a monotonic `seq`, and appends a turn-log line.
Illegal transitions are rejected and leave the state untouched.

> **Optional.** The methodology works without this tool — the
> [tooling contract](https://github.com/franciscotbjr/stateful-spec/blob/main/methodology/multi-agent-flow.md)
> can be satisfied by any implementation or by relaying turns by hand. An agent guided by the
> methodology must **ask the user's permission before using it**. A Node/TypeScript twin is published
> as [`@stateful-spec/flow`](https://www.npmjs.com/package/@stateful-spec/flow).

## Install

```
cargo install stateful-spec-flow      # installs the `flow` binary
```

## Verbs

| Group | Verbs |
|-------|-------|
| PM | `approve-plan --total <n>` · `spec-ready` · `approve` · `request-changes "<reason>"` · `block "<reason>"` · `submit-spec` (three-agent) |
| Architect | `approve-spec` · `request-spec-changes "<reason>"` (design gate) |
| Engineer | `submit [--gate "k=v …"]` · `advance [--allow-dirty]` · `hand-back "<reason>"` |
| Utility | `status` · `poll --role <pm\|engineer\|architect> [--interval N] [--timeout N] [--stall-after N] [--once]` · `spawn` |

Exit codes — transitions: `0` ok · `1` precondition/gate refused · `2` usage · `3` IO.
`poll`: `0` your turn · `1` DONE · `2` BLOCKED · `3` IO · `4` timeout · `5` waiting (`--once`).

`advance` refuses on a default branch and while product code (outside `.stateful-spec/`) is
uncommitted, and archives the completed milestone's auxiliaries via `git mv`. It is the only verb
that touches git; `--allow-dirty` bypasses the gate.

## `spawn` — configurable, tool-agnostic

`flow spawn` launches the next agent. The tool is agnostic: it resolves the spawn **kind** from the
flow `step` (`commit` | `arch-review` | `engineer`) and runs a **command template** the project
supplies in `.stateful-spec/flow.conf`. Nothing about any agent runner or persona is hardcoded.

```ini
# .stateful-spec/flow.conf
[spawn]
program     = my-agent-runner
arch_review = run "/my-architect {profile} {verdict_directive}" {spec}
engineer    = run "/my-engineer {profile}" {handoff}
commit      = run /my-commit

[git]
default_branches = main, master
```

Placeholders substituted per kind: `{role}` (`architect`/`engineer`), `{profile}` (from `--profile`),
`{spec}` (the spec draft, on `arch-review`), `{handoff}` (the review handoff, on a changes-requested
engineer turn), `{verdict_directive}` (a parseable-verdict instruction, on `arch-review`). A token
that resolves to empty is dropped. With no template configured for a kind, `spawn` fails with
guidance — every other verb still works.

## Build & test

```
cargo build
cargo test
```

Zero external dependencies (pure `std`); MSRV 1.74. MIT licensed.
