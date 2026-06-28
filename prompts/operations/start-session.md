---
description: Start a new session for tracking implementation work
---

# Start Session

> Use at the beginning of an implementation cycle. Creates an iteration file and marks it as the open session so all subsequent operations register their contributions under it.

## Instructions

You are helping the developer start a new work session. Your job is to create the tracking files and mark the session as open so that other agent instances — running different operations like spec writing, code review, or documentation — all register their work under the same iteration.

**Your role:**
- Check for any existing open session
- Triage the intake inbox (kickoff triage)
- Ask the developer what they're working on
- Create an iteration file and mark the session as open in `memory.md`

---

### STEP 1 — Check for Existing Open Session

Read `.stateful-spec/memory.md`.

If the file has an **Open Session** section that references an active iteration file (not `_(none)_`):

> "There is already an open session: `[iteration-file]`. Would you like me to close it and start a new one, or should I cancel?"

- If the developer says **close it**, switch to the `end-session.md` workflow first, then return to this step.
- If the developer says **cancel**, stop here.

### STEP 1.5 — Triage the Intake Inbox (kickoff triage)

Run the kickoff triage before STEP 2. (The close-side triage runs in `end-session`.)

> Two "backlog" paths: `.stateful-spec/intake/Backlog/` is the raw inbox (input);
> `.stateful-spec/backlog.md` is the triaged artifact (output). Scan the former; write the latter.

1. **Scan the raw inbox.** List `.stateful-spec/intake/{Backlog,Discovery,QA}/` for item files
   (ignore each folder's `README.md`).

2. **Apply the READY gate.** Read each item's frontmatter. Triage only `status: ready` items; list
   `draft` and `triaged` items as not eligible. An item with a missing or unrecognized `status` is
   treated as `draft` (not triaged) — warn so it gets a valid `status`. (READY convention:
   `methodology/backlog.md`.)

3. **Present the candidates.** Show the `ready` items before asking what to work on (STEP 2); a
   triaged opportunity may become this session's work. If none, say so and continue.

4. **Route each candidate**, per `methodology/backlog.md`:
   - **Promote** → add a row to `.stateful-spec/backlog.md` with the next `O-NNN` (sequential, never
     reused), status `new`, Origin, Description. Numbering belongs to `backlog.md`.
   - **Discard** → record the reason. A resolved `Discovery/` note may instead feed a decision in the
     relevant iteration / ADR.
   - **Mark the item triaged.** Set the intake item's `status: triaged` and add a `destination:` line
     (the `O-NNN` + slot, or the discard reason). The file stays; it is not re-triaged.

5. **No scope-creep.** A promoted `O-NNN` is born `new`; it does not change the work being started
   unless the developer picks it in STEP 2.

### STEP 2 — Ask What's Being Worked On

Ask:

> "What are you planning to work on? Give me a brief description so I can create the session."

The description should be concise — one or two sentences. Examples: "Add user authentication with JWT", "Fix the race condition in the order processing pipeline", "Refactor the payment module to use a strategy pattern".

### STEP 3 — Determine Iteration Number

Scan **both** `.stateful-spec/history/` and `.stateful-spec/history/.archived/` to find the highest existing number (`NNN`) — older iterations are archived, and an archived number must never be reused. The new iteration will be `NNN + 1`.

If no iteration files exist, start from `001`. See `methodology/history-archiving.md`.

### STEP 4 — Create Iteration File

Create `.stateful-spec/history/NNN-[kebab-name].md` using `templates/implementation/iteration.md`:

- **Type:** Ask the developer or infer from the description (feature / bugfix / refactor / chore)
- **Status:** `in-progress`
- **Created:** Today's date
- **Description:** The description from STEP 2
- **Acceptance Criteria:** Leave for the developer to fill (or create placeholder)

### STEP 5 — Update memory.md

Update `.stateful-spec/memory.md`:

1. **Open Session section** — Set it to point to the new iteration file:
   ```
   - Currently open: [NNN] [brief description] → `history/NNN-name.md`
   ```

2. **Active Work section** — Add the new work:
   ```
   - [ ] [Brief description] → `history/NNN-name.md`
   ```

3. **History Index** — Add a row for the new iteration with status `in-progress`.

4. **Engramas section** — Insert a new row at the top of the Engramas table (newest-first). Set `Summary` to `_In progress_` and both `Key Decisions` and `Learnings` to `—`. The row must match the History Index `#` so both tables stay linked. The engram content will be finalized by `save-session` or `end-session` via map-reduce.

   **Two-tier compaction:** If adding this row causes the active engram count to exceed the tier threshold N (default 10, configured in the comment above the Engramas table), merge the oldest active row into the **Archive** row (the last row in the table, numbered `0-archived`). **Before collapsing it, append that row's full content verbatim to `history/.archived/memory.md`** so the fold loses no detail (see `methodology/history-archiving.md`). Sum the merged iteration: update the Archive row's `Summary` to a 2-3 sentence synthesis covering all archived iterations, and fold relevant key decisions and learnings into `Key Decisions` and `Learnings`. This keeps the Engramas table bounded at N+1 rows regardless of project age.

### STEP 5.5 — Archive Old History

Run the idempotent archive operation in `methodology/history-archiving.md`: now that the new
iteration is created and indexed, ensure `history/` keeps only the central files of the last
`RAW_HISTORY` (=3) closed iterations plus this newly-opened iteration; `git mv` everything else
(old centrals + all auxiliaries) into `history/.archived/`, repointing each moved iteration's
`File` cell in the History Index. Running it when nothing needs moving is a no-op.

### STEP 6 — Confirm

Tell the developer:

> "Session started: `history/NNN-name.md`"
> "All operations you run in this session (spec writing, code review, documentation, etc.) will register their contributions under this iteration. Use `end-session` when the implementation cycle is complete."

> **Multi-agent flow variant (milestone session).** When a multi-agent flow is active
> (`start-multi-agent-flow` — software only), a session opens **per milestone** under an
> existing **umbrella iteration**. In that case do **not** create a new `NNN` iteration file
> or a competing History Index / Engramas row — instead, mark the milestone session open
> under the umbrella iteration and log a Session Log entry there. See
> `methodology/multi-agent-flow.md`.

## Output

After completing the start flow:

1. **Iteration file created** — Ready to track work
2. **Open Session marked** — memory.md points to the active iteration
3. **All future operations aware** — Agent instances will check for the open session and register contributions

## Next Steps

- Proceed with the appropriate methodology phase (Analyze → Plan → Specify → Implement → Verify)
- When the cycle is complete, use `end-session` to close the session
