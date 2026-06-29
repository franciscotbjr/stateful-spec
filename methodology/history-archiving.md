# History Archiving & Context Budget

**Goal:** Keep the per-session context the agent loads **bounded** as a project ages, without
losing recall quality. Two artifacts grow without limit if left alone — the `history/` folder
(one central file per iteration **plus** every milestone spec and review handoff) and the
`memory.md` prose. This doc defines how `history/` is pruned into an archive that stays indexed
(never orphaned) and never bulk-read.

> **This doc is the canon (the *procedure*).** In a project that adopts the methodology, the actual
> files live at `.stateful-spec/history/` (raw, recent) and `.stateful-spec/history/.archived/`
> (archived); the index that resolves every iteration lives in `.stateful-spec/memory.md` (the
> **History Index**). This file defines how those work — it is not itself a history.

## Two bounded tiers, two thresholds

A project's recall is kept bounded at two layers, each with a named threshold documented where it
is enforced:

| Tier | Artifact | Threshold | Bounds |
|------|----------|-----------|--------|
| **Compiled memory** | Engramas table in `memory.md` | `N = 10` (comment above the table) | the active rows; oldest folds into the `0-archived` row |
| **Raw history** | `history/` folder | `RAW_HISTORY = 3` | the central files kept on disk outside `.archived/` |

`RAW_HISTORY` is the number of distinct **closed** iterations whose central file stays in
`history/`. It is configurable; the default is **3**. Engramas (`N`) bound *what the agent reads
for context*; `RAW_HISTORY` bounds *what sits un-archived on disk*. They are independent knobs.

## Preserving folded Engramas (no data loss on fold)

The Engrama two-tier compaction is **lossy**: when the active row count exceeds `N`, the oldest row
folds into the `0-archived` synthesis row, which destroys that row's compiled Summary / Key Decisions
/ Learnings in place. To keep the roll non-destructive, **before** collapsing a row into
`0-archived`, append its **full content verbatim** to `history/.archived/memory.md`:

> ```markdown
> ## NNN — folded YYYY-MM-DD
>
> | # | Summary | Key Decisions | Learnings |
> |---|---------|---------------|-----------|
> | NNN | <verbatim Summary> | <verbatim Key Decisions> | <verbatim Learnings> |
> ```

- **Append-only**, newest at the bottom; one section per folded iteration.
- Lives in the cold store, so the **reading rule** applies: never bulk-read; open it (by iteration
  `#`) only when the `0-archived` synthesis is insufficient. Default context still comes from the
  active Engramas + `RAW_HISTORY` centrals.
- This is a `memory.md`-tier operation (independent of the `history/` folder `git mv`). It runs at
  the same triggers — `start-session` / `end-session` / `save-session`, wherever the two-tier
  compaction runs.
- **Timing skew (why the cold store can look "stale"):** this append fires at threshold `N`
  (engram fold), while the central-file `git mv` fires at `RAW_HISTORY` (see *Two bounded tiers*).
  Because the two are independent, an iteration's **central file** can sit in `history/.archived/`
  for up to `N − RAW_HISTORY` iterations **before** — or without yet — its **engram row** is folded
  into `history/.archived/memory.md`. Both cadences write into the same `history/.archived/`
  directory but on different schedules, so a recently-archived central file whose engram row is not
  yet in the cold store is **expected**, not a missed append. The cold store always holds exactly
  the rows that have already folded out of the active Engramas window.

## Central vs auxiliary files

- **Central file** — the one `NNN-name.md` that the **History Index `File` column names** for an
  iteration. This is the authoritative definition. Do **not** infer "central" from a filename
  regex — naming is not always a reliable signal. A file like `NNN-name-mK-spec.md` (a milestone
  spec), `handoff-NNN-name-mK-review.md` (a review handoff), or `NNN-qa-corrections.md` (a QA log)
  is an **auxiliary**, not a central, even though it begins with a number.
- **Auxiliary file** — everything else tied to an iteration: per-milestone specs
  (`NNN-name-mK-spec.md`), review handoffs (`handoff-NNN-…-review*.md`), QA-correction logs,
  turn-log archives, and any other supporting artifact.

## The archive operation (idempotent)

