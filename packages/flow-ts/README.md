# @stateful-spec/flow

A small **cross-platform, zero-runtime-dependency** Node/TypeScript CLI that is the single writer of
`.stateful-spec/flow-state.md` for the [Stateful Spec](https://github.com/franciscotbjr/stateful-spec)
**multi-agent flow**. It is the Node twin of the Rust crate
[`stateful-spec-flow`](https://crates.io/crates/stateful-spec-flow) — **identical verbs, state-file
contract, and exit codes** (parity is enforced by a shared conformance suite). Instead of agents
hand-editing YAML, they call **validated verbs**: each checks its precondition, mutates the state
atomically, bumps a monotonic `seq`, and appends a turn-log line. Illegal transitions are rejected
and leave the state untouched.

> **Optional.** The methodology works without this tool — the
> [tooling contract](https://github.com/franciscotbjr/stateful-spec/blob/main/methodology/multi-agent-flow.md)
> can be satisfied by any implementation or by relaying turns by hand. An agent guided by the
> methodology must **ask the user's permission before using it**.

## Install

```
npm install -g @stateful-spec/flow      # provides the `flow` command
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
uncommitted, and archives the completed milestone's auxiliaries via `git mv`. `--allow-dirty` bypasses.

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

Placeholders: `{role}` · `{profile}` · `{spec}` · `{handoff}` · `{verdict_directive}`. A token that
resolves to empty is dropped. With no template configured for a kind, `spawn` fails with guidance —
every other verb still works.

## Build & test

```
npm install
npm test        # tsc + node --test
```

Node ≥ 18. Zero runtime dependencies; built with TypeScript. MIT licensed.
