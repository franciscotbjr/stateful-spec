// Low-level IO helpers: ISO timestamp and atomic write. No domain knowledge.

import { writeFileSync, renameSync } from "node:fs";
import { dirname, basename, join } from "node:path";

function pad(n: number, w: number): string {
  return String(n).padStart(w, "0");
}

/** Convert days-since-Unix-epoch to a civil `[year, month, day]`.
 *  Howard Hinnant's `civil_from_days` algorithm (public domain). */
export function civilFromDays(z0: number): [number, number, number] {
  const z = z0 + 719468;
  const era = Math.floor((z >= 0 ? z : z - 146096) / 146097);
  const doe = z - era * 146097;
  const yoe = Math.floor(
    (doe - Math.floor(doe / 1460) + Math.floor(doe / 36524) - Math.floor(doe / 146096)) / 365,
  );
  const y = yoe + era * 400;
  const doy = doe - (365 * yoe + Math.floor(yoe / 4) - Math.floor(yoe / 100));
  const mp = Math.floor((5 * doy + 2) / 153);
  const d = doy - Math.floor((153 * mp + 2) / 5) + 1;
  const m = mp < 10 ? mp + 3 : mp - 9;
  return [m <= 2 ? y + 1 : y, m, d];
}

/** Current UTC as `YYYY-MM-DDTHH:MM:SSZ` (ISO-8601). */
export function nowIsoUtc(): string {
  const secs = Math.floor(Date.now() / 1000);
  const days = Math.floor(secs / 86400);
  const tod = ((secs % 86400) + 86400) % 86400;
  const h = Math.floor(tod / 3600);
  const mi = Math.floor((tod % 3600) / 60);
  const s = tod % 60;
  const [y, m, d] = civilFromDays(days);
  return `${pad(y, 4)}-${pad(m, 2)}-${pad(d, 2)}T${pad(h, 2)}:${pad(mi, 2)}:${pad(s, 2)}Z`;
}

/** Write `content` to `path` atomically (temp file in the same dir + rename). */
export function writeAtomic(path: string, content: string): void {
  const dir = dirname(path);
  const name = basename(path) || "flow-state.md";
  const tmp = join(dir, "." + name + ".tmp");
  writeFileSync(tmp, content);
  renameSync(tmp, path);
}
