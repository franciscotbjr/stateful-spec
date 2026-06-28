//! `flow` — the Stateful Spec multi-agent state CLI (write-side + poll).
//!
//! Instead of agents hand-editing the YAML frontmatter of
//! `.stateful-spec/flow-state.md`, they invoke **validated verbs** here. Each verb
//! checks its precondition, mutates the state atomically, appends a `#seq`-tagged
//! turn-log line, and (for `advance`) refuses while product code is uncommitted.
//!
//! Cross-platform and zero-dependency. The tool is OPTIONAL — the methodology
//! works without it (see the repository's `methodology/multi-agent-flow.md`).

mod config;
mod frontmatter;
mod git;
mod io;
mod poll;
mod spawn;
mod transition;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::thread::sleep;
use std::time::{Duration, Instant};

use frontmatter::FrontMatter;
use poll::PollDecision;
use transition::{apply, Verb};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        usage();
        exit(2);
    }
    let verb = args[0].clone();
    let opts = Opts::parse(&args[1..]);
    match verb.as_str() {
        "status" => cmd_status(&opts),
        "poll" => cmd_poll(&opts),
        "spec-ready" | "approve" | "request-changes" | "submit" | "advance" | "block"
        | "approve-plan" | "submit-spec" | "approve-spec" | "request-spec-changes" | "hand-back" => {
            cmd_transition(&verb, &opts)
        }
        "spawn" => cmd_spawn(&opts),
        "-h" | "--help" | "help" => usage(),
        other => {
            eprintln!("flow: unknown verb '{other}'");
            usage();
            exit(2);
        }
    }
}

/// Parsed command-line options (zero-dependency, hand-rolled).
struct Opts {
    flow_file: Option<PathBuf>,
    config: Option<PathBuf>,
    message: Option<String>,
    role: String,
    interval: u64,
    timeout: u64,
    stall_after: u64,
    once: bool,
    allow_dirty: bool,
    profile: Option<String>,
    working_dir: Option<PathBuf>,
    gate: Option<String>,
    total: Option<u64>,
    model: Option<String>,
    positionals: Vec<String>,
}

impl Opts {
    fn parse(args: &[String]) -> Opts {
        let mut o = Opts {
            flow_file: None,
            config: None,
            message: None,
            role: "engineer".to_string(),
            interval: 5,
            timeout: 0,
            stall_after: 900, // 15 min default; 0 disables
            once: false,
            allow_dirty: false,
            profile: None,
            working_dir: None,
            gate: None,
            total: None,
            model: None,
            positionals: Vec::new(),
        };
        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--flow-file" => o.flow_file = next(args, &mut i).map(PathBuf::from),
                "--config" => o.config = next(args, &mut i).map(PathBuf::from),
                "--message" | "-m" => o.message = next(args, &mut i),
                "--role" => {
                    if let Some(v) = next(args, &mut i) {
                        o.role = v;
                    }
                }
                "--interval" => {
                    if let Some(v) = next(args, &mut i) {
                        o.interval = v.parse().unwrap_or(5);
                    }
                }
                "--timeout" => {
                    if let Some(v) = next(args, &mut i) {
                        o.timeout = v.parse().unwrap_or(0);
                    }
                }
                "--stall-after" => {
                    if let Some(v) = next(args, &mut i) {
                        o.stall_after = v.parse().unwrap_or(900);
                    }
                }
                "--once" => o.once = true,
                "--allow-dirty" => o.allow_dirty = true,
                "--profile" => o.profile = next(args, &mut i),
                "--working-dir" => o.working_dir = next(args, &mut i).map(PathBuf::from),
                "--gate" => o.gate = next(args, &mut i),
                "--total" => {
                    if let Some(v) = next(args, &mut i) {
                        o.total = v.parse().ok();
                    }
                }
                "--model" => o.model = next(args, &mut i),
                other => o.positionals.push(other.to_string()),
            }
            i += 1;
        }
        o
    }

    /// The reason text for `request-changes`/`block`/etc. (from `-m` or positionals).
    fn reason(&self) -> Option<String> {
        self.message.clone().or_else(|| {
            if self.positionals.is_empty() {
                None
            } else {
                Some(self.positionals.join(" "))
            }
        })
    }
}

