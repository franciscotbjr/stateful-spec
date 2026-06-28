//! Robust parse/render of the YAML frontmatter of `flow-state.md`.
//!
//! The frontmatter is a flat `key: value` block delimited by two `---` lines,
//! followed by the append-only turn-log body. We preserve the body verbatim and
//! treat every value as a string, which makes the round-trip exact and robust to
//! schema additions.

/// The parsed frontmatter as an ordered list of `(key, value)` pairs.
///
/// Order is preserved so re-rendering keeps the file stable.
pub struct FrontMatter {
    entries: Vec<(String, String)>,
}

impl FrontMatter {
    /// Split `content` into its frontmatter and the body that follows the
    /// closing `---`. Returns an error if the two delimiters are not found.
    pub fn parse(content: &str) -> Result<(Self, String), String> {
        // Locate the two `---` delimiter lines by byte offset, preserving the
        // body exactly (we never reflow it).
        let mut markers: Vec<(usize, usize)> = Vec::new();
        let mut idx = 0usize;
        for line in content.split_inclusive('\n') {
            if line.trim_end() == "---" {
                markers.push((idx, idx + line.len()));
                if markers.len() == 2 {
                    break;
                }
            }
            idx += line.len();
        }
        if markers.len() < 2 {
            return Err("flow-state.md: frontmatter delimiters '---' not found".to_string());
        }

        let fm_start = markers[0].1; // just after the opening `---\n`
        let fm_end = markers[1].0; // just before the closing `---`
        let body_start = markers[1].1; // just after the closing `---\n`

        let fm_str = &content[fm_start..fm_end];
        let body = &content[body_start..];

        let mut entries = Vec::new();
        for raw in fm_str.lines() {
            let line = raw.trim_end();
            if line.trim().is_empty() {
                continue;
            }
            // Split on the first ':' — keys never contain a colon; values
            // (e.g. ISO timestamps) may.
            if let Some(pos) = line.find(':') {
                let key = line[..pos].trim().to_string();
                let value = line[pos + 1..].trim().to_string();
                if !key.is_empty() {
                    entries.push((key, value));
                }
            }
        }

        Ok((FrontMatter { entries }, body.to_string()))
    }

    /// Re-emit the document: `---` + entries + `---` + the preserved body.
    pub fn render(&self, body: &str) -> String {
        let mut s = String::from("---\n");
        for (k, v) in &self.entries {
            s.push_str(k);
            s.push_str(": ");
            s.push_str(v);
            s.push('\n');
        }
        s.push_str("---\n");
        s.push_str(body);
        s
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
    }

    /// Update an existing key in place, or append it if absent (preserving order).
    pub fn set(&mut self, key: &str, value: &str) {
        if let Some(entry) = self.entries.iter_mut().find(|(k, _)| k == key) {
            entry.1 = value.to_string();
        } else {
            self.entries.push((key.to_string(), value.to_string()));
        }
    }

    /// Parse a numeric field, defaulting to 0 when absent or unparseable.
    pub fn get_u64(&self, key: &str) -> u64 {
        self.get(key)
            .and_then(|v| v.trim().parse::<u64>().ok())
            .unwrap_or(0)
    }

    /// Increment the monotonic `seq` counter and return the new value.
    pub fn bump_seq(&mut self) -> u64 {
        let next = self.get_u64("seq") + 1;
        self.set("seq", &next.to_string());
        next
    }

    #[cfg(test)]
    pub fn from_pairs(pairs: &[(&str, &str)]) -> Self {
        FrontMatter {
            entries: pairs
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "---\nprocess_status: RUNNING\ncurrent_milestone: 14\nturn: pm\nstep: AWAITING_REVIEW\nblocked_reason: null\nupdated_at: 2026-06-09T12:30:00Z\n---\n\n# Flow turn-log\n\n- line one · flow-state.md\n";

    #[test]
    fn test_parse_roundtrip_is_exact() {
        let (fm, body) = FrontMatter::parse(SAMPLE).unwrap();
        assert_eq!(fm.get("process_status"), Some("RUNNING"));
        assert_eq!(fm.get("turn"), Some("pm"));
        // ISO timestamp with colons survives intact.
        assert_eq!(fm.get("updated_at"), Some("2026-06-09T12:30:00Z"));
        assert_eq!(fm.get("blocked_reason"), Some("null"));
        assert_eq!(fm.render(&body), SAMPLE);
    }

    #[test]
    fn test_set_updates_in_place_and_appends() {
        let (mut fm, _) = FrontMatter::parse(SAMPLE).unwrap();
        fm.set("turn", "engineer");
        assert_eq!(fm.get("turn"), Some("engineer"));
        assert!(fm.get("seq").is_none());
        assert_eq!(fm.bump_seq(), 1);
        assert_eq!(fm.get("seq"), Some("1"));
        assert_eq!(fm.bump_seq(), 2);
    }

    #[test]
    fn test_missing_delimiters_errors() {
        assert!(FrontMatter::parse("no frontmatter here").is_err());
    }
}
