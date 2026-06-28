//! The legal state transitions of the multi-agent flow, as **pure** functions.
//!
//! Each verb validates its precondition against the current frontmatter and,
//! on success, mutates it to the next state and bumps the monotonic `seq`.
//! Illegal transitions are rejected — this is how the "wrong turn flag → deadlock"
//! class of bug becomes impossible by construction.
//!
//! No IO here (no clock, no git, no disk): everything is unit-testable.

use crate::frontmatter::FrontMatter;

/// A requested state transition.
pub enum Verb {
    /// PM handed off the approved spec → hand off to the engineer.
    SpecReady,
    /// PM approved the delivery.
    Approve,
    /// PM rejected the delivery (carries the reason); bumps `review_round`.
    RequestChanges(String),
    /// Any agent halts the flow for the human (carries the reason).
    Block(String),
    /// Engineer delivered (gate green) → hand off to the PM for review.
    Submit,
    /// Engineer committed the approved milestone → advance to the next (or DONE).
    Advance,
    /// PM approved the master plan → start the run (carries `total_milestones`).
    /// The only writer of `AWAITING_PLAN_APPROVAL → RUNNING`.
    ApprovePlan(u64),
    /// PM submits the milestone spec to the architect for design-gate review.
    SubmitSpec,
    /// Architect approves the spec at the design gate.
    ApproveSpec,
    /// Architect requests spec changes (carries the reason); bumps `spec_review_round`.
    RequestSpecChanges(String),
    /// Engineer hands the milestone back to the PM for re-scope (carries the reason).
    /// Non-counted: zeros `spec_review_round`, does not increment any round.
    HandBack(String),
}

/// What happened, for the turn-log line.
pub struct Outcome {
    pub role: &'static str,
    pub action: String,
}

