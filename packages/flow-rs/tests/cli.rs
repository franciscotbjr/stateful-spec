//! Integration tests — drive the compiled `flow` binary end-to-end against a
//! temp `flow-state.md`. The `advance` git gates are bypassed with `--allow-dirty`
//! so these run anywhere (no git repo / cargo workspace required).

use std::path::PathBuf;
use std::process::{Command, Output};

fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_flow")
}

/// Create an isolated temp `.stateful-spec/flow-state.md` with the given
/// frontmatter (each entry a `key: value` line) and return its path.
fn temp_state(name: &str, fields: &[(&str, &str)]) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("flow-it-{}-{name}", std::process::id()));
    let ss = dir.join(".stateful-spec");
    std::fs::create_dir_all(&ss).unwrap();
    let p = ss.join("flow-state.md");
    let mut fm = String::from("---\n");
    for (k, v) in fields {
        fm.push_str(&format!("{k}: {v}\n"));
    }
    fm.push_str("---\n\n# Turn-log\n");
    std::fs::write(&p, fm).unwrap();
    p
}

fn run(flow: &PathBuf, verb: &str, extra: &[&str]) -> Output {
    let mut c = Command::new(bin());
    c.arg(verb).arg("--flow-file").arg(flow);
    for a in extra {
        c.arg(a);
    }
    c.output().unwrap()
}

fn field(flow: &PathBuf, key: &str) -> String {
    let content = std::fs::read_to_string(flow).unwrap();
    let prefix = format!("{key}:");
    for line in content.lines() {
        let t = line.trim();
        if t.starts_with(&prefix) {
            return t[prefix.len()..].trim().to_string();
        }
    }
    String::new()
}

fn running(milestone: &str, total: &str, turn: &str, step: &str) -> Vec<(&'static str, String)> {
    vec![
        ("process_status", "RUNNING".into()),
        ("iteration", "013-demo".into()),
        ("total_milestones", total.into()),
        ("current_milestone", milestone.into()),
        ("turn", turn.into()),
        ("step", step.into()),
        ("review_round", "0".into()),
        ("max_review_rounds", "3".into()),
        ("spec_review_round", "0".into()),
        ("max_spec_review_rounds", "3".into()),
        ("seq", "0".into()),
    ]
}

fn as_pairs<'a>(v: &'a [(&'static str, String)]) -> Vec<(&'static str, &'a str)> {
    v.iter().map(|(k, val)| (*k, val.as_str())).collect()
}

#[test]
fn test_full_cycle_via_binary() {
    let f = running("1", "3", "engineer", "SPEC_READY");
    let flow = temp_state("full", &as_pairs(&f));

    let o = run(&flow, "submit", &[]);
    assert!(o.status.success());
    assert_eq!(field(&flow, "step"), "AWAITING_REVIEW");
    assert_eq!(field(&flow, "turn"), "pm");

    let o = run(&flow, "approve", &[]);
    assert!(o.status.success());
    assert_eq!(field(&flow, "step"), "APPROVED");

    let o = run(&flow, "advance", &["--allow-dirty"]);
    assert!(o.status.success());
    assert_eq!(field(&flow, "current_milestone"), "2");
    assert_eq!(field(&flow, "step"), "SPEC_PENDING");
    assert_eq!(field(&flow, "turn"), "pm");
    assert_eq!(field(&flow, "seq"), "3");
}

#[test]
fn test_illegal_transition_rejected_and_state_untouched() {
    // engineer cannot spec-ready; state untouched (no seq write).
    let f = running("1", "3", "engineer", "APPROVED");
    let flow = temp_state("illegal", &as_pairs(&f));
    let o = run(&flow, "spec-ready", &[]);
    assert_eq!(o.status.code(), Some(1));
    assert_eq!(field(&flow, "step"), "APPROVED");
    assert_eq!(field(&flow, "seq"), "0");
}

#[test]
fn test_approve_plan_starts_run_via_binary() {
    let flow = temp_state(
        "plan",
        &[
            ("process_status", "AWAITING_PLAN_APPROVAL"),
            ("turn", "pm"),
            ("step", "SPEC_PENDING"),
            ("seq", "0"),
        ],
    );
    // missing --total → usage error (exit 2), state untouched.
    let o = run(&flow, "approve-plan", &[]);
    assert_eq!(o.status.code(), Some(2));
    assert_eq!(field(&flow, "process_status"), "AWAITING_PLAN_APPROVAL");

    let o = run(&flow, "approve-plan", &["--total", "5"]);
    assert!(o.status.success());
    assert_eq!(field(&flow, "process_status"), "RUNNING");
    assert_eq!(field(&flow, "total_milestones"), "5");
    assert_eq!(field(&flow, "current_milestone"), "1");
}

#[test]
fn test_submit_records_gate_evidence() {
    let f = running("1", "3", "engineer", "SPEC_READY");
    let flow = temp_state("gate", &as_pairs(&f));
    let o = run(&flow, "submit", &["--gate", "fmt=0 test=0 build=0"]);
    assert!(o.status.success());
    let content = std::fs::read_to_string(&flow).unwrap();
    assert!(content.contains("gate=[fmt=0 test=0 build=0]"));
}

#[test]
fn test_full_design_gate_cycle_via_binary() {
    let f = running("1", "2", "pm", "SPEC_PENDING");
    let flow = temp_state("design", &as_pairs(&f));

    assert!(run(&flow, "submit-spec", &[]).status.success());
    assert_eq!(field(&flow, "step"), "ARCH_REVIEW");
    assert_eq!(field(&flow, "turn"), "architect");

    assert!(run(&flow, "request-spec-changes", &["needs edge cases"])
        .status
        .success());
    assert_eq!(field(&flow, "step"), "SPEC_CHANGES_REQUESTED");
    assert_eq!(field(&flow, "spec_review_round"), "1");

    assert!(run(&flow, "submit-spec", &[]).status.success());
    assert!(run(&flow, "approve-spec", &[]).status.success());
    assert_eq!(field(&flow, "step"), "SPEC_APPROVED");

    assert!(run(&flow, "spec-ready", &[]).status.success());
    assert_eq!(field(&flow, "step"), "SPEC_READY");
    assert_eq!(field(&flow, "turn"), "engineer");
}

#[test]
fn test_poll_once_branches() {
    // waiting (turn != role) → exit 5
    let f = running("1", "3", "engineer", "SPEC_READY");
    let flow = temp_state("poll", &as_pairs(&f));
    let o = run(&flow, "poll", &["--role", "pm", "--once"]);
    assert_eq!(o.status.code(), Some(5));
    // your turn → exit 0
    let o = run(&flow, "poll", &["--role", "engineer", "--once"]);
    assert_eq!(o.status.code(), Some(0));
}

#[test]
fn test_status_runs() {
    let f = running("2", "4", "pm", "AWAITING_REVIEW");
    let flow = temp_state("status", &as_pairs(&f));
    let o = run(&flow, "status", &[]);
    assert!(o.status.success());
    let out = String::from_utf8_lossy(&o.stdout);
    assert!(out.contains("status=RUNNING"));
    assert!(out.contains("step=AWAITING_REVIEW"));
    assert!(out.contains("m=2/4"));
}
