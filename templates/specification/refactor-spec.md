# Refactoring Specification

> Use this template when restructuring code without changing its external behavior.

---

## Metadata

- **Refactoring Name:** [Short descriptive name, e.g., "Extract auth middleware", "Consolidate API clients"]
- **Status:** Draft | Ready | In Progress | Complete
- **Risk Level:** Low | Medium | High
- **Scope:** [e.g., Single file, module, cross-module, system-wide]

## Motivation

[Why is this refactoring needed? What problem does the current structure cause?]

### Current Pain Points

- [e.g., Duplicated logic across 5 files]
- [e.g., Module is too large to navigate (800+ lines)]
- [e.g., Pattern is inconsistent with the rest of the codebase]

## Current State

### Structure

```
[Show the current file/module structure affected by the refactoring]
```

### Problems

| Problem | Location | Impact |
|---------|----------|--------|
| [e.g., Code duplication] | [files] | [Maintenance burden] |
| [e.g., Tight coupling] | [files] | [Can't test in isolation] |

### Code Example (Before)

```
[Show a representative example of the current problematic code]
```

## Proposed State

### Structure

```
[Show the target file/module structure after refactoring]
```

### Code Example (After)

```
[Show what the same example looks like after refactoring]
```

## Migration Plan

> Step-by-step plan to move from current to proposed state.

### Step 1: [Description]

- Files affected: [list]
- Risk: Low/Medium/High
- Can be committed independently: Yes/No

### Step 2: [Description]

- Files affected: [list]
- Risk: Low/Medium/High
- Can be committed independently: Yes/No

[Continue for each step...]

## Behavioral Invariants

> These behaviors MUST NOT change as a result of the refactoring.

- [ ] [e.g., API response format remains identical]
- [ ] [e.g., All existing tests pass without modification]
- [ ] [e.g., Error messages remain the same]
- [ ] [e.g., Performance does not degrade]

## Test Strategy

### Existing Tests

| Test Suite | Expected Result |
|------------|----------------|
| [e.g., Unit tests for module X] | Must pass unchanged |
| [e.g., Integration tests] | Must pass unchanged |

### New Tests Needed

| Test | Purpose |
|------|---------|
| [e.g., Test new extracted module] | [Verify extracted logic works in isolation] |

### Verification Steps

1. Run all existing tests before starting → all pass
2. Make changes incrementally
3. Run all tests after each step → all pass
4. Run quality gates after completion → all pass

## Risks & Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| [e.g., Breaking import paths] | Medium | High | [Update all consumers, grep for old paths] |
| [e.g., Subtle behavior change] | Low | High | [Comprehensive test suite, careful review] |

## Files Changed

### Modified

| File | Change Description |
|------|-------------------|
| [file path] | [What changes] |

### Created

| File | Purpose |
|------|---------|
| [file path] | [What it contains] |

### Deleted

| File | Reason |
|------|--------|
| [file path] | [Why it's no longer needed] |

## Out of Scope

[What this refactoring explicitly does NOT change. Important to prevent scope creep.]

## References

- [Link to related ADR, tech debt issue, or discussion]
