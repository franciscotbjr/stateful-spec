---
description: End the current session by summarizing work and closing the iteration
---

# End Session

> Use at the end of an implementation cycle. The AI analyzes the open session, summarizes all work, closes the iteration, and clears the Open Session flag.

## Instructions

You are helping the developer end a work session. Your job is to review everything that happened during the open session, summarize it, mark the iteration as complete, and clear the Open Session flag so the next session starts fresh.

**Your role:**
- Confirm the developer wants to end the session
- Analyze the open iteration file and any Session Log entries
- Summarize the work
- Close the iteration and clear the Open Session flag

> **Note:** If you want to save progress without closing the cycle, use `save-session` instead.

---

### STEP 1 — Check for Open Session

Read `.stateful-spec/memory.md`.

Look for the **Open Session** section. If it shows `_(none)_` or the section doesn't exist:

> "I don't see an open session. Would you like me to start one with `start-session`, or should I do a regular `save-session`?"

- If the developer says **start one**, switch to the `start-session.md` workflow.
- If the developer says **save-session**, switch to the `save-session.md` workflow.
- If neither, stop here.

### STEP 2 — Read the Open Iteration

Read the iteration file referenced in the Open Session section (e.g., `.stateful-spec/history/NNN-name.md`).

Gather:
- **Description** — What was being worked on
- **Session Log** — All timestamped entries from different operations
- **Implementation Tasks** — What was checked off
- **Decisions Made** — Any decisions recorded
- **Acceptance Criteria** — Were they all met?

### STEP 3 — Ask for Confirmation

Show the developer a summary:

> "Here's what I found in the open session `[iteration-file]`:"
> - **Started:** [created date]
> - **Operations logged:** [list of operations from Session Log]
> - **Tasks completed:** [summary of checked tasks]
> - **Acceptance criteria met:** [which ones are done? which are not?]
>
> "Would you like to end this session? (yes/no)"

If the developer says **no**, stop here. The session remains open.

### STEP 4 — Summarize and Close

1. **Update the iteration file:**
   - Add a final Session Log entry: `Now | end-session | Session closed. [Summary of all work done.]`
   - Mark **Status** as `done` (or `review` if not all criteria met)
   - Fill **Completed** date with today's date

2. **Finalize Engramas:**
   - Run the Engramas map-reduce compaction on the iteration file:
     - **Map:** Group Session Log entries in batches of 5, summarize each batch into 1-2 lines.
     - **Reduce:** Combine Description + batch summaries + Decisions Made + Blockers & Notes into `Summary` (1-2 sentences), `Key Decisions` (up to 3 bullets, `—` if none), `Learnings` (up to 3 bullets, `—` if none).
   - Update the corresponding row in the **Engramas** table in `memory.md`.
   - **Two-tier compaction:** Check whether the active engram count (rows with a numeric `#`, excluding the Archive row) exceeds the tier threshold N (default 10, configured in the comment above the Engramas table). If it does, merge the oldest active row into the **Archive** row (numbered `0-archived`, the last row in the table): synthesize a 2-3 sentence summary covering all archived iterations for `Summary`, and fold relevant key decisions and learnings into `Key Decisions` and `Learnings`. This keeps the Engramas table bounded at N+1 rows regardless of project age.

3. **Update `memory.md`:**
   - **Open Session** — Set back to `_(none)_`
   - **Active Work** — Move the completed item to **Recent Completions**
   - **History Index** — Update the iteration's status to `done`

### STEP 5 — Confirm Close

Tell the developer:

> "Session closed: `history/NNN-name.md`"
> - All operations from this session are summarized in the iteration file.
> - The next `start-session` will create a new, separate iteration.

### STEP 6 — Suggest Commit (Optional)

If there are uncommitted changes, ask:

> "Would you like me to commit these session updates? Suggested message:"
> ```
> chore: close session for [iteration name]
> ```

If yes, stage and commit.

> **Multi-agent flow variant (milestone session).** When ending a session inside a
> multi-agent flow (`start-multi-agent-flow` — software only), you are closing a **milestone
> session** under an umbrella iteration, not the iteration itself. Log the milestone's close
> in the umbrella iteration's Session Log and advance the flow per
> `methodology/multi-agent-flow.md`. **Do not** clear the Open Session or move the umbrella
> iteration to Recent Completions until the **final** milestone reaches `DONE` — only then
> close the umbrella iteration, set its History Index status to `done`, and compile its
> single Engrama row.

## Output

After completing the end flow:

1. **Iteration closed** — Status marked as done, all contributions summarized
2. **Open Session cleared** — memory.md ready for next session
3. **Recent Completions updated** — Work is visible in project history

## Next Steps

- Start the next session with `start-session` when beginning a new implementation cycle