/// Apply `verb` to `fm`, mutating it in place. Returns the turn-log outcome or a
/// human-readable precondition error.
pub fn apply(fm: &mut FrontMatter, verb: &Verb) -> Result<Outcome, String> {
    let status = fm.get("process_status").unwrap_or("").to_string();
    let turn = fm.get("turn").unwrap_or("").to_string();
    let step = fm.get("step").unwrap_or("").to_string();

    let outcome = match verb {
        Verb::SpecReady => {
            require_running(&status)?;
            require_turn(&turn, "pm")?;
            require_step(&step, &["SPEC_APPROVED"])?;
            fm.set("step", "SPEC_READY");
            fm.set("turn", "engineer");
            fm.set("updated_by", "pm");
            Outcome {
                role: "pm",
                action: "SPEC_APPROVED→SPEC_READY (handoff to engineer)".to_string(),
            }
        }
        Verb::SubmitSpec => {
            require_running(&status)?;
            require_turn(&turn, "pm")?;
            require_step(&step, &["SPEC_PENDING", "SPEC_CHANGES_REQUESTED"])?;
            fm.set("step", "ARCH_REVIEW");
            fm.set("turn", "architect");
            fm.set("updated_by", "pm");
            Outcome {
                role: "pm",
                action: format!("{step}→ARCH_REVIEW (spec submitted to reviewer)"),
            }
        }
        Verb::ApproveSpec => {
            require_running(&status)?;
            require_turn(&turn, "architect")?;
            require_step(&step, &["ARCH_REVIEW"])?;
            fm.set("step", "SPEC_APPROVED");
            fm.set("turn", "pm");
            fm.set("updated_by", "architect");
            Outcome {
                role: "architect",
                action: "ARCH_REVIEW→SPEC_APPROVED (design gate approved)".to_string(),
            }
        }
        Verb::RequestSpecChanges(reason) => {
            require_running(&status)?;
            require_turn(&turn, "architect")?;
            require_step(&step, &["ARCH_REVIEW"])?;
            let round = fm.get_u64("spec_review_round") + 1;
            let max = fm.get_u64("max_spec_review_rounds");
            fm.set("spec_review_round", &round.to_string());
            fm.set("step", "SPEC_CHANGES_REQUESTED");
            fm.set("turn", "pm");
            fm.set("updated_by", "architect");
            if max > 0 && round > max {
                fm.set("process_status", "BLOCKED");
                fm.set(
                    "blocked_reason",
                    &format!("stall: spec_review_round {round} > max {max} — {reason}"),
                );
                Outcome {
                    role: "architect",
                    action: format!(
                        "ARCH_REVIEW→SPEC_CHANGES_REQUESTED (spec round {round}) → BLOCKED (stall)"
                    ),
                }
            } else {
                Outcome {
                    role: "architect",
                    action: format!(
                        "ARCH_REVIEW→SPEC_CHANGES_REQUESTED (spec round {round}): {reason}"
                    ),
                }
            }
        }
        Verb::HandBack(reason) => {
            require_running(&status)?;
            require_turn(&turn, "engineer")?;
            require_step(&step, &["SPEC_READY", "CHANGES_REQUESTED"])?;
            fm.set("step", "SPEC_PENDING");
            fm.set("turn", "pm");
            fm.set("spec_review_round", "0");
            fm.set("updated_by", "engineer");
            Outcome {
                role: "engineer",
                action: format!("{step}→SPEC_PENDING (handed back to PM for re-scope): {reason}"),
            }
        }
        Verb::Approve => {
            require_running(&status)?;
            require_turn(&turn, "pm")?;
            require_step(&step, &["AWAITING_REVIEW"])?;
            fm.set("step", "APPROVED");
            fm.set("turn", "engineer");
            fm.set("updated_by", "pm");
            Outcome {
                role: "pm",
                action: "AWAITING_REVIEW→APPROVED".to_string(),
            }
        }
        Verb::RequestChanges(reason) => {
            require_running(&status)?;
            require_turn(&turn, "pm")?;
            require_step(&step, &["AWAITING_REVIEW"])?;
            let round = fm.get_u64("review_round") + 1;
            let max = fm.get_u64("max_review_rounds");
            fm.set("review_round", &round.to_string());
            fm.set("step", "CHANGES_REQUESTED");
            fm.set("turn", "engineer");
            fm.set("updated_by", "pm");
            if max > 0 && round > max {
                fm.set("process_status", "BLOCKED");
                fm.set(
                    "blocked_reason",
                    &format!("stall: review_round {round} > max {max} — {reason}"),
                );
                Outcome {
                    role: "pm",
                    action: format!(
                        "AWAITING_REVIEW→CHANGES_REQUESTED (round {round}) → BLOCKED (stall)"
                    ),
                }
            } else {
                Outcome {
                    role: "pm",
                    action: format!("AWAITING_REVIEW→CHANGES_REQUESTED (round {round}): {reason}"),
                }
            }
        }
        Verb::Block(reason) => {
            fm.set("process_status", "BLOCKED");
            fm.set("blocked_reason", reason);
            let role = if turn == "engineer" { "engineer" } else { "pm" };
            fm.set("updated_by", role);
            Outcome {
                role,
                action: format!("→BLOCKED: {reason}"),
            }
        }
        Verb::Submit => {
            require_running(&status)?;
            require_turn(&turn, "engineer")?;
            require_step(&step, &["SPEC_READY", "CHANGES_REQUESTED"])?;
            fm.set("step", "AWAITING_REVIEW");
            fm.set("turn", "pm");
            fm.set("updated_by", "engineer");
            Outcome {
                role: "engineer",
                action: format!("{step}→AWAITING_REVIEW (delivery; gate green)"),
            }
        }
        Verb::Advance => {
            require_running(&status)?;
            require_turn(&turn, "engineer")?;
            require_step(&step, &["APPROVED"])?;
            let cur = fm.get_u64("current_milestone");
            let total = fm.get_u64("total_milestones");
            fm.set("updated_by", "engineer");
            if total > 0 && cur >= total {
                fm.set("process_status", "DONE");
                fm.set("done", "true");
                fm.set("step", "COMMITTED");
                fm.set("turn", "pm");
                Outcome {
                    role: "engineer",
                    action: format!("APPROVED→COMMITTED (m{cur}) → DONE (last milestone)"),
                }
            } else {
                let next = cur + 1;
                fm.set("current_milestone", &next.to_string());
                fm.set("step", "SPEC_PENDING");
                fm.set("review_round", "0");
                fm.set("spec_review_round", "0");
                fm.set("turn", "pm");
                Outcome {
                    role: "engineer",
                    action: format!("APPROVED→COMMITTED (m{cur}) → m{next} SPEC_PENDING"),
                }
            }
        }
        Verb::ApprovePlan(total) => {
            // The one transition out of the planning gate. It does NOT require
            // RUNNING — it produces it — so it checks AWAITING_PLAN_APPROVAL itself.
            if status != "AWAITING_PLAN_APPROVAL" {
                return Err(format!(
                    "process_status must be AWAITING_PLAN_APPROVAL (found '{status}')"
                ));
            }
            require_turn(&turn, "pm")?;
            fm.set("process_status", "RUNNING");
            fm.set("total_milestones", &total.to_string());
            fm.set("current_milestone", "1");
            fm.set("step", "SPEC_PENDING");
            fm.set("review_round", "0");
            fm.set("spec_review_round", "0");
            fm.set("turn", "pm");
            fm.set("updated_by", "pm");
            Outcome {
                role: "pm",
                action: format!(
                    "AWAITING_PLAN_APPROVAL→RUNNING (plan approved; total={total}; m1 SPEC_PENDING)"
                ),
            }
        }
    };

    fm.bump_seq();
    Ok(outcome)
}