fn next(args: &[String], i: &mut usize) -> Option<String> {
    if *i + 1 < args.len() {
        *i += 1;
        Some(args[*i].clone())
    } else {
        None
    }
}

fn cmd_transition(verb: &str, opts: &Opts) {
    let path = locate(opts).unwrap_or_else(|e| fail(&e, 3));
    let content = fs::read_to_string(&path)
        .unwrap_or_else(|e| fail(&format!("cannot read {}: {e}", path.display()), 3));
    let (mut fm, body) = FrontMatter::parse(&content).unwrap_or_else(|e| fail(&e, 3));

    let v = match verb {
        "spec-ready" => Verb::SpecReady,
        "approve" => Verb::Approve,
        "submit" => Verb::Submit,
        "advance" => Verb::Advance,
        "request-changes" => Verb::RequestChanges(
            opts.reason()
                .unwrap_or_else(|| fail("request-changes requires a reason", 2)),
        ),
        "block" => Verb::Block(
            opts.reason()
                .unwrap_or_else(|| fail("block requires a reason", 2)),
        ),
        "approve-plan" => Verb::ApprovePlan(
            opts.total
                .unwrap_or_else(|| fail("approve-plan requires --total <n>", 2)),
        ),
        "submit-spec" => Verb::SubmitSpec,
        "approve-spec" => Verb::ApproveSpec,
        "request-spec-changes" => Verb::RequestSpecChanges(
            opts.reason()
                .unwrap_or_else(|| fail("request-spec-changes requires a reason", 2)),
        ),
        "hand-back" => Verb::HandBack(
            opts.reason()
                .unwrap_or_else(|| fail("hand-back requires a reason", 2)),
        ),
        _ => unreachable!(),
    };

    // The `advance` gates: refuse on a default branch, and refuse while product
    // code (outside .stateful-spec/) is uncommitted — kills the "commit spanning
    // two milestones" bug. Both run before `apply`, so a refused advance leaves
    // the state untouched.
    if verb == "advance" && !opts.allow_dirty {
        let repo = repo_root(&path);
        let cfg = config::Config::load(&repo, opts.config.as_deref());
        match git::git_current_branch(&repo) {
            Ok(b) if git::is_default_branch(&b, &cfg.default_branches) => {
                eprintln!("flow advance refused: on default branch '{b}'.");
                eprintln!("create/switch to a feature branch before advancing.");
                exit(1);
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("flow advance: branch check failed: {e}");
                exit(1);
            }
        }
        match git::product_tree_dirty(&repo) {
            Ok(dirty) if !dirty.is_empty() => {
                eprintln!("flow advance refused: uncommitted product changes:");
                for p in &dirty {
                    eprintln!("  {p}");
                }
                eprintln!("commit the milestone first, or pass --allow-dirty.");
                exit(1);
            }
            Err(e) => {
                eprintln!("flow advance: git check failed: {e}");
                exit(1);
            }
            _ => {}
        }
    }

    // Capture the milestone being completed before `apply` increments it, so a
    // successful `advance` can archive its auxiliaries.
    let advance_ctx: Option<(String, u64)> = if verb == "advance" {
        Some((
            fm.get("iteration").unwrap_or("").to_string(),
            fm.get_u64("current_milestone"),
        ))
    } else {
        None
    };

    let outcome = apply(&mut fm, &v).unwrap_or_else(|e| fail(&format!("{verb} refused: {e}"), 1));

    let now = io::now_iso_utc();
    fm.set("updated_at", &now);
    let seq = fm.get_u64("seq");
    let mut new_body = body;
    if !new_body.ends_with('\n') {
        new_body.push('\n');
    }
    // On `submit`, fold the real gate exit codes (`--gate "fmt=0 test=0 …"`) into
    // the turn-log line as durable handoff evidence (anti "false green").
    let action = match (&opts.gate, verb) {
        (Some(g), "submit") => format!("{} · gate=[{g}]", outcome.action),
        _ => outcome.action.clone(),
    };
    new_body.push_str(&format!(
        "- {now} · #{seq} · {} · {} · flow-state.md\n",
        outcome.role, action
    ));

    let rendered = fm.render(&new_body);
    if let Err(e) = io::write_atomic(&path, &rendered) {
        fail::<()>(&format!("write failed: {e}"), 3);
    }

    println!(
        "OK {verb}: status={} step={} turn={} m={}/{} round={} seq={}",
        fm.get("process_status").unwrap_or("?"),
        fm.get("step").unwrap_or("?"),
        fm.get("turn").unwrap_or("?"),
        fm.get("current_milestone").unwrap_or("?"),
        fm.get("total_milestones").unwrap_or("?"),
        fm.get("review_round").unwrap_or("?"),
        seq,
    );
    if let Some(r) = fm.get("blocked_reason") {
        if r != "null" {
            println!("BLOCKED: {r}");
        }
    }

    // Archive the just-completed milestone's auxiliaries (spec + handoffs) into
    // history/.archived/ by construction.
    if let Some((iteration, k)) = advance_ctx {
        let repo = repo_root(&path);
        match git::archive_milestone_aux(&repo, &iteration, k) {
            Ok(moved) if !moved.is_empty() => {
                println!("archived milestone {k} auxiliaries: {}", moved.join(", "));
            }
            Ok(_) => {}
            Err(e) => eprintln!("flow advance: archive warning: {e}"),
        }
    }
}

