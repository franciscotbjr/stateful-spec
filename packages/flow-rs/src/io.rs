//! Low-level IO helpers: ISO timestamp and atomic write. No domain knowledge.

use std::fs;
use std::io;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

/// Current UTC time as `YYYY-MM-DDTHH:MM:SSZ` (ISO-8601), with no dependency on
/// `chrono`/`time`. Used for `updated_at` and the turn-log timestamp.
pub fn now_iso_utc() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    let days = secs.div_euclid(86_400);
    let tod = secs.rem_euclid(86_400);
    let (h, mi, s) = (tod / 3600, (tod % 3600) / 60, tod % 60);
    let (y, m, d) = civil_from_days(days);
    format!("{y:04}-{m:02}-{d:02}T{h:02}:{mi:02}:{s:02}Z")
}

/// Convert days-since-Unix-epoch to a civil `(year, month, day)`.
/// Howard Hinnant's `civil_from_days` algorithm (public domain).
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097; // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365; // [0, 399]
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32; // [1, 31]
    let m = (if mp < 10 { mp + 3 } else { mp - 9 }) as u32; // [1, 12]
    (if m <= 2 { y + 1 } else { y }, m, d)
}

/// Write `content` to `path` atomically (temp file in the same dir + rename),
/// so a concurrent reader never sees a half-written `flow-state.md`.
pub fn write_atomic(path: &Path, content: &str) -> io::Result<()> {
    let dir = path.parent().unwrap_or_else(|| Path::new("."));
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("flow-state.md");
    let tmp = dir.join(format!(".{file_name}.tmp"));
    fs::write(&tmp, content)?;
    fs::rename(&tmp, path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iso_epoch_zero() {
        // 0 secs since epoch = 1970-01-01T00:00:00Z
        let (y, m, d) = civil_from_days(0);
        assert_eq!((y, m, d), (1970, 1, 1));
    }

    #[test]
    fn test_iso_known_date() {
        // 2026-06-07 is 20611 days after the epoch.
        let (y, m, d) = civil_from_days(20_611);
        assert_eq!((y, m, d), (2026, 6, 7));
    }

    #[test]
    fn test_now_iso_shape() {
        let s = now_iso_utc();
        assert_eq!(s.len(), 20); // YYYY-MM-DDTHH:MM:SSZ
        assert!(s.ends_with('Z'));
        assert_eq!(&s[4..5], "-");
    }
}
