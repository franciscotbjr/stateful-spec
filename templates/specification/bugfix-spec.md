# Bugfix Specification

> Use this template to analyze a bug, identify the root cause, and plan the fix.

---

## Metadata

- **Bug Title:** [Short descriptive title]
- **Severity:** Critical | High | Medium | Low
- **Status:** Investigating | Root Cause Found | Fix In Progress | Resolved
- **Reported By:** [Name, issue link, or channel]
- **Affected Version:** [e.g., v1.3.2, commit hash, branch]

## Bug Description

[What is happening that shouldn't be? Be specific — include error messages, unexpected behavior, incorrect output.]

## Steps to Reproduce

1. [Step 1]
2. [Step 2]
3. [Step 3]

**Expected Behavior:** [What should happen]

**Actual Behavior:** [What actually happens]

## Environment

- **OS / Platform:** [e.g., Ubuntu 22.04, Chrome 120, iOS 17]
- **Runtime Version:** [e.g., Node 20.x, Python 3.12, Rust 1.75]
- **Relevant Config:** [Any configuration that affects the behavior]

## Root Cause Analysis

### Investigation

[What did you check? What did you find? Walk through the debugging process.]

### Root Cause

[The actual cause of the bug. Be specific — reference files, functions, line numbers.]

```
[Code snippet showing the problematic code, if applicable]
```

### Why It Wasn't Caught

[Optional but valuable: Why didn't existing tests catch this?]

## Proposed Fix

### Approach

[Describe the fix approach in plain language.]

### Code Changes

| File | Change Description |
|------|-------------------|
| [file path] | [What changes and why] |

### Fix Code

```
[Show the corrected code, if it's concise enough]
```

## Test Scenarios

### Regression Test

> A test that would have caught this bug and prevents it from recurring.

| # | Scenario | Input | Expected Output |
|---|----------|-------|-----------------|
| 1 | [The exact scenario that triggered the bug] | [input] | [correct output] |

### Related Scenarios

| # | Scenario | Input | Expected Output |
|---|----------|-------|-----------------|
| 1 | [Similar edge case that might be affected] | [input] | [expected output] |

## Impact Assessment

- **Who is affected?** [e.g., All users, users with feature X, admin only]
- **What's the blast radius?** [e.g., Data corruption, UI glitch, crash]
- **Is there a workaround?** [Yes/No — describe if yes]
- **Does this need a hotfix?** [Yes/No — describe urgency]

## Verification

- [ ] Root cause confirmed
- [ ] Fix implemented
- [ ] Regression test added
- [ ] Related scenarios tested
- [ ] Quality gates pass
- [ ] Fix verified in the affected environment

## References

- [Link to issue, error log, monitoring dashboard, or related PR]