fn cmd_status(opts: &Opts) {
    let path = locate(opts).unwrap_or_else(|e| fail(&e, 3));
    let content = fs::read_to_string(&path).unwrap_or_else(|e| fail(&e.to_string(), 3));
    let (fm, _) = FrontMatter::parse(&content).unwrap_or_else(|e| fail(&e, 3));
    println!(
        "status={} turn={} step={} m={}/{} round={}/{} seq={} updated_at={}",
        fm.get("process_status").unwrap_or("?"),
        fm.get("turn").unwrap_or("?"),
        fm.get("step").unwrap_or("?"),
        fm.get("current_milestone").unwrap_or("?"),
        fm.get("total_milestones").unwrap_or("?"),
        fm.get("review_round").unwrap_or("?"),
        fm.get("max_review_rounds").unwrap_or("?"),
        fm.get("seq").unwrap_or("0"),
        fm.get("updated_at").unwrap_or("?"),
    );
    if let Some(r) = fm.get("blocked_reason") {
        if r != "null" {
            println!("blocked_reason: {r}");
        }
    }
}

/// Launch the next agent from the configured `[spawn]` command templates. The
/// spawn kind (commit | arch-review | engineer) is resolved from the flow `step`;
/// the project supplies the command via `.stateful-spec/flow.conf`.
/// Exit codes: 0 spawned · 1 program not found · 2 not configured / spawn failed ·
/// 3 flow state unreadable.
fn cmd_spawn(opts: &Opts) {
    let state_path = match locate(opts) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("SPAWN_FAILED: {e}");
            exit(3);
        }
    };
    let repo = repo_root(&state_path);
    let cwd = opts.working_dir.clone().unwrap_or_else(|| repo.clone());

    let parsed = fs::read_to_string(&state_path)
        .map_err(|e| e.to_string())
        .and_then(|c| FrontMatter::parse(&c).map(|(fm, _)| fm))
        .map(|fm| {
            (
                fm.get("step").unwrap_or("?").to_string(),
                fm.get("iteration").unwrap_or("").to_string(),
                fm.get("current_milestone").unwrap_or("").to_string(),
            )
        });
    let (step, iteration, milestone) = match parsed {
        Ok(t) => t,
        Err(e) => {
            eprintln!("SPAWN_FAILED: cannot read flow state {}: {e}", state_path.display());
            spawn::write_spawn_result(&repo, 3, 0, "unknown");
            exit(3);
        }
    };

    let kind = spawn::spawn_kind(&step);
    let role = spawn::role_for(&step);
    let cfg = config::Config::load(&repo, opts.config.as_deref());

    // Context mentions (existence-checked) fill {spec} / {handoff} in the template.
    let spec = spawn::spec_mention(&step, &iteration, &milestone)
        .filter(|rel| repo.join(rel).is_file())
        .unwrap_or_default();
    let handoff = spawn::handoff_mention(&step, &iteration, &milestone)
        .filter(|rel| repo.join(rel).is_file())
        .unwrap_or_default();
    let verdict = if kind == "arch-review" {
        spawn::VERDICT_DIRECTIVE
    } else {
        ""
    };
    let profile = opts.profile.as_deref().unwrap_or("");
    if !spec.is_empty() {
        println!("flow spawn: spec mention attached ({spec})");
    }
    if !handoff.is_empty() {
        println!("flow spawn: handoff mention attached ({handoff})");
    }

    let template = match cfg.spawn_template(kind) {
        Some(t) => t.to_string(),
        None => {
            eprintln!("SPAWN_FAILED: no '{kind}' spawn template configured.");
            eprintln!(
                "set [spawn] {key} in .stateful-spec/flow.conf (see packages/flow-rs/README.md).",
                key = kind.replace('-', "_")
            );
            spawn::write_spawn_result(&repo, 2, 0, kind);
            exit(2);
        }
    };
    let prog_name = match &cfg.spawn_program {
        Some(p) => p.clone(),
        None => {
            eprintln!("SPAWN_FAILED: no [spawn] program configured in .stateful-spec/flow.conf");
            spawn::write_spawn_result(&repo, 2, 0, kind);
            exit(2);
        }
    };
    let prog = match spawn::resolve_program(&prog_name) {
        Some(p) => p,
        None => {
            eprintln!("SPAWN_FAILED: spawn program '{prog_name}' not found on PATH");
            spawn::write_spawn_result(&repo, 1, 0, kind);
            exit(1);
        }
    };

    let subs = spawn::Subs {
        role,
        profile,
        spec: &spec,
        handoff: &handoff,
        verdict_directive: verdict,
    };
    let mut args: Vec<String> = Vec::new();
    if let Some(m) = &opts.model {
        args.push("--model".to_string());
        args.push(m.clone());
    }
    args.extend(spawn::build_args(&template, &subs));

    match spawn::launch(&prog, &args, &cwd) {
        Ok(pid) => {
            println!("AGENT_SPAWNED pid={pid} ({kind})");
            spawn::write_spawn_result(&repo, 0, pid, kind);
        }
        Err(e) => {
            eprintln!("SPAWN_FAILED: {e}");
            spawn::write_spawn_result(&repo, 2, 0, kind);
            exit(2);
        }
    }
}

