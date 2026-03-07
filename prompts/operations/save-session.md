# Prompt: Save Session

## Context

Use this prompt at the end of a work session to generate a structured summary that can be used to resume work in the next session. This replaces manual session notes.

## Prerequisites

- You've done work during this session that you want to preserve context for
- AI has context about the work done during this session

## Input

No additional input needed — the AI generates the summary from the current conversation context.

If the AI has lost context, provide a brief reminder of what was done.

## Prompt

<!-- Copy from here -->

Please generate a session summary so I can resume work next time. Review everything we did in this session and produce a structured summary with the following sections:

**1. Session Overview**
- Date: [today's date]
- Duration: [approximate]
- Phase: [which methodology phase we're in]

**2. Work Completed**
List everything that was accomplished in this session:
- Files created, modified, or deleted
- Features implemented
- Tests written
- Documentation updated
- Decisions made

**3. Current State**
Describe the exact state of the work:
- What is complete and working
- What is partially done
- What quality gates have been run (and their results)

**4. Decisions Made**
List any decisions made during this session with brief rationale:
- [Decision] — [Why]

**5. Open Issues**
List anything unresolved:
- Bugs found but not fixed
- Questions that need answers
- Blockers that need resolution

**6. Next Steps**
Ordered list of what to do next when resuming:
1. [First thing to do]
2. [Second thing to do]
3. ...

**7. Context for Next Session**
Any additional context the AI will need to pick up where we left off (key file references, important details that might not be obvious from the code alone).

Format the summary as a clean Markdown document that I can save to a file.

<!-- To here -->

## Expected Output

A complete Markdown session summary document ready to be saved.

## Next Steps

- Review the summary for accuracy
- Save it to a file in the project (e.g., `docs/sessions/session-YYYY-MM-DD.md` or wherever the project keeps session notes)
- Use `prompts/initialization/resume-session.md` with this summary in the next session