Every trigger below points to this **single** procedure (DRY). Running it twice is a no-op.

> Ensure `history/` contains only:
> **(a)** the central `NNN-name.md` of the last `RAW_HISTORY` (=3) distinct **closed** iterations,
> plus **(b)** the **open** iteration's central file and its **current-milestone** auxiliaries.
>
> `git mv` everything else into `history/.archived/` — old centrals, *all* auxiliaries of closed
> iterations (even auxiliaries of the 3 kept iterations), and the completed-milestone auxiliaries
> of the open umbrella. For **each** move, repoint that iteration's `File` cell in the memory.md
> **History Index** to `history/.archived/NNN-name.md`.
>
> Use `git mv` (never delete+recreate) so `git log --follow` preserves history and the move is
> reversible. `.archived/` is **committed**, not gitignored.

The open iteration's **current-milestone** auxiliaries are never touched — only milestones already
closed get their spec+handoffs archived (see the milestone trigger). This de-risks a later
milestone needing an earlier milestone's file: it is still reachable by its exact indexed path.

## Reading rule

- **Never list or bulk-read `history/.archived/`.** It is a cold store, not a browse target.
- A **targeted single-file read by the exact path from the History Index** is allowed when an
  archived detail is genuinely needed and the Engramas + raw history do not suffice.
- Default context still comes from the **Engramas** section and the `RAW_HISTORY` central files in
  `history/`.

## Next-`NNN` detection scans BOTH directories

Detecting the next iteration number must scan **both** `history/` and `history/.archived/` and take
the max. The agent skips `.archived/` for *reading*, but **not** for *number allocation* — this
guarantees a number is never reused even under non-default `RAW_HISTORY` or manual archival. Every
place that computes the next `NNN` (`start-session`, `resume-session`, `save-session`) follows this
rule.

## Triggers

The operation runs, idempotently, at these boundaries:

- **Iteration start** — `start-session` (after the new iteration is created and indexed); also
  `resume-session` when its direct-task entry creates a new iteration.
- **Iteration end** — `end-session` (after the iteration is closed and memory updated).
- **Milestone close** — inside a multi-agent flow, when a milestone session closes (the umbrella
  advances to the next milestone): archive the **completed** milestones' spec+handoffs, keeping
  only the **current** milestone's. See [`multi-agent-flow.md`](multi-agent-flow.md).

Because the operation is idempotent, overlapping triggers are safe — each run only moves what is
not already archived.

## One-time migration (when adopting this in an existing project)

A project that predates this doc archives its backlog of history in one pass:

- `git mv` all auxiliaries + every central older than the last `RAW_HISTORY` iterations into
  `history/.archived/`.
- Repoint each archived iteration's `File` cell in the History Index.
- Verify with `git log --follow <archived file>` that history is preserved and `git status` shows
  **renames** (R), not delete+add.
- Rewrite the unbounded `memory.md` prose sections (Last Updated / Active Work / Open Session) to
  pointer form via **verified fold-down** — confirm each cut fact already lives in an Engrama row
  or a `history/NNN-*.md` file before removing it; never blind-delete.

## Per-Type Meaning

This applies to **every** Project Type (see [`project-types.md`](project-types.md)). The archive
tier, the `RAW_HISTORY` knob, the reading rule, and scan-both-dirs are identical regardless of
whether iterations deliver code, skills, or studies.

## Completion Criteria (when adopting this in a project)

- [ ] `history/.archived/` exists and is committed (not gitignored).
- [ ] `history/` holds exactly the last `RAW_HISTORY` closed centrals + the open iteration's
      central (+ its current-milestone auxiliaries).
- [ ] Every History Index `File` cell resolves — archived rows → `history/.archived/…`, recent rows
      → `history/…`.
- [ ] `RAW_HISTORY` is documented beside the Engramas `N` and the archive procedure.
- [ ] Next-`NNN` detection in `start-session` / `resume-session` / `save-session` scans both dirs.
- [ ] Running the archive operation a second time is a no-op (idempotent).
- [ ] The two-tier Engrama fold appends the folded row's full content to
      `history/.archived/memory.md` before collapsing it into `0-archived` (no data loss on the roll).
