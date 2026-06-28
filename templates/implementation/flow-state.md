---
process_status: PLANNING        # PLANNING | AWAITING_PLAN_APPROVAL | RUNNING | BLOCKED | DONE
iteration: NNN-name             # the umbrella iteration this flow drives
total_milestones: 0             # set once the milestone plan is approved
current_milestone: 0            # 1-based; 0 until RUNNING
turn: pm                        # pm | engineer | architect (architect only in the three-agent flow)
step: SPEC_PENDING              # SPEC_PENDING | SPEC_READY | AWAITING_REVIEW | CHANGES_REQUESTED | APPROVED | COMMITTED
                                # three-agent flow adds: ARCH_REVIEW | SPEC_APPROVED | SPEC_CHANGES_REQUESTED
profile: null                   # three-agent flow: the current milestone's engineer profile (domain), else null
review_round: 0                 # delivery-review cycles for the current milestone (stall guard)
max_review_rounds: 3            # delivery-gate cap before BLOCKED (configurable)
spec_review_round: 0            # three-agent flow: design-gate cycles (reset per milestone)
max_spec_review_rounds: 3       # three-agent flow: design-gate cap
blocked_reason: null            # text when process_status = BLOCKED, else null
done: false
updated_by: pm                  # pm | engineer | architect
updated_at: YYYY-MM-DDTHH:MM:SSZ
seq: 0                          # monotonic; bumped on every transition (strictly chronological log)
---

# Flow State — [Iteration NNN-name]

> Coordination signal for the multi-agent flow — two-agent or three-agent (see `methodology/multi-agent-flow.md`).
> One active flow at a time. The frontmatter above is authoritative; this body is an
> **append-only turn-log**. Only the agent whose `turn` it is writes.

## Turn Log

| Timestamp | Role | Action | File |
|-----------|------|--------|------|
| [YYYY-MM-DD HH:MM] | pm | Kicked off flow; wrote master plan. | `history/NNN-name.md` |
