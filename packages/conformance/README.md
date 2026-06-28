# Conformance fixtures — Rust ≡ TS parity

These fixtures are the **single source of truth** that both reference implementations
([`flow-rs`](../flow-rs) and [`flow-ts`](../flow-ts)) must satisfy identically. Each `cases/*.txt`
file describes an initial `flow-state.md` and a sequence of verb invocations with the expected exit
code and resulting frontmatter after each step. Both test suites load the same files and assert the
same outcomes — that is what guarantees the two binaries behave the same.

A deliberately tiny, line-based format keeps both parsers dependency-free (the Rust crate stays
zero-dependency; the Node package stays zero-runtime-dependency):

```
# comment line
[initial]
key = value            # the starting flow-state frontmatter
...

[step]
run    = <verb> [args...]   # invoked as: flow <verb> [args] --flow-file <case>
exit   = <int>             # expected process exit code
expect = key=value key=value   # frontmatter fields to assert after this step
```

Reasons passed to `request-changes` / `request-spec-changes` / `hand-back` use single tokens (no
spaces) so the `run` line splits on whitespace.

Run them via each package's test suite:

- Rust: `cargo test --manifest-path packages/flow-rs/Cargo.toml` (the `conformance` integration test)
- Node: `npm test` in `packages/flow-ts` (the `conformance.test` file)
