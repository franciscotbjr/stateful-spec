// Conformance suite — drives the compiled CLI against the SAME shared fixtures
// as the Rust crate (`../../conformance/cases/*.txt`). Together they guarantee
// Rust ≡ TS behaviour. Zero-dependency line-based parser.

import { test } from "node:test";
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { mkdtempSync, mkdirSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url)); // dist/test
const cli = join(here, "..", "src", "cli.js"); // dist/src/cli.js
const casesDir = join(here, "..", "..", "..", "conformance", "cases"); // packages/conformance/cases

interface Step {
  run: string;
  exit: number;
  expect: [string, string][];
}

function parseCase(text: string): { initial: [string, string][]; steps: Step[] } {
  const initial: [string, string][] = [];
  const steps: Step[] = [];
  let section = "";
  for (const raw of text.split("\n")) {
    const line = raw.trim();
    if (line === "" || line.startsWith("#")) continue;
    if (line === "[initial]") {
      section = "initial";
      continue;
    }
    if (line === "[step]") {
      steps.push({ run: "", exit: 0, expect: [] });
      section = "step";
      continue;
    }
    const eq = line.indexOf("=");
    if (eq < 0) continue;
    const k = line.slice(0, eq).trim();
    const v = line.slice(eq + 1).trim();
    if (section === "initial") {
      initial.push([k, v]);
    } else if (section === "step") {
      const s = steps[steps.length - 1];
      if (k === "run") s.run = v;
      else if (k === "exit") s.exit = parseInt(v, 10);
      else if (k === "expect") {
        for (const pair of v.split(/\s+/)) {
          const e = pair.indexOf("=");
          if (e > 0) s.expect.push([pair.slice(0, e), pair.slice(e + 1)]);
        }
      }
    }
  }
  return { initial, steps };
}

function writeState(initial: [string, string][]): string {
  const dir = mkdtempSync(join(tmpdir(), "flow-conf-"));
  const ss = join(dir, ".stateful-spec");
  mkdirSync(ss, { recursive: true });
  const p = join(ss, "flow-state.md");
  let fm = "---\n";
  for (const [k, v] of initial) fm += `${k}: ${v}\n`;
  fm += "---\n\n# Turn-log\n";
  writeFileSync(p, fm);
  return p;
}

function runStep(flow: string, run: string): number {
  const parts = run.split(/\s+/);
  try {
    execFileSync("node", [cli, parts[0], "--flow-file", flow, ...parts.slice(1)], {
      encoding: "utf8",
    });
    return 0;
  } catch (e: unknown) {
    const err = e as { status?: number };
    return typeof err.status === "number" ? err.status : 1;
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

for (const file of readdirSync(casesDir)
  .filter((f) => f.endsWith(".txt"))
  .sort()) {
  test(`conformance: ${file}`, () => {
    const { initial, steps } = parseCase(readFileSync(join(casesDir, file), "utf8"));
    const flow = writeState(initial);
    steps.forEach((s, i) => {
      const code = runStep(flow, s.run);
      assert.equal(code, s.exit, `step ${i} '${s.run}' exit`);
      for (const [k, v] of s.expect) {
        assert.equal(field(flow, k), v, `step ${i} '${s.run}' field ${k}`);
      }
    });
  });
}
