#!/usr/bin/env node
// `flow` — the Stateful Spec multi-agent state CLI (write-side + poll).
// Node/TypeScript twin of the Rust `stateful-spec-flow` crate: identical verbs,
// state-file contract, and exit codes. Cross-platform, zero runtime dependencies.

import { existsSync, readFileSync } from "node:fs";
import { cwd } from "node:process";
import { dirname, isAbsolute, join, resolve } from "node:path";

import { Config } from "./config.js";
import { FrontMatter } from "./frontmatter.js";
import * as git from "./git.js";
import { nowIsoUtc, writeAtomic } from "./io.js";
import { decide, StallWatch } from "./poll.js";
import * as spawn from "./spawn.js";
import { apply, type Verb } from "./transition.js";

interface Opts {
  flowFile?: string;
  config?: string;
  message?: string;
  role: string;
  interval: number;
  timeout: number;
  stallAfter: number;
  once: boolean;
  allowDirty: boolean;
  profile?: string;
  workingDir?: string;
  gate?: string;
  total?: number;
  model?: string;
  positionals: string[];
}

function fail(msg: string, code: number): never {
  process.stderr.write(`flow: ${msg}\n`);
  process.exit(code);
}

function parseOpts(args: string[]): Opts {
  const o: Opts = {
    role: "engineer",
    interval: 5,
    timeout: 0,
    stallAfter: 900,
    once: false,
    allowDirty: false,
    positionals: [],
  };
  let i = 0;
  const nextVal = (): string | undefined => {
    if (i + 1 < args.length) {
      i += 1;
      return args[i];
    }
    return undefined;
  };
  while (i < args.length) {
    const a = args[i];
    switch (a) {
      case "--flow-file": o.flowFile = nextVal(); break;
      case "--config": o.config = nextVal(); break;
      case "--message":
      case "-m": o.message = nextVal(); break;
      case "--role": { const v = nextVal(); if (v !== undefined) o.role = v; break; }
      case "--interval": { const v = nextVal(); o.interval = v ? parseIntOr(v, 5) : 5; break; }
      case "--timeout": { const v = nextVal(); o.timeout = v ? parseIntOr(v, 0) : 0; break; }
      case "--stall-after": { const v = nextVal(); o.stallAfter = v ? parseIntOr(v, 900) : 900; break; }
      case "--once": o.once = true; break;
      case "--allow-dirty": o.allowDirty = true; break;
      case "--profile": o.profile = nextVal(); break;
      case "--working-dir": o.workingDir = nextVal(); break;
      case "--gate": o.gate = nextVal(); break;
      case "--total": { const v = nextVal(); if (v && /^\d+$/.test(v)) o.total = parseInt(v, 10); break; }
      case "--model": o.model = nextVal(); break;
      default: o.positionals.push(a); break;
    }
    i += 1;
  }
  return o;
}

function parseIntOr(v: string, dflt: number): number {
  return /^\d+$/.test(v) ? parseInt(v, 10) : dflt;
}

function reason(o: Opts): string | undefined {
  if (o.message !== undefined) return o.message;
  return o.positionals.length > 0 ? o.positionals.join(" ") : undefined;
}

function locate(o: Opts): string {
  if (o.flowFile !== undefined) {
    const f = isAbsolute(o.flowFile) ? o.flowFile : resolve(o.flowFile);
    if (existsSync(f)) return f;
    fail(`flow file not found: ${o.flowFile}`, 3);
  }
  let dir = cwd();
  for (;;) {
    const cand = join(dir, ".stateful-spec", "flow-state.md");
    if (existsSync(cand)) return cand;
    const parent = dirname(dir);
    if (parent === dir) {
      fail("could not locate .stateful-spec/flow-state.md (use --flow-file)", 3);
    }
    dir = parent;
  }
}

function repoRoot(flowFile: string): string {
  return dirname(dirname(flowFile));
}

