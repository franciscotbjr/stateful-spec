// Generic git helpers for the `advance` gates and milestone archiving.

import { execFileSync } from "node:child_process";
import { existsSync, mkdirSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";

interface GitResult {
  ok: boolean;
  stdout: string;
  stderr: string;
}

function git(repoRoot: string, args: string[]): GitResult {
  try {
    const stdout = execFileSync("git", args, {
      cwd: repoRoot,
      encoding: "utf8",
      stdio: ["ignore", "pipe", "pipe"],
    });
    return { ok: true, stdout, stderr: "" };
  } catch (e: unknown) {
    const err = e as { status?: number; stdout?: Buffer | string; stderr?: Buffer | string; message?: string };
    if (typeof err.status === "number") {
      return {
        ok: false,
        stdout: err.stdout?.toString() ?? "",
        stderr: err.stderr?.toString() ?? "",
      };
    }
    throw new Error(`could not run git: ${err.message ?? String(e)}`);
  }
}

/** Uncommitted product paths (anything outside `.stateful-spec/`). Empty = clean. */
export function productTreeDirty(repoRoot: string): string[] {
  const r = git(repoRoot, ["status", "--porcelain"]);
  if (!r.ok) throw new Error(`git status failed: ${r.stderr.trim()}`);
  return r.stdout
    .split("\n")
    .map((l) => (l.length >= 3 ? l.slice(3) : ""))
    .map((p) => p.trim().replace(/^"|"$/g, ""))
    .filter((p) => p !== "" && !p.startsWith(".stateful-spec/"));
}

/** The current git branch; `HEAD` when detached. */
export function gitCurrentBranch(repoRoot: string): string {
  const r = git(repoRoot, ["rev-parse", "--abbrev-ref", "HEAD"]);
  if (!r.ok) throw new Error(`git rev-parse failed: ${r.stderr.trim()}`);
  return r.stdout.trim();
}

/** Whether `branch` is in the configured default-branch set. Pure. */
export function isDefaultBranch(branch: string, defaults: string[]): boolean {
  return defaults.includes(branch);
}

/** Whether `name` is an auxiliary of milestone `k` for `iteration`. Pure. */
export function isMilestoneAux(name: string, iteration: string, k: number): boolean {
  if (iteration === "") return false;
  const spec = `${iteration}-m${k}-spec.md`;
  const handoffPrefix = `handoff-${iteration}-m${k}-review`;
  return name === spec || (name.startsWith(handoffPrefix) && name.endsWith(".md"));
}

/** Archive milestone `k`'s auxiliaries into `history/.archived/` via `git mv`.
 *  Idempotent; returns the moved names. */
export function archiveMilestoneAux(repoRoot: string, iteration: string, k: number): string[] {
  if (iteration === "") return [];
  const hist = join(repoRoot, ".stateful-spec", "history");
  const archived = join(hist, ".archived");
  if (!existsSync(hist)) return [];
  const names = readdirSync(hist)
    .filter((n) => {
      try {
        return statSync(join(hist, n)).isFile();
      } catch {
        return false;
      }
    })
    .filter((n) => isMilestoneAux(n, iteration, k))
    .sort();
  if (names.length === 0) return [];
  mkdirSync(archived, { recursive: true });
  const moved: string[] = [];
  for (const name of names) {
    const src = `.stateful-spec/history/${name}`;
    const dst = `.stateful-spec/history/.archived/${name}`;
    const r = git(repoRoot, ["mv", src, dst]);
    if (r.ok) moved.push(name);
    else console.error(`flow advance: archive skip ${name}: ${r.stderr.trim()}`);
  }
  return moved;
}
