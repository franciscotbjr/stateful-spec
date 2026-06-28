//! Pure decision logic for the `poll` loop, extracted so it is unit-testable
//! without IO, sleeps, or process exit.

/// What a single poll observation means for the given role.
#[derive(Debug, PartialEq, Eq)]
pub enum PollDecision {
    /// It is the role's turn to act.
    Turn,
    /// The flow finished (terminal).
    Done,
    /// The flow is blocked for the human (carries the reason).
    Blocked(String),
    /// Still the other agent's turn (carries the current `turn`).
    Waiting(String),
}

/// Decide what a poll observation means. Pure: no IO. DONE/BLOCKED take
/// precedence over whose turn it is.
pub fn decide(status: &str, turn: &str, role: &str, blocked_reason: &str) -> PollDecision {
    if status == "DONE" {
        PollDecision::Done
    } else if status == "BLOCKED" {
        PollDecision::Blocked(blocked_reason.to_string())
    } else if turn == role {
        PollDecision::Turn
    } else {
        PollDecision::Waiting(turn.to_string())
    }
}

/// Stall watchdog for the blocking poll: while we wait for the other agent,
/// track the flow-state `seq`; if it does not change for `stall_after` seconds,
/// a warning is due (and again every further `stall_after` window). Pure and
/// tick-driven so it is unit-testable.
///
/// The watchdog only *detects* a stall; the recovery protocol stays manual —
/// run `status` first (a flipped turn means the other agent finished, not died),
/// and judge a worker's liveness by its activity (CPU delta / new files), never
/// by a wall-clock timestamp (process clocks may be local, not UTC).
pub struct StallWatch {
    last_seq: Option<String>,
    unchanged_for: u64,
    warned_at: u64,
}

impl StallWatch {
    pub fn new() -> Self {
        Self {
            last_seq: None,
            unchanged_for: 0,
            warned_at: 0,
        }
    }

    /// Observe one poll tick (`tick_secs` since the previous observation).
    /// Returns `Some(seconds_without_change)` when a stall warning is due.
    pub fn observe(&mut self, seq: &str, tick_secs: u64, stall_after: u64) -> Option<u64> {
        if stall_after == 0 {
            return None; // watchdog disabled
        }
        if self.last_seq.as_deref() != Some(seq) {
            self.last_seq = Some(seq.to_string());
            self.unchanged_for = 0;
            self.warned_at = 0;
            return None;
        }
        self.unchanged_for += tick_secs;
        if self.unchanged_for >= self.warned_at + stall_after {
            self.warned_at = self.unchanged_for;
            Some(self.unchanged_for)
        } else {
            None
        }
    }
}

impl Default for StallWatch {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_done_takes_precedence_over_turn() {
        assert_eq!(decide("DONE", "pm", "pm", ""), PollDecision::Done);
    }

    #[test]
    fn test_blocked_carries_reason() {
        assert_eq!(
            decide("BLOCKED", "pm", "pm", "stall"),
            PollDecision::Blocked("stall".to_string())
        );
    }

    #[test]
    fn test_my_turn() {
        assert_eq!(decide("RUNNING", "pm", "pm", ""), PollDecision::Turn);
    }

    #[test]
    fn test_waiting_for_other_role() {
        assert_eq!(
            decide("RUNNING", "engineer", "pm", ""),
            PollDecision::Waiting("engineer".to_string())
        );
    }

    // -- StallWatch --

    #[test]
    fn test_stall_watch_warns_after_threshold() {
        let mut w = StallWatch::new();
        assert_eq!(w.observe("42", 5, 10), None); // first sight of seq=42
        assert_eq!(w.observe("42", 5, 10), None); // 5s unchanged
        assert_eq!(w.observe("42", 5, 10), Some(10)); // 10s -> warn
    }

    #[test]
    fn test_stall_watch_resets_on_seq_change() {
        let mut w = StallWatch::new();
        assert_eq!(w.observe("42", 10, 10), None);
        assert_eq!(w.observe("42", 10, 10), Some(10));
        assert_eq!(w.observe("43", 10, 10), None); // progress -> reset
        assert_eq!(w.observe("43", 10, 10), Some(10)); // stalls again
    }

    #[test]
    fn test_stall_watch_rewarns_every_window() {
        let mut w = StallWatch::new();
        assert_eq!(w.observe("7", 0, 10), None);
        assert_eq!(w.observe("7", 10, 10), Some(10));
        assert_eq!(w.observe("7", 5, 10), None); // 15s, next warn at 20
        assert_eq!(w.observe("7", 5, 10), Some(20));
    }

    #[test]
    fn test_stall_watch_disabled_with_zero() {
        let mut w = StallWatch::new();
        assert_eq!(w.observe("1", 0, 0), None);
        assert_eq!(w.observe("1", 3600, 0), None);
    }
}