/// Blocking poll. Exit codes: 0 your turn · 1 DONE · 2 BLOCKED · 3 IO error ·
/// 4 timeout · 5 waiting (with `--once`). A stall watchdog warns on stderr (does
/// not exit) when `seq` is unchanged for `--stall-after` seconds while waiting.
fn cmd_poll(opts: &Opts) {
    let path = locate(opts).unwrap_or_else(|e| fail(&e, 3));
    let start = Instant::now();
    let mut watch = poll::StallWatch::new();
    loop {
        let parsed = fs::read_to_string(&path)
            .map_err(|e| e.to_string())
            .and_then(|c| FrontMatter::parse(&c).map(|(fm, _)| fm));
        let fm = match parsed {
            Ok(fm) => fm,
            Err(e) => {
                if opts.once {
                    fail::<()>(&format!("poll: {e}"), 3);
                }
                if opts.timeout > 0 && start.elapsed().as_secs() >= opts.timeout {
                    eprintln!(
                        "flow poll: timeout after {}s (state unreadable: {e})",
                        opts.timeout
                    );
                    exit(4);
                }
                sleep(Duration::from_secs(opts.interval));
                continue;
            }
        };
        let status = fm.get("process_status").unwrap_or("");
        let turn = fm.get("turn").unwrap_or("");
        let blocked = fm.get("blocked_reason").unwrap_or("");
        match poll::decide(status, turn, &opts.role, blocked) {
            PollDecision::Done => {
                println!("DONE");
                exit(1);
            }
            PollDecision::Blocked(reason) => {
                println!("BLOCKED: {reason}");
                exit(2);
            }
            PollDecision::Turn => {
                println!(
                    ">>> {} TURN (step={} m={})",
                    opts.role,
                    fm.get("step").unwrap_or("?"),
                    fm.get("current_milestone").unwrap_or("?"),
                );
                exit(0);
            }
            PollDecision::Waiting(t) => {
                if opts.once {
                    println!("waiting (turn={t})");
                    exit(5);
                }
                if opts.timeout > 0 && start.elapsed().as_secs() >= opts.timeout {
                    eprintln!(
                        "flow poll: timeout after {}s waiting for turn={}",
                        opts.timeout, opts.role
                    );
                    exit(4);
                }
                if let Some(secs) =
                    watch.observe(fm.get("seq").unwrap_or(""), opts.interval, opts.stall_after)
                {
                    eprintln!(
                        "flow poll: STALL WARNING — turn={t}, no flow-state change for {secs}s \
                         (seq unchanged). Before declaring the other agent dead, run `flow status` \
                         (a flipped turn means it FINISHED), then judge liveness by the worker's \
                         activity (CPU delta / new files), not by a wall-clock timestamp."
                    );
                }
                sleep(Duration::from_secs(opts.interval));
            }
        }
    }
}

