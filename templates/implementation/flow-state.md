---
process_status: PLANNING        # PLANNING | AWAITING_PLAN_APPROVAL | RUNNING | BLOCKED | DONE
iteration: NNN-name             # the umbrella iteration this flow drives
total_milestones: 0             # set once the milestone plan is approved
current_milestone: 0            # 1-based; 0 until RUNNING
turn: pm                        # pm | engineer — whose turn it is right now
step: SPEC_PENDING              # SPEC_PENDING | SPEC_READY | AWAITING_REVIEW | CHANGES_REQUESTED | APPROVED | COMMITTED
review_round: 0                 # review cycles for the current milestone (stall guard)
max_review_rounds: 3            # cap before BLOCKED (configurable)
blocked_reason: null            # text when process_status = BLOCKED, else null
done: false
updated_by: pm                  # pm | engineer
updated_at: YYYY-MM-DDTHH:MM:SSZ
---

# Flow State — [Iteration NNN-name]

> Coordination signal for the two-agent flow (see `methodology/multi-agent-flow.md`).
> One active flow at a time. The frontmatter above is authoritative; this body is an
> **append-only turn-log**. Only the agent whose `turn` it is writes.

## Turn Log

| Timestamp | Role | Action | File |
|-----------|------|--------|------|
| [YYYY-MM-DD HH:MM] | pm | Kicked off flow; wrote master plan. | `history/NNN-name.md` |
