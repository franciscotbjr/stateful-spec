# Project Definition — Rust Library

> Pre-filled preset for a Rust library/crate (single crate or a Cargo workspace). Customize for your project; delete the optional blocks that do not apply (e.g. the proc-macro split if you ship no macros).

---

## Project Identity

- **Project Name:** [your-crate-name]
- **Description:** [One-sentence description]
- **Project Type:** library
- **Repository URL:** [https://github.com/user/repo]
- **License:** MIT OR Apache-2.0

## Technology Stack

### Language(s)

| Language | Version | Role |
|----------|---------|------|
| Rust | [edition, e.g., 2024] | Primary |

- **MSRV (rust-version):** [e.g., 1.96] — declare it in `Cargo.toml` (`rust-version`, or `[workspace.package].rust-version`) so contributors and CI know the supported language surface.
- **Toolchain pinning:** commit a `rust-toolchain.toml` at the **repo root** (where the gates run) pinning the channel and the components every gate needs — otherwise divergent local toolchains produce false-green `fmt`/`clippy`:

  ```toml
  [toolchain]
  channel = "[e.g., 1.96.0]"
  components = ["rustfmt", "clippy"]
  ```

### Framework(s)

| Framework | Version | Purpose |
|-----------|---------|---------|
| [e.g., tokio] | [e.g., 1.52.3] | [Async runtime — if async] |

### Key Dependencies

| Dependency | Version | Purpose |
|------------|---------|---------|
| [e.g., serde] | [e.g., 1.0.228] | [Serialization / deserialization] |
| [e.g., serde_json] | [e.g., 1.0.150] | [JSON support] |
| [e.g., thiserror] | [e.g., 2.0.18] | [Error derive macros] |
| [e.g., tracing] | [e.g., 0.1.44] | [Structured instrumentation] |
| [e.g., reqwest] | [e.g., 0.13.4] | [HTTP client — if needed] |
| [e.g., async-trait] | [e.g., 0.1.89] | [Async trait support — if needed] |

### Build System & Package Manager

- **Package Manager:** cargo
- **Build Tool:** cargo
- **Task Runner:** cargo (via Cargo.toml aliases / `just` / `make`, optional)

### Cargo / Workspace

> Optional — applies if the crate is part of a Cargo workspace; a single-crate project can ignore this.

- **Workspace metadata:** centralize shared fields (`version`, `edition`, `rust-version`, `license`, `repository`, `keywords`, `categories`) in `[workspace.package]`; members inherit via `field.workspace = true`, keeping only per-crate `description`/`readme` local — bumps then happen in one place and never drift.
- **Workspace dependencies:** pin every third-party dependency (and its features) once in `[workspace.dependencies]`; members use `dep.workspace = true`. List internal path deps with **both** `path` and `version` so the entry works for local builds and for `cargo publish` (which resolves by version).
- **Resolver:** set `resolver = "2"`.

## Repository Structure

Single crate:

```
project/
├── Cargo.toml
├── rust-toolchain.toml
├── src/
│   ├── lib.rs            # crate root: //! docs, prelude, curated re-exports, crate-level lints
│   ├── error.rs          # centralized Error enum + Result<T> alias
│   └── [module]/
│       ├── mod.rs        # facade: declares submodules, re-exports public items
│       └── [type].rs     # one public type per file (+ co-located #[cfg(test)] tests)
├── tests/                # integration tests (one file per external boundary)
├── examples/             # runnable usage examples
├── README.md
├── CHANGELOG.md
└── LICENSE
```

Optional — workspace for a library that **ships procedural macros** (proc-macros must be their own crate):

```
workspace/
├── Cargo.toml            # [workspace], [workspace.package], [workspace.dependencies]
├── rust-toolchain.toml
├── [crate]/              # runtime crate; re-exports the macro crate (pub use [crate]_macros::*;)
└── [crate]-macros/       # companion proc-macro crate ([lib] proc-macro = true)
```

> A runnable example/reference crate, if useful, is just another `publish = false` workspace member.

### Key Directories

| Directory | Purpose |
|-----------|---------|
| src/ | Library source code |
| tests/ | Integration tests |
| examples/ | Usage examples |
| [crate]-macros/ | Companion proc-macro crate (optional) |

## Code Conventions

### Naming

| Item | Convention | Example |
|------|-----------|---------|
| Files | snake_case (named after the public type they hold) | user_service.rs |
| Functions/Methods | snake_case | get_user_by_id |
| Types/Structs/Enums | PascalCase | UserService |
| Constants | SCREAMING_SNAKE_CASE | MAX_RETRIES |
| Modules | snake_case | http_client |
| Lifetimes | lowercase, short | 'a, 'de |

### Code Style

- **Formatter:** rustfmt (default settings)
- **Max Line Length:** 100 (rustfmt default)
- **Indentation:** 4 spaces
- **Import Order:** std → external crates → crate internal → super/self

#### Lints

- Crate root carries `#![deny(missing_docs)]` and `#![deny(rustdoc::broken_intra_doc_links)]` so undocumented public items and dead doc links fail the build.
- In a workspace, centralize lints in `[workspace.lints]` and opt each crate in with `[lints] workspace = true`.

### Patterns & Conventions

- **Module layout:** one public type per file, grouped into domain modules; each module exposes a `mod.rs` facade that declares the private submodules and re-exports the public items. Consumers import from the module path, not individual files.
- **Prelude & public surface:** expose a `pub mod prelude` re-exporting the small set of types/traits/macros a typical user needs, and re-export the primary types at the crate root. The prelude **is** the public API — every addition is a SemVer commitment.
- **Visibility:** start private; widen deliberately with `pub(crate)` / `pub(super)`; reserve `pub` for the deliberate public surface.
- **Error handling:** centralize one crate-wide `Error` enum in `src/error.rs` deriving `thiserror::Error`, plus `pub type Result<T> = std::result::Result<T, Error>;`, both re-exported from the prelude. Implement `From<ForeignError>` **manually** for external types you auto-convert through `?` — do **not** use `#[from]` for variants that would publicly embed a foreign error type. Mark public, likely-to-grow enums `#[non_exhaustive]`; because that blocks downstream construction, also ship public constructor functions for the variants consumers must build. (For RPC/protocol crates: keep "operation ran but failed" as data in `Ok(...)` distinct from "call could not be performed" as `Err`.)
- **Construction / API design:** required fields in the constructor; optional config via chainable `with_*(mut self, …) -> Self` builders that **never panic**. Accept `impl Into<String>` to cut caller `.to_string()` noise.
- **Concurrency:** in an async/multi-threaded library, public shareable types must be `Send + Sync` — treat non-`Send` as a compile-time design error. Reach for interior mutability (`Arc<RwLock<…>>` / `tokio::Mutex`) only where state is genuinely shared.
- **Robustness:** never panic on values from the network, the user, or a remote peer — convert fallible parses into typed `Err`. Reserve `unwrap()` for provably-infallible constants.
- **Logging:** instrument with `tracing` (never `println!`/`log`); a library must **not** depend on `tracing-subscriber` or configure output — the consuming application owns the subscriber.
- **Serde / wire fidelity (if the crate serializes an external format):** derive `Serialize, Deserialize` (+ `Debug, Clone, PartialEq`), map casing with `#[serde(rename/rename_all)]`, annotate every `Option<T>` with `#[serde(skip_serializing_if = "Option::is_none")]`, model closed sets as `#[serde(tag = "type")]` enums, and verify with round-trip tests.
- **Secrets:** never derive `Debug` on types holding credentials/tokens — hand-write a `Debug` that masks sensitive fields, proven by a test.
- **Identity:** derive self-reported name/version from `env!("CARGO_PKG_NAME")` / `env!("CARGO_PKG_VERSION")`, never hard-coded.

### Feature Flags

> Optional — applies if the crate has optional functionality.

| Feature | Default | Activates | Purpose |
|---------|---------|-----------|---------|
| [e.g., feature-a] | yes | — | [Minimal / most-common capability] |
| [e.g., feature-b] | no | `dep:[optional-crate]` | [Heavier opt-in capability] |

- Keep `default` minimal; make heavier capabilities **additive** opt-in features that activate optional dependencies via `dep:` syntax.
- Gate the corresponding modules, re-exports, examples (`required-features`), and integration-test files (`#![cfg(feature = "…")]`) on their features.
- The library and its tests must build and pass with **default features only**; quality gates additionally run `--all-features`.
- A disabled optional path should **fail closed** with an actionable error, never silently no-op.
- Set `[package.metadata.docs.rs] all-features = true` so docs.rs renders the full surface.

## Testing

### Strategy

- **Unit Tests:** co-located in `#[cfg(test)] mod tests { use super::*; }`; small in-module fakes exercise trait/dispatch logic; `#[tokio::test]` for async paths.
- **Integration Tests:** in `tests/`, one file per external boundary; feature-gate files needing optional capabilities with `#![cfg(feature = "…")]`. Drive the assembled public API as a downstream user would.
- **Test Framework:** cargo test [or `cargo nextest` in CI]
- **Mocking:** [e.g., in-module fakes implementing the public trait; an HTTP mock crate if needed]
- **Coverage Target:** No formal target; focus on behavior coverage.

### Conventions

- **Assert behavior, not existence** — assert the value, state change, or emitted event across happy/edge/error paths. `is_ok()` alone pins nothing.
- For serialized types, assert serde round-trips and wire shape explicitly.
- Add compile-time `assert_send_sync::<T>()` tests (`fn assert_send_sync<T: Send + Sync>() {}`) for every public shareable type, so accidental loss of `Send`/`Sync` is a compile error.

### Test Naming Convention

`test_{what_is_being_tested}_{scenario}` — e.g., `test_serialize_minimal`, `test_call_unknown_item_error`.

## Quality Gates

> The gate is the **floor**, not the bar — clippy-green code can still fail on idiomatic Rust, behavior-asserting tests, and duplication. Run on Linux/macOS/Windows. (In a multi-crate repo, run gates from the repo root, e.g. via `--manifest-path`, rather than `cd`-ing into a sub-workspace.)

```bash
# Formatter check
cargo fmt --all --check

# Linter — warnings are errors; --all-targets so test modules are linted too
cargo clippy --all-features --all-targets -- -D warnings

# Tests
cargo test --all-features

# Build
cargo build --all-features

# Doc build
cargo doc --all-features --no-deps
```

## Documentation

### Required Documentation Files

| File | Purpose |
|------|---------|
| README.md | Project overview, usage, feature flags |
| CHANGELOG.md | Version history (Keep a Changelog format + SemVer) |
| ARCHITECTURE.md | (Optional) Module map + key design decisions (Decision / Alternatives / Rationale) |
| LICENSE | License file |

### Documentation Style

- **Code Comments:** rustdoc (`///` public items, `//` internal, `//!` module headers).
- **Doc Examples:** runnable doctests — prefer `no_run` with hidden `# use …` setup; use `ignore` only when external infrastructure is required.
- **CHANGELOG:** Keep a Changelog format with a top `[Unreleased]` section and dated `## [x.y.z] — YYYY-MM-DD` headers; each entry a concrete user-facing change.

## Deployment / Distribution

### Publishing (crates.io)

- Carry the full crates.io/docs.rs metadata: `description`, `license`, `repository`, `keywords`, `categories`, `readme`.
- Use `exclude = [...]` to keep non-shipping files out of the published tarball (CI config, IDE dirs, internal docs).
- In a workspace, publish **dependency crates before dependents** (e.g. a proc-macro crate before the runtime crate) — `cargo publish` discards the local `path` and resolves by version. `cargo release --workspace` computes the order; mark internal/reference crates `publish = false`.
- Authenticate via the `CARGO_REGISTRY_TOKEN` env var (not interactive `cargo login`); treat the actual release as a **human-gated** action.
- Track `Cargo.lock` in git for binaries/apps; for pure libraries it is optional.

### CI/CD

- **CI/CD:** [e.g., GitHub Actions]
- **Build workflow:** a cheap gate job (`fmt --check` → `clippy --all-features --all-targets -D warnings` → `doc --no-deps`), then a cross-OS test/build matrix depending (`needs:`) on the gate. A pure library needs no binary-artifact workflow.
- **Publish workflow:** triggered only on version tags (`v*`), with publish depending on a security-audit job (e.g. `cargo audit`) and the cross-OS tests.
- **Branch Strategy:** main + feature branches.

## Constraints & Non-Negotiables

- No unsafe code without justification and documentation
- All public items must have rustdoc documentation (`#![deny(missing_docs)]`)
- All public shareable types must be Send + Sync (verified by a compile-time test)
- No `#[from]` on error variants that expose external types — use manual From impls
- Public, growable enums are `#[non_exhaustive]` and ship public constructors for variants downstream must build
- The library builds and passes tests with **default features only**; optional functionality lives behind feature flags
- The library never depends on `tracing-subscriber` — the application owns subscriber configuration
- Secrets are redacted in manual `Debug` impls; library code never panics on external/user-controlled input