function buildVerb(verb: string, o: Opts): Verb {
  const needReason = (): string => {
    const r = reason(o);
    if (r === undefined) fail(`${verb} requires a reason`, 2);
    return r;
  };
  switch (verb) {
    case "spec-ready": return { kind: "spec-ready" };
    case "approve": return { kind: "approve" };
    case "submit": return { kind: "submit" };
    case "advance": return { kind: "advance" };
    case "submit-spec": return { kind: "submit-spec" };
    case "approve-spec": return { kind: "approve-spec" };
    case "block": return { kind: "block", reason: needReason() };
    case "request-changes": return { kind: "request-changes", reason: needReason() };
    case "request-spec-changes": return { kind: "request-spec-changes", reason: needReason() };
    case "hand-back": return { kind: "hand-back", reason: needReason() };
    case "approve-plan": {
      if (o.total === undefined) fail("approve-plan requires --total <n>", 2);
      return { kind: "approve-plan", total: o.total };
    }
    default: throw new Error("unreachable verb");
  }
}

function cmdTransition(verb: string, o: Opts): void {
  const path = locate(o);
  let content: string;
  try {
    content = readFileSync(path, "utf8");
  } catch (e) {
    fail(`cannot read ${path}: ${String(e)}`, 3);
  }
  let parsed: { fm: FrontMatter; body: string };
  try {
    parsed = FrontMatter.parse(content);
  } catch (e) {
    fail(String((e as Error).message ?? e), 3);
  }
  const { fm } = parsed;
  let { body } = parsed;

  const v = buildVerb(verb, o);

  // The `advance` gates: refuse on a default branch and while product code is
  // uncommitted. Both run before apply, so a refused advance leaves state untouched.
  if (verb === "advance" && !o.allowDirty) {
    const repo = repoRoot(path);
    const cfg = Config.load(repo, o.config);
    try {
      const b = git.gitCurrentBranch(repo);
      if (git.isDefaultBranch(b, cfg.defaultBranches)) {
        process.stderr.write(`flow advance refused: on default branch '${b}'.\n`);
        process.stderr.write("create/switch to a feature branch before advancing.\n");
        process.exit(1);
      }
    } catch (e) {
      process.stderr.write(`flow advance: branch check failed: ${String((e as Error).message ?? e)}\n`);
      process.exit(1);
    }
    try {
      const dirty = git.productTreeDirty(repo);
      if (dirty.length > 0) {
        process.stderr.write("flow advance refused: uncommitted product changes:\n");
        for (const p of dirty) process.stderr.write(`  ${p}\n`);
        process.stderr.write("commit the milestone first, or pass --allow-dirty.\n");
        process.exit(1);
      }
    } catch (e) {
      process.stderr.write(`flow advance: git check failed: ${String((e as Error).message ?? e)}\n`);
      process.exit(1);
    }
  }

  const advanceCtx: [string, number] | null =
    verb === "advance" ? [fm.get("iteration") ?? "", fm.getU64("current_milestone")] : null;

  let outcome;
  try {
    outcome = apply(fm, v);
  } catch (e) {
    fail(`${verb} refused: ${String((e as Error).message ?? e)}`, 1);
  }

  const now = nowIsoUtc();
  fm.set("updated_at", now);
  const seq = fm.getU64("seq");
  if (!body.endsWith("\n")) body += "\n";
  const action =
    o.gate !== undefined && verb === "submit"
      ? `${outcome.action} · gate=[${o.gate}]`
      : outcome.action;
  body += `- ${now} · #${seq} · ${outcome.role} · ${action} · flow-state.md\n`;

  try {
    writeAtomic(path, fm.render(body));
  } catch (e) {
    fail(`write failed: ${String((e as Error).message ?? e)}`, 3);
  }

  process.stdout.write(
    `OK ${verb}: status=${fm.get("process_status") ?? "?"} step=${fm.get("step") ?? "?"} ` +
      `turn=${fm.get("turn") ?? "?"} m=${fm.get("current_milestone") ?? "?"}/${fm.get("total_milestones") ?? "?"} ` +
      `round=${fm.get("review_round") ?? "?"} seq=${seq}\n`,
  );
  const br = fm.get("blocked_reason");
  if (br !== undefined && br !== "null") process.stdout.write(`BLOCKED: ${br}\n`);

  if (advanceCtx) {
    const repo = repoRoot(path);
    try {
      const moved = git.archiveMilestoneAux(repo, advanceCtx[0], advanceCtx[1]);
      if (moved.length > 0) {
        process.stdout.write(`archived milestone ${advanceCtx[1]} auxiliaries: ${moved.join(", ")}\n`);
      }
    } catch (e) {
      process.stderr.write(`flow advance: archive warning: ${String((e as Error).message ?? e)}\n`);
    }
  }
}

