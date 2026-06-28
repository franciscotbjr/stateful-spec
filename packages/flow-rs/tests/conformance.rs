//! Conformance suite — drives the compiled `flow` binary against the shared
//! `../conformance/cases/*.txt` fixtures and asserts the exit code + resulting
//! frontmatter after each step. The Node twin runs the SAME fixtures; together
//! they guarantee Rust ≡ TS behaviour. Zero-dependency line-based parser.

use std::path::{Path, PathBuf};
use std::process::Command;

fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_flow")
}

struct Step {
    run: String,
    exit: i32,
    expect: Vec<(String, String)>,
}

fn parse_case(text: &str) -> (Vec<(String, String)>, Vec<Step>) {
    let mut initial: Vec<(String, String)> = Vec::new();
    let mut steps: Vec<Step> = Vec::new();
    let mut section = "";
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line == "[initial]" {
            section = "initial";
            continue;
        }
        if line == "[step]" {
            steps.push(Step {
                run: String::new(),
                exit: 0,
                expect: Vec::new(),
            });
            section = "step";
            continue;
        }
        let (k, v) = match line.split_once('=') {
            Some((k, v)) => (k.trim(), v.trim()),
            None => continue,
        };
        if section == "initial" {
            initial.push((k.to_string(), v.to_string()));
        } else if section == "step" {
            let s = steps.last_mut().unwrap();
            match k {
                "run" => s.run = v.to_string(),
                "exit" => s.exit = v.parse().unwrap_or(0),
                "expect" => {
                    for pair in v.split_whitespace() {
                        if let Some((pk, pv)) = pair.split_once('=') {
                            s.expect.push((pk.to_string(), pv.to_string()));
                        }
                    }
                }
                _ => {}
            }
        }
    }
    (initial, steps)
}

fn write_state(dir: &Path, initial: &[(String, String)]) -> PathBuf {
    let ss = dir.join(".stateful-spec");
    std::fs::create_dir_all(&ss).unwrap();
    let p = ss.join("flow-state.md");
    let mut fm = String::from("---\n");
    for (k, v) in initial {
        fm.push_str(&format!("{k}: {v}\n"));
    }
    fm.push_str("---\n\n# Turn-log\n");
    std::fs::write(&p, fm).unwrap();
    p
}

fn field(flow: &Path, key: &str) -> String {
    let content = std::fs::read_to_string(flow).unwrap();
    let prefix = format!("{key}:");
    for line in content.lines() {
        let t = line.trim();
        if t.starts_with(&prefix) {
            return t[prefix.len()..].trim().to_string();
        }
    }
    String::new()
}

#[test]
fn conformance_cases_pass() {
    let cases_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("conformance")
        .join("cases");
    let mut files: Vec<PathBuf> = std::fs::read_dir(&cases_dir)
        .expect("conformance cases dir")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().map(|e| e == "txt").unwrap_or(false))
        .collect();
    files.sort();
    assert!(!files.is_empty(), "no conformance cases in {}", cases_dir.display());

    for file in files {
        let name = file.file_name().unwrap().to_string_lossy().to_string();
        let text = std::fs::read_to_string(&file).unwrap();
        let (initial, steps) = parse_case(&text);
        let tmp = std::env::temp_dir().join(format!(
            "flow-conf-{}-{}",
            std::process::id(),
            name.replace('.', "_")
        ));
        let _ = std::fs::remove_dir_all(&tmp);
        let flow = write_state(&tmp, &initial);

        for (i, step) in steps.iter().enumerate() {
            let parts: Vec<&str> = step.run.split_whitespace().collect();
            let mut cmd = Command::new(bin());
            cmd.arg(parts[0]).arg("--flow-file").arg(&flow);
            for a in &parts[1..] {
                cmd.arg(a);
            }
            let out = cmd.output().unwrap();
            let code = out.status.code().unwrap_or(-1);
            assert_eq!(
                code,
                step.exit,
                "[{name}] step {i} '{}' exit: got {code} want {} (stderr: {})",
                step.run,
                step.exit,
                String::from_utf8_lossy(&out.stderr)
            );
            for (k, v) in &step.expect {
                let got = field(&flow, k);
                assert_eq!(&got, v, "[{name}] step {i} '{}' field {k}", step.run);
            }
        }
        let _ = std::fs::remove_dir_all(&tmp);
    }
}
