//! `flow spawn` — launch the next agent for the multi-agent flow.
//!
//! The package is **agnostic**: it knows HOW to spawn (resolve the program, build
//! argv from a configured command template, launch cross-platform, record the
//! result), and the project supplies WHAT to spawn via `[spawn]` command
//! templates in its config (see `config.rs`). No agent tool or persona is
//! hardcoded.
//!
//! Platform note: on Windows the child is launched via PowerShell's
//! `Start-Process` (so an npm `.cmd` shim is not mangled by the post-CVE-2024-24576
//! batch-file argument escaping) and its real PID is captured; on Unix it is a
//! backgrounded child that inherits the terminal.

use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// The spawn kind for the current `step` — which command template applies.
/// `APPROVED` → `commit`; `ARCH_REVIEW` → `arch-review`; otherwise → `engineer`.
pub fn spawn_kind(step: &str) -> &'static str {
    match step {
        "APPROVED" => "commit",
        "ARCH_REVIEW" => "arch-review",
        _ => "engineer",
    }
}

/// The role token for the current `step`: `architect` at the design gate,
/// `engineer` otherwise. Substituted into `{role}`.
pub fn role_for(step: &str) -> &'static str {
    if step == "ARCH_REVIEW" {
        "architect"
    } else {
        "engineer"
    }
}

/// A generic, tool-independent directive a project can place at
/// `{verdict_directive}` in its arch-review template, so the design-gate verdict
/// is recoverable from the reviewer's final message even if it never runs a verb
/// (see `arch-verdict.md`).
pub const VERDICT_DIRECTIVE: &str =
    "End your final message with a fenced verdict block containing exactly: \
     verdict: APPROVED|CHANGES_REQUESTED / milestone: <k> / reason: <one line, only if CHANGES_REQUESTED>.";

/// Relative path of the review handoff to mention — only for a `CHANGES_REQUESTED`
/// turn (the engineer is born with the PM's feedback in context). Fills `{handoff}`.
/// The caller verifies the file exists before substituting.
pub fn handoff_mention(step: &str, iteration: &str, milestone: &str) -> Option<String> {
    if step != "CHANGES_REQUESTED" || iteration.is_empty() || milestone.is_empty() {
        return None;
    }
    Some(format!(
        ".stateful-spec/history/handoff-{iteration}-m{milestone}-review.md"
    ))
}

/// Relative path of the milestone spec draft — only for an `ARCH_REVIEW` turn
/// (the architect is born reading the spec under review). Fills `{spec}`.
pub fn spec_mention(step: &str, iteration: &str, milestone: &str) -> Option<String> {
    if step != "ARCH_REVIEW" || iteration.is_empty() || milestone.is_empty() {
        return None;
    }
    Some(format!(
        ".stateful-spec/history/{iteration}-m{milestone}-spec.md"
    ))
}

/// Placeholder values substituted into a spawn command template.
pub struct Subs<'a> {
    pub role: &'a str,
    pub profile: &'a str,
    pub spec: &'a str,
    pub handoff: &'a str,
    pub verdict_directive: &'a str,
}

/// Build the argv for a spawn from a command `template`: tokenize (quote-aware),
/// substitute placeholders per token, and drop any token that resolves to empty
/// (so an absent `{handoff}`/`{spec}` leaves no empty argument).
pub fn build_args(template: &str, subs: &Subs) -> Vec<String> {
    tokenize(template)
        .into_iter()
        .map(|tok| substitute(&tok, subs))
        .filter(|t| !t.is_empty())
        .collect()
}

fn substitute(tok: &str, s: &Subs) -> String {
    tok.replace("{role}", s.role)
        .replace("{profile}", s.profile)
        .replace("{spec}", s.spec)
        .replace("{handoff}", s.handoff)
        .replace("{verdict_directive}", s.verdict_directive)
}

/// Split a command string into argv, keeping double-quoted spans as single
/// tokens (the quotes are stripped). No escape handling — keep templates simple.
fn tokenize(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut in_quote = false;
    let mut has = false;
    for ch in s.chars() {
        match ch {
            '"' => {
                in_quote = !in_quote;
                has = true;
            }
            c if c.is_whitespace() && !in_quote => {
                if has {
                    out.push(std::mem::take(&mut cur));
                    has = false;
                }
            }
            c => {
                cur.push(c);
                has = true;
            }
        }
    }
    if has {
        out.push(cur);
    }
    out
}

