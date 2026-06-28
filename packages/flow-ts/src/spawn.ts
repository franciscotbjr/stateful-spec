// `flow spawn` — launch the next agent for the multi-agent flow.
//
// Agnostic: the package knows HOW to spawn (resolve the program, build argv from
// a configured command template, launch, record the result); the project supplies
// WHAT to spawn via `[spawn]` command templates (see config.ts). Nothing about any
// agent runner or persona is hardcoded.

import { spawn as cpSpawn } from "node:child_process";
import { mkdirSync, statSync, writeFileSync } from "node:fs";
import { delimiter, isAbsolute, join } from "node:path";

/** Spawn kind for the current `step`: which command template applies. */
export function spawnKind(step: string): string {
  if (step === "APPROVED") return "commit";
  if (step === "ARCH_REVIEW") return "arch-review";
  return "engineer";
}

/** Role token for the current `step`. Substituted into `{role}`. */
export function roleFor(step: string): string {
  return step === "ARCH_REVIEW" ? "architect" : "engineer";
}

/** A generic, tool-independent verdict directive for `{verdict_directive}`. */
export const VERDICT_DIRECTIVE =
  "End your final message with a fenced verdict block containing exactly: " +
  "verdict: APPROVED|CHANGES_REQUESTED / milestone: <k> / reason: <one line, only if CHANGES_REQUESTED>.";

/** Review handoff path — only for a CHANGES_REQUESTED turn. Fills `{handoff}`. */
export function handoffMention(step: string, iteration: string, milestone: string): string | undefined {
  if (step !== "CHANGES_REQUESTED" || iteration === "" || milestone === "") return undefined;
  return `.stateful-spec/history/handoff-${iteration}-m${milestone}-review.md`;
}

/** Milestone spec draft path — only for an ARCH_REVIEW turn. Fills `{spec}`. */
export function specMention(step: string, iteration: string, milestone: string): string | undefined {
  if (step !== "ARCH_REVIEW" || iteration === "" || milestone === "") return undefined;
  return `.stateful-spec/history/${iteration}-m${milestone}-spec.md`;
}

export interface Subs {
  role: string;
  profile: string;
  spec: string;
  handoff: string;
  verdictDirective: string;
}

/** Tokenize (quote-aware), substitute per token, drop tokens resolving to empty. */
export function buildArgs(template: string, subs: Subs): string[] {
  return tokenize(template)
    .map((tok) => substitute(tok, subs))
    .filter((t) => t !== "");
}

function substitute(tok: string, s: Subs): string {
  return tok
    .replaceAll("{role}", s.role)
    .replaceAll("{profile}", s.profile)
    .replaceAll("{spec}", s.spec)
    .replaceAll("{handoff}", s.handoff)
    .replaceAll("{verdict_directive}", s.verdictDirective);
}

/** Split a command string into argv, keeping double-quoted spans as one token. */
export function tokenize(s: string): string[] {
  const out: string[] = [];
  let cur = "";
  let inQuote = false;
  let has = false;
  for (const ch of s) {
    if (ch === '"') {
      inQuote = !inQuote;
      has = true;
    } else if (/\s/.test(ch) && !inQuote) {
      if (has) {
        out.push(cur);
        cur = "";
        has = false;
      }
    } else {
      cur += ch;
      has = true;
    }
  }
  if (has) out.push(cur);
  return out;
}

function isFile(p: string): boolean {
  try {
    return statSync(p).isFile();
  } catch {
    return false;
  }
}

/** Resolve the spawn program: a path (used as-is if it exists) or a name on PATH. */
export function resolveProgram(name: string): string | undefined {
  if (isAbsolute(name) || name.includes("/") || name.includes("\\")) {
    return isFile(name) ? name : undefined;
  }
  const pathVar = process.env.PATH ?? "";
  const names = candidateNames(name);
  for (const dir of pathVar.split(delimiter)) {
    if (dir === "") continue;
    for (const n of names) {
      const cand = join(dir, n);
      if (isFile(cand)) return cand;
    }
  }
  return undefined;
}

function candidateNames(name: string): string[] {
  if (process.platform === "win32") {
    return [name, `${name}.cmd`, `${name}.exe`, `${name}.bat`];
  }
  return [name];
}

/** Launch `prog` with `args` in `cwd` as a detached background child; return PID.
 *  On Windows a shell is used so an npm `.cmd`/`.bat` shim runs correctly. */
export function launch(prog: string, args: string[], cwd: string): number {
  const useShell = process.platform === "win32";
  const child = cpSpawn(prog, args, {
    cwd,
    detached: true,
    stdio: "ignore",
    shell: useShell,
  });
  child.unref();
  if (child.pid === undefined) {
    throw new Error("could not obtain child pid");
  }
  return child.pid;
}

/** Write `spawn-result.txt` (read by a standby heartbeat). */
export function writeSpawnResult(repoRoot: string, exitCode: number, pid: number, kind: string): void {
  const dir = join(repoRoot, ".stateful-spec");
  try {
    mkdirSync(dir, { recursive: true });
  } catch {
    // ignore
  }
  const spawnedAt = Date.now();
  const content = `exit_code=${exitCode}\npid=${pid}\nkind=${kind}\nspawned_at=${spawnedAt}\n`;
  try {
    writeFileSync(join(dir, "spawn-result.txt"), content);
  } catch {
    // ignore
  }
}

export type ZombieGuard = { kind: "ok" } | { kind: "kill"; live: number };

/** Pure decision: kill stale agent processes before spawning a new one? */
export function zombieGuard(live: number): ZombieGuard {
  return live === 0 ? { kind: "ok" } : { kind: "kill", live };
}
