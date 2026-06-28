import { test } from "node:test";
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { mkdtempSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

import { FrontMatter } from "../src/frontmatter.js";
import { apply } from "../src/transition.js";
import { Config } from "../src/config.js";

const here = dirname(fileURLToPath(import.meta.url));
const cli = join(here, "..", "src", "cli.js"); // dist/src/cli.js after compile

function tempState(fields: [string, string][]): string {
  const dir = mkdtempSync(join(tmpdir(), "flow-ts-it-"));
  const ss = join(dir, ".stateful-spec");
  mkdirSync(ss, { recursive: true });
  const p = join(ss, "flow-state.md");
  let fm = "---\n";
  for (const [k, v] of fields) fm += `${k}: ${v}\n`;
  fm += "---\n\n# Turn-log\n";
  writeFileSync(p, fm);
  return p;
}

function run(flow: string, verb: string, extra: string[] = []): { code: number; stdout: string } {
  try {
    const stdout = execFileSync("node", [cli, verb, "--flow-file", flow, ...extra], {
      encoding: "utf8",
    });
    return { code: 0, stdout };
  } catch (e: unknown) {
    const err = e as { status?: number; stdout?: Buffer | string };
    return {
      code: typeof err.status === "number" ? err.status : 1,
      stdout: err.stdout?.toString() ?? "",
    };
  }
}

function field(flow: string, key: string): string {
  const content = readFileSync(flow, "utf8");
  const prefix = `${key}:`;
  for (const line of content.split("\n")) {
    const t = line.trim();
    if (t.startsWith(prefix)) return t.slice(prefix.length).trim();
  }
  return "";
}

function running(m: string, total: string, turn: string, step: string): [string, string][] {
  return [
    ["process_status", "RUNNING"],
    ["iteration", "013-demo"],
    ["total_milestones", total],
    ["current_milestone", m],
    ["turn", turn],
    ["step", step],
    ["review_round", "0"],
    ["max_review_rounds", "3"],
    ["spec_review_round", "0"],
    ["max_spec_review_rounds", "3"],
    ["seq", "0"],
  ];
}

// ── unit ──────────────────────────────────────────────────────────────

test("frontmatter roundtrip is exact", () => {
  const sample = "---\nprocess_status: RUNNING\nturn: pm\nupdated_at: 2026-06-09T12:30:00Z\n---\n\n# log\n";
  const { fm, body } = FrontMatter.parse(sample);
  assert.equal(fm.get("turn"), "pm");
  assert.equal(fm.get("updated_at"), "2026-06-09T12:30:00Z");
  assert.equal(fm.render(body), sample);
});

test("transition happy path + seq bump", () => {
  const fm = FrontMatter.fromPairs([
    ["process_status", "RUNNING"],
    ["turn", "pm"],
    ["step", "SPEC_APPROVED"],
  ]);
  apply(fm, { kind: "spec-ready" });
  assert.equal(fm.get("step"), "SPEC_READY");
  assert.equal(fm.get("turn"), "engineer");
  assert.equal(fm.get("seq"), "1");
});

test("transition rejects illegal and leaves state untouched", () => {
  const fm = FrontMatter.fromPairs([
    ["process_status", "RUNNING"],
    ["turn", "engineer"],
    ["step", "APPROVED"],
  ]);
  assert.throws(() => apply(fm, { kind: "spec-ready" }));
  assert.equal(fm.get("step"), "APPROVED");
  assert.equal(fm.get("seq"), undefined);
});

test("config parse spawn + git", () => {
  const c = Config.parse(
    "[spawn]\nprogram = my-runner\nengineer = run {profile}\n[git]\ndefault_branches = trunk, release\n",
  );
  assert.equal(c.spawnProgram, "my-runner");
  assert.equal(c.spawnTemplate("engineer"), "run {profile}");
  assert.deepEqual(c.defaultBranches, ["trunk", "release"]);
});

// ── CLI integration ───────────────────────────────────────────────────

test("full cycle via binary", () => {
  const flow = tempState(running("1", "3", "engineer", "SPEC_READY"));
  assert.equal(run(flow, "submit").code, 0);
  assert.equal(field(flow, "step"), "AWAITING_REVIEW");
  assert.equal(run(flow, "approve").code, 0);
  assert.equal(field(flow, "step"), "APPROVED");
  assert.equal(run(flow, "advance", ["--allow-dirty"]).code, 0);
  assert.equal(field(flow, "current_milestone"), "2");
  assert.equal(field(flow, "step"), "SPEC_PENDING");
  assert.equal(field(flow, "seq"), "3");
});

test("illegal transition rejected, state untouched", () => {
  const flow = tempState(running("1", "3", "engineer", "APPROVED"));
  assert.equal(run(flow, "spec-ready").code, 1);
  assert.equal(field(flow, "step"), "APPROVED");
  assert.equal(field(flow, "seq"), "0");
});

test("approve-plan starts run; missing --total is usage error", () => {
  const flow = tempState([
    ["process_status", "AWAITING_PLAN_APPROVAL"],
    ["turn", "pm"],
    ["step", "SPEC_PENDING"],
    ["seq", "0"],
  ]);
  assert.equal(run(flow, "approve-plan").code, 2);
  assert.equal(field(flow, "process_status"), "AWAITING_PLAN_APPROVAL");
  assert.equal(run(flow, "approve-plan", ["--total", "5"]).code, 0);
  assert.equal(field(flow, "process_status"), "RUNNING");
  assert.equal(field(flow, "total_milestones"), "5");
});

test("submit records gate evidence", () => {
  const flow = tempState(running("1", "3", "engineer", "SPEC_READY"));
  assert.equal(run(flow, "submit", ["--gate", "fmt=0 test=0"]).code, 0);
  assert.ok(readFileSync(flow, "utf8").includes("gate=[fmt=0 test=0]"));
});

test("design-gate cycle via binary", () => {
  const flow = tempState(running("1", "2", "pm", "SPEC_PENDING"));
  assert.equal(run(flow, "submit-spec").code, 0);
  assert.equal(field(flow, "step"), "ARCH_REVIEW");
  assert.equal(field(flow, "turn"), "architect");
  assert.equal(run(flow, "request-spec-changes", ["needs edge cases"]).code, 0);
  assert.equal(field(flow, "spec_review_round"), "1");
  assert.equal(run(flow, "submit-spec").code, 0);
  assert.equal(run(flow, "approve-spec").code, 0);
  assert.equal(field(flow, "step"), "SPEC_APPROVED");
  assert.equal(run(flow, "spec-ready").code, 0);
  assert.equal(field(flow, "step"), "SPEC_READY");
  assert.equal(field(flow, "turn"), "engineer");
});

test("poll --once branches", () => {
  const flow = tempState(running("1", "3", "engineer", "SPEC_READY"));
  assert.equal(run(flow, "poll", ["--role", "pm", "--once"]).code, 5);
  assert.equal(run(flow, "poll", ["--role", "engineer", "--once"]).code, 0);
});

test("status prints state", () => {
  const flow = tempState(running("2", "4", "pm", "AWAITING_REVIEW"));
  const r = run(flow, "status");
  assert.equal(r.code, 0);
  assert.ok(r.stdout.includes("status=RUNNING"));
  assert.ok(r.stdout.includes("m=2/4"));
});