/// Resolve the spawn program: a path (used as-is if it exists) or a name searched
/// on `PATH` (with platform extensions on Windows).
pub fn resolve_program(name: &str) -> Option<PathBuf> {
    let p = Path::new(name);
    if p.is_absolute() || p.components().count() > 1 {
        return if p.is_file() { Some(p.to_path_buf()) } else { None };
    }
    find_in_path(std::env::var_os("PATH"), &candidate_names(name))
}

fn candidate_names(name: &str) -> Vec<String> {
    #[cfg(windows)]
    {
        vec![
            name.to_string(),
            format!("{name}.cmd"),
            format!("{name}.exe"),
            format!("{name}.bat"),
        ]
    }
    #[cfg(not(windows))]
    {
        vec![name.to_string()]
    }
}

/// Search a `PATH`-style variable for the first existing file among `names`.
/// Takes the path value explicitly so it is testable without mutating the env.
fn find_in_path(path: Option<OsString>, names: &[String]) -> Option<PathBuf> {
    let path = path?;
    for dir in std::env::split_paths(&path) {
        for name in names {
            let candidate = dir.join(name);
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }
    None
}

/// Launch `prog` with `args` in `cwd`, returning the launched process's PID.
pub fn launch(prog: &Path, args: &[String], cwd: &Path) -> Result<u32, String> {
    #[cfg(windows)]
    {
        launch_windows(prog, args, cwd)
    }
    #[cfg(not(windows))]
    {
        launch_unix(prog, args, cwd)
    }
}

/// Windows launch — delegates to PowerShell's `Start-Process`. This avoids the
/// post-CVE-2024-24576 batch-file argument escaping that Rust's `Command` applies
/// to a `.cmd` program (which mangles a `run <message>` argument), and gives the
/// child its own window. `-PassThru` returns the Process; `.Id` is the real PID.
#[cfg(windows)]
fn launch_windows(prog: &Path, args: &[String], cwd: &Path) -> Result<u32, String> {
    let arg_list = args
        .iter()
        .map(|a| ps_single_quote(a))
        .collect::<Vec<_>>()
        .join(",");
    let script = format!(
        "$ErrorActionPreference='Stop'; \
         (Start-Process -FilePath {file} -ArgumentList {args} \
          -WorkingDirectory {cwd} -PassThru).Id",
        file = ps_single_quote(&prog.display().to_string()),
        args = arg_list,
        cwd = ps_single_quote(&cwd.display().to_string()),
    );
    let out = Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", &script])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| format!("failed to launch powershell: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "Start-Process failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    let stdout = String::from_utf8_lossy(&out.stdout);
    stdout
        .lines()
        .rev()
        .find_map(|l| l.trim().parse::<u32>().ok())
        .ok_or_else(|| {
            format!(
                "could not parse PID from Start-Process output: {:?}",
                stdout.trim()
            )
        })
}

/// Unix launch — a backgrounded child inheriting the terminal.
#[cfg(not(windows))]
fn launch_unix(prog: &Path, args: &[String], cwd: &Path) -> Result<u32, String> {
    Command::new(prog)
        .args(args)
        .current_dir(cwd)
        .stdin(Stdio::null())
        .spawn()
        .map(|child| child.id())
        .map_err(|e| e.to_string())
}

/// Quote `s` as a PowerShell single-quoted string literal (escape `'` as `''`).
#[cfg(windows)]
fn ps_single_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "''"))
}

/// Write the `spawn-result.txt` signal file (read by a standby heartbeat).
/// `spawned_at` (epoch ms) lets a watcher correlate the session of this spawn.
pub fn write_spawn_result(repo_root: &Path, exit_code: i32, pid: u32, kind: &str) {
    let dir = repo_root.join(".stateful-spec");
    let _ = std::fs::create_dir_all(&dir);
    let spawned_at = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    let content =
        format!("exit_code={exit_code}\npid={pid}\nkind={kind}\nspawned_at={spawned_at}\n");
    let _ = std::fs::write(dir.join("spawn-result.txt"), content);
}

/// Decision returned by `zombie_guard`: whether it is safe to spawn, or stale
/// agent processes must be killed first so exactly one runs after the spawn. The
/// enumeration of live processes happens in the runtime; this type only encodes
/// the decision rule in a unit-testable form.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[allow(dead_code)]
pub enum ZombieGuard {
    Ok,
    KillStaleThenSpawn { live: usize },
}

