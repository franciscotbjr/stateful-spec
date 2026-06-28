// Pure decision logic for the `poll` loop, unit-testable without IO.

export type PollDecision =
  | { kind: "turn" }
  | { kind: "done" }
  | { kind: "blocked"; reason: string }
  | { kind: "waiting"; turn: string };

/** DONE/BLOCKED take precedence over whose turn it is. Pure. */
export function decide(
  status: string,
  turn: string,
  role: string,
  blockedReason: string,
): PollDecision {
  if (status === "DONE") return { kind: "done" };
  if (status === "BLOCKED") return { kind: "blocked", reason: blockedReason };
  if (turn === role) return { kind: "turn" };
  return { kind: "waiting", turn };
}

/** Stall watchdog: warns when `seq` is unchanged for `stallAfter` seconds while
 *  waiting (and again every further window). Pure and tick-driven. */
export class StallWatch {
  private lastSeq: string | null = null;
  private unchangedFor = 0;
  private warnedAt = 0;

  observe(seq: string, tickSecs: number, stallAfter: number): number | null {
    if (stallAfter === 0) return null;
    if (this.lastSeq !== seq) {
      this.lastSeq = seq;
      this.unchangedFor = 0;
      this.warnedAt = 0;
      return null;
    }
    this.unchangedFor += tickSecs;
    if (this.unchangedFor >= this.warnedAt + stallAfter) {
      this.warnedAt = this.unchangedFor;
      return this.unchangedFor;
    }
    return null;
  }
}
