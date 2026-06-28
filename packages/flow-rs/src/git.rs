//! Generic git helpers for the `advance` gates and milestone archiving.
//! Every git interaction lives here; the commands are standard and portable.

use std::fs;
use std::path::Path;
use std::process::Command;

/// Returns the list of uncommitted **product** paths (anything outside
/// `.stateful-spec/`). An empty list means the milestone's work is committed.
///
/// This is the `advance` gate: it refuses to advance to the next milestone while
/// product code is uncommitted (the "commit spanning two milestones" bug), while
/// still allowing the methodology bookkeeping (`flow-state.md`, the iteration
/// file) to be dirty during the transition.
pub fn product_tree_dirty(repo_root: &Path) -> Result<Vec<String>, String> {
    let out = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .current_dir(repo_root)
        .output()
        .map_err(|e| format!("could not run git: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "git status failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    let stdout = String::from_utf8_lossy(&out.stdout);
    let dirty = stdout
        .lines()
        // porcelain v1 lines are `XY <path>` (rename: `XY <old> -> <new>`).
        .filter_map(|l| l.get(3..))
        .map(|p| p.trim().trim_matches('"').to_string())
        .filter(|p| !p.is_empty() && !p.starts_with(".stateful-spec/"))
        .collect();
    Ok(dirty)
}

/// The current git branch (`git rev-parse --abbrev-ref HEAD`); `"HEAD"` when
/// detached. Used by the `advance` branch guard.
pub fn git_current_branch(repo_root: &Path) -> Result<String, String> {
    let out = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .current_dir(repo_root)
        .output()
        .map_err(|e| format!("could not run git: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "git rev-parse failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

/// Whether `branch` is a repository default branch a milestone must never land
/// on. The set is configurable (`[git] default_branches`); the default is
/// `main` / `master`. Pure.
pub fn is_default_branch(branch: &str, defaults: &[String]) -> bool {
    defaults.iter().any(|d| d == branch)
}

/// Whether `name` is an auxiliary file of milestone `k` for `iteration` — its
/// spec draft (`<iter>-m<k>-spec.md`) or a review handoff
/// (`handoff-<iter>-m<k>-review*.md`). Pure (no IO). Used by the advance-time
/// milestone archiver.
pub fn is_milestone_aux(name: &str, iteration: &str, k: u64) -> bool {
    if iteration.is_empty() {
        return false;
    }
    let spec = format!("{iteration}-m{k}-spec.md");
    let handoff_prefix = format!("handoff-{iteration}-m{k}-review");
    name == spec || (name.starts_with(&handoff_prefix) && name.ends_with(".md"))
}

/// Archive milestone `k`'s auxiliaries (spec + review handoffs) into
/// `history/.archived/` via `git mv`. Idempotent — files already moved or absent
/// are skipped. Returns the moved file names. Auxiliaries are not in the History
/// Index, so no repointing is needed (that is the iteration-close job).
pub fn archive_milestone_aux(repo_root: &Path, iteration: &str, k: u64) -> Result<Vec<String>, String> {
    if iteration.is_empty() {
        return Ok(Vec::new());
    }
    let hist = repo_root.join(".stateful-spec").join("history");
    let archived = hist.join(".archived");
    let entries = match fs::read_dir(&hist) {
        Ok(e) => e,
        Err(_) => return Ok(Vec::new()),
    };
    let mut names: Vec<String> = entries
        .flatten()
        .filter(|e| e.path().is_file())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .filter(|n| is_milestone_aux(n, iteration, k))
        .collect();
    names.sort();
    if names.is_empty() {
        return Ok(Vec::new());
    }
    fs::create_dir_all(&archived).map_err(|e| format!("mkdir .archived: {e}"))?;
    let mut moved = Vec::new();
    for name in names {
        let src = format!(".stateful-spec/history/{name}");
        let dst = format!(".stateful-spec/history/.archived/{name}");
        let out = Command::new("git")
            .arg("mv")
            .arg(&src)
            .arg(&dst)
            .current_dir(repo_root)
            .output()
            .map_err(|e| format!("could not run git mv: {e}"))?;
        if out.status.success() {
            moved.push(name);
        } else {
            eprintln!(
                "flow advance: archive skip {name}: {}",
                String::from_utf8_lossy(&out.stderr).trim()
            );
        }
    }
    Ok(moved)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> Vec<String> {
        vec!["main".to_string(), "master".to_string()]
    }

    #[test]
    fn test_is_default_branch() {
        let d = defaults();
        assert!(is_default_branch("main", &d));
        assert!(is_default_branch("master", &d));
        assert!(!is_default_branch("feature/013-x", &d));
        assert!(!is_default_branch("chore/y", &d));
        assert!(!is_default_branch("HEAD", &d)); // detached: not a default-branch refusal
    }

    #[test]
    fn test_is_default_branch_configurable() {
        let d = vec!["trunk".to_string()];
        assert!(is_default_branch("trunk", &d));
        assert!(!is_default_branch("main", &d)); // not in the configured set
    }

    #[test]
    fn test_is_milestone_aux() {
        assert!(is_milestone_aux("013-x-m2-spec.md", "013-x", 2));
        assert!(is_milestone_aux("handoff-013-x-m2-review.md", "013-x", 2));
        assert!(is_milestone_aux("handoff-013-x-m2-review-r1.md", "013-x", 2));
        // not a match: different milestone, the umbrella central, or empty iteration.
        assert!(!is_milestone_aux("013-x-m3-spec.md", "013-x", 2));
        assert!(!is_milestone_aux("013-x.md", "013-x", 2));
        assert!(!is_milestone_aux("013-x-m2-spec.md", "", 2));
        assert!(!is_milestone_aux("handoff-013-x-m2-review.txt", "013-x", 2));
    }

    #[test]
    fn test_archive_milestone_aux_empty_iteration_is_noop() {
        let r = archive_milestone_aux(Path::new("."), "", 1).unwrap();
        assert!(r.is_empty());
    }
}