/// Pure decision: given the count of live agent processes, should the spawner
/// kill them before launching the new agent?
#[allow(dead_code)]
pub fn zombie_guard(live: usize) -> ZombieGuard {
    if live == 0 {
        ZombieGuard::Ok
    } else {
        ZombieGuard::KillStaleThenSpawn { live }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn subs<'a>() -> Subs<'a> {
        Subs {
            role: "engineer",
            profile: "",
            spec: "",
            handoff: "",
            verdict_directive: "",
        }
    }

    #[test]
    fn test_spawn_kind() {
        assert_eq!(spawn_kind("APPROVED"), "commit");
        assert_eq!(spawn_kind("ARCH_REVIEW"), "arch-review");
        assert_eq!(spawn_kind("SPEC_READY"), "engineer");
        assert_eq!(spawn_kind("CHANGES_REQUESTED"), "engineer");
    }

    #[test]
    fn test_role_for() {
        assert_eq!(role_for("ARCH_REVIEW"), "architect");
        assert_eq!(role_for("SPEC_READY"), "engineer");
        assert_eq!(role_for("APPROVED"), "engineer");
    }

    #[test]
    fn test_tokenize_quote_aware() {
        let t = tokenize("run \"/flow-engineer fe\" {handoff}");
        assert_eq!(t, vec!["run", "/flow-engineer fe", "{handoff}"]);
    }

    #[test]
    fn test_build_args_drops_empty_placeholders() {
        // engineer template with no handoff → the {handoff} token is dropped.
        let s = subs();
        let a = build_args("run \"/flow-engineer {profile}\" {handoff}", &s);
        assert_eq!(a, vec!["run", "/flow-engineer "]);
    }

    #[test]
    fn test_build_args_substitutes() {
        let s = Subs {
            role: "architect",
            profile: "fe",
            spec: ".stateful-spec/history/013-m3-spec.md",
            handoff: "",
            verdict_directive: "VD",
        };
        let a = build_args("run \"/arch {role} {profile} {verdict_directive}\" {spec}", &s);
        assert_eq!(
            a,
            vec![
                "run".to_string(),
                "/arch architect fe VD".to_string(),
                ".stateful-spec/history/013-m3-spec.md".to_string(),
            ]
        );
    }

    #[test]
    fn test_handoff_mention_only_on_changes_requested() {
        assert_eq!(
            handoff_mention("CHANGES_REQUESTED", "013-x", "7").unwrap(),
            ".stateful-spec/history/handoff-013-x-m7-review.md"
        );
        assert_eq!(handoff_mention("SPEC_READY", "013-x", "7"), None);
        assert_eq!(handoff_mention("ARCH_REVIEW", "013-x", "7"), None);
        assert_eq!(handoff_mention("CHANGES_REQUESTED", "", "7"), None);
    }

    #[test]
    fn test_spec_mention_only_on_arch_review() {
        assert_eq!(
            spec_mention("ARCH_REVIEW", "030", "3").unwrap(),
            ".stateful-spec/history/030-m3-spec.md"
        );
        assert_eq!(spec_mention("SPEC_READY", "030", "3"), None);
        assert_eq!(spec_mention("ARCH_REVIEW", "030", ""), None);
    }

    #[test]
    fn test_find_in_path_resolves_existing_file() {
        let dir = std::env::temp_dir().join(format!("flow-spawn-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let exe = dir.join("agent-runner");
        std::fs::write(&exe, b"#!/bin/sh\n").unwrap();
        let found = find_in_path(
            Some(dir.clone().into_os_string()),
            &["agent-runner".to_string()],
        );
        assert_eq!(found.as_deref(), Some(exe.as_path()));
        assert!(find_in_path(Some(dir.clone().into_os_string()), &["nope-xyz".to_string()]).is_none());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[cfg(windows)]
    #[test]
    fn test_ps_single_quote_escapes_inner_quotes() {
        assert_eq!(ps_single_quote("a'b"), "'a''b'");
        assert_eq!(ps_single_quote("plain"), "'plain'");
    }

    #[test]
    fn test_zombie_guard() {
        assert_eq!(zombie_guard(0), ZombieGuard::Ok);
        assert_eq!(zombie_guard(1), ZombieGuard::KillStaleThenSpawn { live: 1 });
        assert_eq!(zombie_guard(3), ZombieGuard::KillStaleThenSpawn { live: 3 });
    }
}