function cmdStatus(o: Opts): void {
  const path = locate(o);
  const { fm } = FrontMatter.parse(readFileSync(path, "utf8"));
  process.stdout.write(
    `status=${fm.get("process_status") ?? "?"} turn=${fm.get("turn") ?? "?"} step=${fm.get("step") ?? "?"} ` +
      `m=${fm.get("current_milestone") ?? "?"}/${fm.get("total_milestones") ?? "?"} ` +
      `round=${fm.get("review_round") ?? "?"}/${fm.get("max_review_rounds") ?? "?"} ` +
      `seq=${fm.get("seq") ?? "0"} updated_at=${fm.get("updated_at") ?? "?"}\n`,
  );
  const br = fm.get("blocked_reason");
  if (br !== undefined && br !== "null") process.stdout.write(`blocked_reason: ${br}\n`);
}

function cmdSpawn(o: Opts): void {
  const statePath = locate(o);
  const repo = repoRoot(statePath);
  const cwdDir = o.workingDir ?? repo;

  let step: string, iteration: string, milestone: string;
  try {
    const { fm } = FrontMatter.parse(readFileSync(statePath, "utf8"));
    step = fm.get("step") ?? "?";
    iteration = fm.get("iteration") ?? "";
    milestone = fm.get("current_milestone") ?? "";
  } catch (e) {
    process.stderr.write(`SPAWN_FAILED: cannot read flow state ${statePath}: ${String(e)}\n`);
    spawn.writeSpawnResult(repo, 3, 0, "unknown");
    process.exit(3);
  }

  const kind = spawn.spawnKind(step);
  const role = spawn.roleFor(step);
  const cfg = Config.load(repo, o.config);

  const specRel = spawn.specMention(step, iteration, milestone);
  const spec = specRel && existsSync(join(repo, specRel)) ? specRel : "";
  const handoffRel = spawn.handoffMention(step, iteration, milestone);
  const handoff = handoffRel && existsSync(join(repo, handoffRel)) ? handoffRel : "";
  const verdict = kind === "arch-review" ? spawn.VERDICT_DIRECTIVE : "";
  const profile = o.profile ?? "";
  if (spec !== "") process.stdout.write(`flow spawn: spec mention attached (${spec})\n`);
  if (handoff !== "") process.stdout.write(`flow spawn: handoff mention attached (${handoff})\n`);

  const template = cfg.spawnTemplate(kind);
  if (template === undefined) {
    process.stderr.write(`SPAWN_FAILED: no '${kind}' spawn template configured.\n`);
    process.stderr.write(
      `set [spawn] ${kind.replace("-", "_")} in .stateful-spec/flow.conf (see packages/flow-ts/README.md).\n`,
    );
    spawn.writeSpawnResult(repo, 2, 0, kind);
    process.exit(2);
  }
  if (cfg.spawnProgram === undefined) {
    process.stderr.write("SPAWN_FAILED: no [spawn] program configured in .stateful-spec/flow.conf\n");
    spawn.writeSpawnResult(repo, 2, 0, kind);
    process.exit(2);
  }
  const prog = spawn.resolveProgram(cfg.spawnProgram);
  if (prog === undefined) {
    process.stderr.write(`SPAWN_FAILED: spawn program '${cfg.spawnProgram}' not found on PATH\n`);
    spawn.writeSpawnResult(repo, 1, 0, kind);
    process.exit(1);
  }

  const subs: spawn.Subs = { role, profile, spec, handoff, verdictDirective: verdict };
  const args: string[] = [];
  if (o.model !== undefined) {
    args.push("--model", o.model);
  }
  args.push(...spawn.buildArgs(template, subs));

  try {
    const pid = spawn.launch(prog, args, cwdDir);
    process.stdout.write(`AGENT_SPAWNED pid=${pid} (${kind})\n`);
    spawn.writeSpawnResult(repo, 0, pid, kind);
  } catch (e) {
    process.stderr.write(`SPAWN_FAILED: ${String((e as Error).message ?? e)}\n`);
    spawn.writeSpawnResult(repo, 2, 0, kind);
    process.exit(2);
  }
}