/// Find `.stateful-spec/flow-state.md`, walking up from the CWD, unless overridden.
fn locate(opts: &Opts) -> Result<PathBuf, String> {
    if let Some(f) = &opts.flow_file {
        return if f.exists() {
            Ok(f.clone())
        } else {
            Err(format!("flow file not found: {}", f.display()))
        };
    }
    let mut dir = env::current_dir().map_err(|e| e.to_string())?;
    loop {
        let cand = dir.join(".stateful-spec").join("flow-state.md");
        if cand.exists() {
            return Ok(cand);
        }
        if !dir.pop() {
            return Err("could not locate .stateful-spec/flow-state.md (use --flow-file)".to_string());
        }
    }
}

/// `<root>/.stateful-spec/flow-state.md` → `<root>`.
fn repo_root(flow_file: &Path) -> PathBuf {
    flow_file
        .parent()
        .and_then(|p| p.parent())
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."))
}

fn fail<T>(msg: &str, code: i32) -> T {
    eprintln!("flow: {msg}");
    exit(code);
}

fn usage() {
    eprintln!("flow — Stateful Spec multi-agent state CLI (validated transitions + poll)");
    eprintln!("usage: flow <verb> [--flow-file <path>] [--config <path>] [options]");
    eprintln!("  PM:    approve-plan --total <n>   (start the run after plan approval)");
    eprintln!("         spec-ready | approve | request-changes \"<reason>\" | block \"<reason>\"");
    eprintln!("         submit-spec   (submit milestone spec to the architect — three-agent flow)");
    eprintln!("  ARCH:  approve-spec | request-spec-changes \"<reason>\"   (design gate)");
    eprintln!("  ENG:   submit [--gate \"fmt=0 test=0 build=0\"] | advance [--allow-dirty]");
    eprintln!("         hand-back \"<reason>\"   (return milestone to PM for re-scope)");
    eprintln!("  spawn: spawn [--profile <name>] [--model <name>] [--working-dir <path>]");
    eprintln!("         launches the next agent using the [spawn] command templates in");
    eprintln!("         .stateful-spec/flow.conf (kind from step: commit | arch-review | engineer).");
    eprintln!("  util:  status | poll --role <pm|engineer|architect> [--timeout N] [--interval N]");
    eprintln!("         [--stall-after N] [--once]");
}
