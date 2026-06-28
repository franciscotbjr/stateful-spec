// Minimal, hand-parsed INI-lite config for the configurable parts of `flow`:
// the `spawn` command templates and the git default-branch set. Lives at
// `<repo_root>/.stateful-spec/flow.conf` (or a `--config <path>` override).

import { readFileSync } from "node:fs";
import { join } from "node:path";

export class Config {
  spawnProgram?: string;
  spawnArchReview?: string;
  spawnEngineer?: string;
  spawnCommit?: string;
  defaultBranches: string[] = ["main", "master"];

  static defaults(): Config {
    return new Config();
  }

  static parse(raw: string): Config {
    const c = new Config();
    let section = "";
    for (let line of raw.split("\n")) {
      line = line.trim();
      if (line === "" || line.startsWith("#") || line.startsWith(";")) continue;
      if (line.startsWith("[") && line.endsWith("]")) {
        section = line.slice(1, -1).trim().toLowerCase();
        continue;
      }
      const eq = line.indexOf("=");
      if (eq < 0) continue;
      const key = line.slice(0, eq).trim().toLowerCase();
      const value = line.slice(eq + 1).trim();
      if (section === "spawn") {
        if (key === "program") c.spawnProgram = value;
        else if (key === "arch_review") c.spawnArchReview = value;
        else if (key === "engineer") c.spawnEngineer = value;
        else if (key === "commit") c.spawnCommit = value;
      } else if (section === "git" && key === "default_branches") {
        const list = value
          .split(",")
          .map((s) => s.trim())
          .filter((s) => s !== "");
        if (list.length > 0) c.defaultBranches = list;
      }
    }
    return c;
  }

  static load(repoRoot: string, overridePath?: string): Config {
    const path = overridePath ?? join(repoRoot, ".stateful-spec", "flow.conf");
    try {
      return Config.parse(readFileSync(path, "utf8"));
    } catch {
      return Config.defaults();
    }
  }

  spawnTemplate(kind: string): string | undefined {
    if (kind === "arch-review") return this.spawnArchReview;
    if (kind === "engineer") return this.spawnEngineer;
    if (kind === "commit") return this.spawnCommit;
    return undefined;
  }
}