function sleepSync(secs: number): void {
  const ms = secs * 1000;
  if (ms <= 0) return;
  Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, ms);
}

function cmdPoll(o: Opts): void {
  const path = locate(o);
  const start = Date.now();
  const watch = new StallWatch();
  for (;;) {
    let fm: FrontMatter;
    try {
      fm = FrontMatter.parse(readFileSync(path, "utf8")).fm;
    } catch (e) {
      if (o.once) fail(`poll: ${String((e as Error).message ?? e)}`, 3);
      if (o.timeout > 0 && (Date.now() - start) / 1000 >= o.timeout) {
        process.stderr.write(`flow poll: timeout after ${o.timeout}s (state unreadable)\n`);
        process.exit(4);
      }
      sleepSync(o.interval);
      continue;
    }
    const status = fm.get("process_status") ?? "";
    const turn = fm.get("turn") ?? "";
    const blocked = fm.get("blocked_reason") ?? "";
    const d = decide(status, turn, o.role, blocked);
    if (d.kind === "done") {
      process.stdout.write("DONE\n");
      process.exit(1);
    } else if (d.kind === "blocked") {
      process.stdout.write(`BLOCKED: ${d.reason}\n`);
      process.exit(2);
    } else if (d.kind === "turn") {
      process.stdout.write(
        `>>> ${o.role} TURN (step=${fm.get("step") ?? "?"} m=${fm.get("current_milestone") ?? "?"})\n`,
      );
      process.exit(0);
    } else {
      if (o.once) {
        process.stdout.write(`waiting (turn=${d.turn})\n`);
        process.exit(5);
      }
      if (o.timeout > 0 && (Date.now() - start) / 1000 >= o.timeout) {
        process.stderr.write(`flow poll: timeout after ${o.timeout}s waiting for turn=${o.role}\n`);
        process.exit(4);
      }
      const secs = watch.observe(fm.get("seq") ?? "", o.interval, o.stallAfter);
      if (secs !== null) {
        process.stderr.write(
          `flow poll: STALL WARNING — turn=${d.turn}, no flow-state change for ${secs}s ` +
            `(seq unchanged). Run \`flow status\` first (a flipped turn means it FINISHED), then ` +
            `judge liveness by the worker's activity, not a wall-clock timestamp.\n`,
        );
      }
      sleepSync(o.interval);
    }
  }
}

function usage(): void {
  process.stderr.write(
    [
      "flow — Stateful Spec multi-agent state CLI (validated transitions + poll)",
      "usage: flow <verb> [--flow-file <path>] [--config <path>] [options]",
      "  PM:    approve-plan --total <n> | spec-ready | approve | request-changes \"<reason>\" | block \"<reason>\"",
      "         submit-spec   (submit milestone spec to the architect — three-agent flow)",
      "  ARCH:  approve-spec | request-spec-changes \"<reason>\"   (design gate)",
      "  ENG:   submit [--gate \"fmt=0 test=0\"] | advance [--allow-dirty] | hand-back \"<reason>\"",
      "  spawn: spawn [--profile <name>] [--model <name>] [--working-dir <path>]",
      "         launches the next agent from the [spawn] command templates in .stateful-spec/flow.conf",
      "  util:  status | poll --role <pm|engineer|architect> [--timeout N] [--interval N] [--stall-after N] [--once]",
    ].join("\n") + "\n",
  );
}

function main(): void {
  const argv = process.argv.slice(2);
  if (argv.length === 0) {
    usage();
    process.exit(2);
  }
  const verb = argv[0];
  const o = parseOpts(argv.slice(1));
  switch (verb) {
    case "status": cmdStatus(o); break;
    case "poll": cmdPoll(o); break;
    case "spec-ready":
    case "approve":
    case "request-changes":
    case "submit":
    case "advance":
    case "block":
    case "approve-plan":
    case "submit-spec":
    case "approve-spec":
    case "request-spec-changes":
    case "hand-back": cmdTransition(verb, o); break;
    case "spawn": cmdSpawn(o); break;
    case "-h":
    case "--help":
    case "help": usage(); break;
    default:
      process.stderr.write(`flow: unknown verb '${verb}'\n`);
      usage();
      process.exit(2);
  }
}

main();