fn require_running(status: &str) -> Result<(), String> {
    if status == "RUNNING" {
        Ok(())
    } else {
        Err(format!("process_status must be RUNNING (found '{status}')"))
    }
}

fn require_turn(turn: &str, expected: &str) -> Result<(), String> {
    if turn == expected {
        Ok(())
    } else {
        Err(format!("not {expected}'s turn (turn='{turn}')"))
    }
}

fn require_step(step: &str, allowed: &[&str]) -> Result<(), String> {
    if allowed.contains(&step) {
        Ok(())
    } else {
        Err(format!("step must be one of {allowed:?} (found '{step}')"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn running(turn: &str, step: &str, milestone: u64, total: u64) -> FrontMatter {
        FrontMatter::from_pairs(&[
            ("process_status", "RUNNING"),
            ("total_milestones", &total.to_string()),
            ("current_milestone", &milestone.to_string()),
            ("turn", turn),
            ("step", step),
            ("review_round", "0"),
            ("max_review_rounds", "3"),
            ("spec_review_round", "0"),
            ("max_spec_review_rounds", "3"),
        ])
    }

    #[test]
    fn test_spec_ready_happy_path() {
        let mut fm = running("pm", "SPEC_APPROVED", 1, 5);
        apply(&mut fm, &Verb::SpecReady).unwrap();
        assert_eq!(fm.get("step"), Some("SPEC_READY"));
        assert_eq!(fm.get("turn"), Some("engineer"));
        assert_eq!(fm.get("seq"), Some("1"));
    }

    #[test]
    fn test_spec_ready_rejected_from_spec_pending() {
        let mut fm = running("pm", "SPEC_PENDING", 1, 5);
        assert!(apply(&mut fm, &Verb::SpecReady).is_err());
        assert_eq!(fm.get("step"), Some("SPEC_PENDING"));
        assert_eq!(fm.get("seq"), None);
    }

    #[test]
    fn test_engineer_cannot_set_spec_ready() {
        let mut fm = running("engineer", "APPROVED", 1, 5);
        assert!(apply(&mut fm, &Verb::SpecReady).is_err());
        assert_eq!(fm.get("step"), Some("APPROVED"));
        assert_eq!(fm.get("seq"), None);
    }

    #[test]
    fn test_submit_then_approve_then_advance() {
        let mut fm = running("engineer", "SPEC_READY", 1, 3);
        apply(&mut fm, &Verb::Submit).unwrap();
        assert_eq!(fm.get("step"), Some("AWAITING_REVIEW"));
        assert_eq!(fm.get("turn"), Some("pm"));

        apply(&mut fm, &Verb::Approve).unwrap();
        assert_eq!(fm.get("step"), Some("APPROVED"));
        assert_eq!(fm.get("turn"), Some("engineer"));

        apply(&mut fm, &Verb::Advance).unwrap();
        assert_eq!(fm.get("current_milestone"), Some("2"));
        assert_eq!(fm.get("step"), Some("SPEC_PENDING"));
        assert_eq!(fm.get("turn"), Some("pm"));
        assert_eq!(fm.get("seq"), Some("3"));
    }

    #[test]
    fn test_advance_on_last_milestone_is_done() {
        let mut fm = running("engineer", "APPROVED", 3, 3);
        apply(&mut fm, &Verb::Advance).unwrap();
        assert_eq!(fm.get("process_status"), Some("DONE"));
        assert_eq!(fm.get("done"), Some("true"));
        assert_eq!(fm.get("turn"), Some("pm"));
    }

    #[test]
    fn test_request_changes_increments_round_and_blocks_at_max() {
        let mut fm = running("pm", "AWAITING_REVIEW", 2, 5);
        fm.set("review_round", "3"); // already at max
        let out = apply(&mut fm, &Verb::RequestChanges("light background".to_string())).unwrap();
        assert_eq!(fm.get("review_round"), Some("4"));
        assert_eq!(fm.get("process_status"), Some("BLOCKED"));
        assert!(out.action.contains("BLOCKED"));
    }

    #[test]
    fn test_block_from_any_state() {
        let mut fm = running("engineer", "SPEC_READY", 1, 5);
        apply(&mut fm, &Verb::Block("publish = human gate".to_string())).unwrap();
        assert_eq!(fm.get("process_status"), Some("BLOCKED"));
        assert_eq!(fm.get("blocked_reason"), Some("publish = human gate"));
    }

    #[test]
    fn test_advance_rejected_when_not_approved() {
        let mut fm = running("engineer", "SPEC_READY", 1, 5);
        assert!(apply(&mut fm, &Verb::Advance).is_err());
    }

    #[test]
    fn test_approve_plan_starts_run() {
        let mut fm = FrontMatter::from_pairs(&[
            ("process_status", "AWAITING_PLAN_APPROVAL"),
            ("turn", "pm"),
            ("step", "SPEC_PENDING"),
            ("review_round", "0"),
            ("max_review_rounds", "3"),
        ]);
        apply(&mut fm, &Verb::ApprovePlan(7)).unwrap();
        assert_eq!(fm.get("process_status"), Some("RUNNING"));
        assert_eq!(fm.get("total_milestones"), Some("7"));
        assert_eq!(fm.get("current_milestone"), Some("1"));
        assert_eq!(fm.get("step"), Some("SPEC_PENDING"));
        assert_eq!(fm.get("turn"), Some("pm"));
        assert_eq!(fm.get("review_round"), Some("0"));
        assert_eq!(fm.get("spec_review_round"), Some("0"));
        assert_eq!(fm.get("seq"), Some("1"));
    }

    #[test]
    fn test_approve_plan_rejected_when_not_awaiting() {
        let mut fm = running("pm", "SPEC_PENDING", 1, 5);
        assert!(apply(&mut fm, &Verb::ApprovePlan(5)).is_err());
        assert_eq!(fm.get("seq"), None);
    }

    // --- Design-gate (three-agent) tests ---

    #[test]
    fn test_submit_spec_from_spec_pending() {
        let mut fm = running("pm", "SPEC_PENDING", 1, 5);
        let out = apply(&mut fm, &Verb::SubmitSpec).unwrap();
        assert_eq!(fm.get("step"), Some("ARCH_REVIEW"));
        assert_eq!(fm.get("turn"), Some("architect"));
        assert_eq!(fm.get("updated_by"), Some("pm"));
        assert!(out.action.contains("SPEC_PENDING→ARCH_REVIEW"));
    }

    #[test]
    fn test_submit_spec_from_spec_changes_requested() {
        let mut fm = running("pm", "SPEC_CHANGES_REQUESTED", 2, 5);
        let out = apply(&mut fm, &Verb::SubmitSpec).unwrap();
        assert_eq!(fm.get("step"), Some("ARCH_REVIEW"));
        assert_eq!(fm.get("turn"), Some("architect"));
        assert!(out.action.contains("SPEC_CHANGES_REQUESTED→ARCH_REVIEW"));
    }

    #[test]
    fn test_submit_spec_rejected_wrong_turn() {
        let mut fm = running("engineer", "SPEC_PENDING", 1, 5);
        assert!(apply(&mut fm, &Verb::SubmitSpec).is_err());
        assert_eq!(fm.get("step"), Some("SPEC_PENDING"));
        assert_eq!(fm.get("seq"), None);
    }

    #[test]
    fn test_approve_spec_happy_path() {
        let mut fm = running("architect", "ARCH_REVIEW", 1, 5);
        let out = apply(&mut fm, &Verb::ApproveSpec).unwrap();
        assert_eq!(fm.get("step"), Some("SPEC_APPROVED"));
        assert_eq!(fm.get("turn"), Some("pm"));
        assert_eq!(fm.get("updated_by"), Some("architect"));
        assert!(out.action.contains("ARCH_REVIEW→SPEC_APPROVED"));
    }

    #[test]
    fn test_request_spec_changes_increments_round() {
        let mut fm = running("architect", "ARCH_REVIEW", 1, 5);
        let out = apply(&mut fm, &Verb::RequestSpecChanges("scope tweak".to_string())).unwrap();
        assert_eq!(fm.get("spec_review_round"), Some("1"));
        assert_eq!(fm.get("step"), Some("SPEC_CHANGES_REQUESTED"));
        assert_eq!(fm.get("turn"), Some("pm"));
        assert!(out.action.contains("spec round 1"));
        assert!(fm.get("process_status") != Some("BLOCKED"));
    }

    #[test]
    fn test_request_spec_changes_blocks_at_max() {
        let mut fm = running("architect", "ARCH_REVIEW", 1, 5);
        fm.set("spec_review_round", "3"); // already at max
        let out = apply(&mut fm, &Verb::RequestSpecChanges("infeasible design".to_string())).unwrap();
        assert_eq!(fm.get("spec_review_round"), Some("4"));
        assert_eq!(fm.get("process_status"), Some("BLOCKED"));
        assert!(out.action.contains("BLOCKED"));
    }

    #[test]
    fn test_full_design_gate_loop() {
        let mut fm = running("pm", "SPEC_PENDING", 1, 5);
        apply(&mut fm, &Verb::SubmitSpec).unwrap();
        assert_eq!(fm.get("step"), Some("ARCH_REVIEW"));
        apply(&mut fm, &Verb::RequestSpecChanges("missing edge cases".to_string())).unwrap();
        assert_eq!(fm.get("step"), Some("SPEC_CHANGES_REQUESTED"));
        assert_eq!(fm.get("spec_review_round"), Some("1"));
        apply(&mut fm, &Verb::SubmitSpec).unwrap();
        assert_eq!(fm.get("step"), Some("ARCH_REVIEW"));
        apply(&mut fm, &Verb::ApproveSpec).unwrap();
        assert_eq!(fm.get("step"), Some("SPEC_APPROVED"));
        apply(&mut fm, &Verb::SpecReady).unwrap();
        assert_eq!(fm.get("step"), Some("SPEC_READY"));
        assert_eq!(fm.get("turn"), Some("engineer"));
    }

    #[test]
    fn test_hand_back_from_spec_ready() {
        let mut fm = running("engineer", "SPEC_READY", 2, 5);
        fm.set("spec_review_round", "2"); // seed >0 to assert zeroing
        let out = apply(&mut fm, &Verb::HandBack("wrong profile".to_string())).unwrap();
        assert_eq!(fm.get("step"), Some("SPEC_PENDING"));
        assert_eq!(fm.get("turn"), Some("pm"));
        assert_eq!(fm.get("spec_review_round"), Some("0"));
        assert_eq!(fm.get("review_round"), Some("0"));
        assert_eq!(fm.get("updated_by"), Some("engineer"));
        assert!(out.action.contains("SPEC_READY→SPEC_PENDING"));
        assert!(out.action.contains("wrong profile"));
    }

    #[test]
    fn test_hand_back_from_changes_requested() {
        let mut fm = running("engineer", "CHANGES_REQUESTED", 3, 5);
        fm.set("review_round", "1");
        fm.set("spec_review_round", "1");
        apply(&mut fm, &Verb::HandBack("re-scope needed".to_string())).unwrap();
        assert_eq!(fm.get("step"), Some("SPEC_PENDING"));
        assert_eq!(fm.get("turn"), Some("pm"));
        assert_eq!(fm.get("spec_review_round"), Some("0"));
        assert_eq!(fm.get("review_round"), Some("1")); // HandBack leaves delivery round alone
    }

    #[test]
    fn test_hand_back_rejected_wrong_turn() {
        let mut fm = running("pm", "SPEC_READY", 1, 5);
        assert!(apply(&mut fm, &Verb::HandBack("nope".to_string())).is_err());
        assert_eq!(fm.get("seq"), None);
    }

    #[test]
    fn test_advance_zeros_spec_review_round() {
        let mut fm = running("engineer", "APPROVED", 1, 3);
        fm.set("spec_review_round", "2");
        apply(&mut fm, &Verb::Advance).unwrap();
        assert_eq!(fm.get("current_milestone"), Some("2"));
        assert_eq!(fm.get("step"), Some("SPEC_PENDING"));
        assert_eq!(fm.get("review_round"), Some("0"));
        assert_eq!(fm.get("spec_review_round"), Some("0"));
    }

    #[test]
    fn test_architect_rejected_on_engineer_verb() {
        let mut fm = running("architect", "SPEC_READY", 1, 5);
        assert!(apply(&mut fm, &Verb::Submit).is_err());
        assert_eq!(fm.get("step"), Some("SPEC_READY"));
        assert_eq!(fm.get("seq"), None);
    }

    #[test]
    fn test_architect_rejected_on_spec_ready() {
        let mut fm = running("architect", "SPEC_APPROVED", 1, 5);
        assert!(apply(&mut fm, &Verb::SpecReady).is_err());
        assert_eq!(fm.get("step"), Some("SPEC_APPROVED"));
        assert_eq!(fm.get("seq"), None);
    }
}
