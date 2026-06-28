//! Minimal, hand-parsed config (zero-dependency) for the configurable parts of
//! `flow`: the `spawn` command templates and the git default-branch set.
//!
//! Lives at `<repo_root>/.stateful-spec/flow.conf` (or a `--config <path>`
//! override). Absent file → built-in defaults (spawn unconfigured; default
//! branches `main`/`master`). INI-lite: `[section]` headers, `key = value`
//! lines, `#`/`;` comments.

use std::path::Path;

pub struct Config {
    /// The agent-runner program to launch on `spawn` (a name on PATH, or a path).
    pub spawn_program: Option<String>,
    /// Command template per spawn kind: `arch-review` | `engineer` | `commit`.
    /// Placeholders: `{role} {profile} {spec} {handoff} {verdict_directive}`.
    pub spawn_arch_review: Option<String>,
    pub spawn_engineer: Option<String>,
    pub spawn_commit: Option<String>,
    /// Branches a milestone must never land on (the `advance` branch guard).
    pub default_branches: Vec<String>,
}

impl Config {
    pub fn defaults() -> Self {
        Self {
            spawn_program: None,
            spawn_arch_review: None,
            spawn_engineer: None,
            spawn_commit: None,
            default_branches: vec!["main".to_string(), "master".to_string()],
        }
    }

    /// Parse INI-lite content. Unknown keys/sections are ignored.
    pub fn parse(raw: &str) -> Self {
        let mut c = Self::defaults();
        let mut section = String::new();
        for line in raw.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
                continue;
            }
            if line.starts_with('[') && line.ends_with(']') {
                section = line[1..line.len() - 1].trim().to_lowercase();
                continue;
            }
            let (key, value) = match line.split_once('=') {
                Some((k, v)) => (k.trim().to_lowercase(), v.trim().to_string()),
                None => continue,
            };
            match (section.as_str(), key.as_str()) {
                ("spawn", "program") => c.spawn_program = Some(value),
                ("spawn", "arch_review") => c.spawn_arch_review = Some(value),
                ("spawn", "engineer") => c.spawn_engineer = Some(value),
                ("spawn", "commit") => c.spawn_commit = Some(value),
                ("git", "default_branches") => {
                    let list: Vec<String> = value
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                    if !list.is_empty() {
                        c.default_branches = list;
                    }
                }
                _ => {}
            }
        }
        c
    }

    /// Load from `<repo_root>/.stateful-spec/flow.conf` (or `override_path`).
    /// Returns defaults when the file is absent or unreadable.
    pub fn load(repo_root: &Path, override_path: Option<&Path>) -> Self {
        let path = override_path
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| repo_root.join(".stateful-spec").join("flow.conf"));
        match std::fs::read_to_string(&path) {
            Ok(raw) => Self::parse(&raw),
            Err(_) => Self::defaults(),
        }
    }

    /// The command template for a spawn kind, if configured.
    pub fn spawn_template(&self, kind: &str) -> Option<&str> {
        match kind {
            "arch-review" => self.spawn_arch_review.as_deref(),
            "engineer" => self.spawn_engineer.as_deref(),
            "commit" => self.spawn_commit.as_deref(),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defaults() {
        let c = Config::defaults();
        assert_eq!(c.default_branches, vec!["main", "master"]);
        assert!(c.spawn_program.is_none());
        assert!(c.spawn_template("engineer").is_none());
    }

    #[test]
    fn test_parse_spawn_and_git() {
        let raw = "\
# comment
[spawn]
program = my-agent-runner
engineer = run \"/flow-engineer {profile}\" {handoff}
arch_review = run \"/flow-architect {profile} {verdict_directive}\" {spec}
commit = run /flow-commit

[git]
default_branches = trunk, release
";
        let c = Config::parse(raw);
        assert_eq!(c.spawn_program.as_deref(), Some("my-agent-runner"));
        assert_eq!(
            c.spawn_template("engineer"),
            Some("run \"/flow-engineer {profile}\" {handoff}")
        );
        assert_eq!(c.spawn_template("commit"), Some("run /flow-commit"));
        assert_eq!(c.default_branches, vec!["trunk", "release"]);
    }

    #[test]
    fn test_parse_empty_keeps_defaults() {
        let c = Config::parse("# only comments\n; nothing here\n");
        assert_eq!(c.default_branches, vec!["main", "master"]);
        assert!(c.spawn_program.is_none());
    }
}
