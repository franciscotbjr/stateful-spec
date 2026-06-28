// Robust parse/render of the YAML frontmatter of `flow-state.md`.
// Flat `key: value` block delimited by two `---` lines, followed by the
// append-only turn-log body (preserved verbatim). Every value is a string.

export class FrontMatter {
  entries: [string, string][] = [];

  static parse(content: string): { fm: FrontMatter; body: string } {
    // Split keeping the trailing newline of each line (like split_inclusive).
    const pieces = content.split(/(?<=\n)/);
    const markers: [number, number][] = [];
    let idx = 0;
    for (const line of pieces) {
      if (line.replace(/[ \t\r\n]+$/, "") === "---") {
        markers.push([idx, idx + line.length]);
        if (markers.length === 2) break;
      }
      idx += line.length;
    }
    if (markers.length < 2) {
      throw new Error("flow-state.md: frontmatter delimiters '---' not found");
    }
    const fmStr = content.slice(markers[0][1], markers[1][0]);
    const body = content.slice(markers[1][1]);

    const fm = new FrontMatter();
    for (const raw of fmStr.split("\n")) {
      const line = raw.replace(/[ \t\r]+$/, "");
      if (line.trim() === "") continue;
      const pos = line.indexOf(":");
      if (pos >= 0) {
        const key = line.slice(0, pos).trim();
        const value = line.slice(pos + 1).trim();
        if (key !== "") fm.entries.push([key, value]);
      }
    }
    return { fm, body };
  }

  render(body: string): string {
    let s = "---\n";
    for (const [k, v] of this.entries) s += `${k}: ${v}\n`;
    s += "---\n" + body;
    return s;
  }

  get(key: string): string | undefined {
    const e = this.entries.find(([k]) => k === key);
    return e ? e[1] : undefined;
  }

  set(key: string, value: string): void {
    const e = this.entries.find(([k]) => k === key);
    if (e) e[1] = value;
    else this.entries.push([key, value]);
  }

  /** Parse a numeric field, defaulting to 0 when absent or not all-digits. */
  getU64(key: string): number {
    const v = this.get(key);
    if (v === undefined) return 0;
    const t = v.trim();
    return /^\d+$/.test(t) ? parseInt(t, 10) : 0;
  }

  bumpSeq(): number {
    const next = this.getU64("seq") + 1;
    this.set("seq", String(next));
    return next;
  }

  static fromPairs(pairs: [string, string][]): FrontMatter {
    const fm = new FrontMatter();
    fm.entries = pairs.map(([k, v]) => [k, v]);
    return fm;
  }
}
