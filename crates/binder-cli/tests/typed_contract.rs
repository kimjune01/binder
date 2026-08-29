use std::fs;
use std::process::{Command, Output};

use serde_json::Value;
use tempfile::TempDir;

fn git(root: &std::path::Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git {}: {}",
        args.join(" "),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap().trim().into()
}

fn fixture(check: &str) -> (TempDir, String, String) {
    let dir = tempfile::tempdir().unwrap();
    git(dir.path(), &["init", "-q"]);
    git(dir.path(), &["config", "user.email", "binder@example.com"]);
    git(dir.path(), &["config", "user.name", "Binder Test"]);

    fs::write(dir.path().join("behavior.txt"), "buggy\n").unwrap();
    git(dir.path(), &["add", "behavior.txt"]);
    git(dir.path(), &["commit", "-qm", "buggy base"]);
    let base = git(dir.path(), &["rev-parse", "HEAD"]);

    fs::write(dir.path().join("behavior.txt"), "fixed\n").unwrap();
    fs::write(dir.path().join("check.sh"), check).unwrap();
    fs::write(
        dir.path().join("claim.yaml"),
        "version: 2\nid: behavior-is-fixed\nclaim: Behavior is fixed.\ndependencies: [behavior.txt, check.sh, claim.yaml]\nrequired_trials: [behavior]\ntrials:\n  - id: behavior\n    evidence_kind: empirical\n    command: [bash, check.sh]\n    overlay_from_head: [check.sh]\n",
    )
    .unwrap();
    git(dir.path(), &["add", "."]);
    git(dir.path(), &["commit", "-qm", "candidate fix and check"]);
    let head = git(dir.path(), &["rev-parse", "HEAD"]);
    (dir, base, head)
}

fn verify(root: &std::path::Path, base: &str, head: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_binder-cli"))
        .args([
            "verify",
            "claim.yaml",
            "--base",
            base,
            "--head",
            head,
            "--format",
            "json",
        ])
        .current_dir(root)
        .output()
        .unwrap()
}

fn document(output: &Output) -> Value {
    serde_json::from_slice(&output.stdout).unwrap_or_else(|error| {
        panic!(
            "invalid JSON: {error}\nstdout: {}\nstderr: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
    })
}

#[test]
fn sensitive_check_refutes_base_and_stands_on_head() {
    let check = r#"#!/usr/bin/env bash
set -euo pipefail
if grep -q '^fixed$' behavior.txt; then
  printf '%s\n' '{"observation":"stood","witness":{"actual":"fixed"}}'
else
  printf '%s\n' '{"observation":"refuted","witness":{"actual":"buggy"}}'
fi
"#;
    let (dir, base, head) = fixture(check);

    let output = verify(dir.path(), &base, &head);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let json = document(&output);
    assert_eq!(json["subject"]["base"], base);
    assert_eq!(json["subject"]["head"], head);
    assert_eq!(json["schema_version"], 2);
    assert_eq!(json["trials"][0]["evidence_kind"], "empirical");
    assert_eq!(json["trials"][0]["base"]["execution"], "completed");
    assert_eq!(json["trials"][0]["base"]["observation"], "refuted");
    assert_eq!(json["trials"][0]["head"]["observation"], "stood");
    assert_eq!(json["trials"][0]["head"]["witness"]["actual"], "fixed");
    assert_eq!(json["freshness"], "current");
    assert_eq!(json["policy"], "warranted");
    assert_eq!(json["receipt_digest"].as_str().unwrap().len(), 64);
}

#[test]
fn insensitive_check_does_not_warrant_the_change() {
    let check = "#!/usr/bin/env bash\nprintf '%s\\n' '{\"observation\":\"stood\",\"witness\":{\"actual\":\"always\"}}'\n";
    let (dir, base, head) = fixture(check);

    let output = verify(dir.path(), &base, &head);
    assert_eq!(output.status.code(), Some(1));
    let json = document(&output);
    assert_eq!(json["trials"][0]["base"]["observation"], "stood");
    assert_eq!(json["trials"][0]["head"]["observation"], "stood");
    assert_eq!(json["policy"], "not-warranted");
}

#[test]
fn broken_check_produces_no_epistemic_verdict() {
    let check = "#!/usr/bin/env bash\necho 'compiler unavailable' >&2\nexit 3\n";
    let (dir, base, head) = fixture(check);

    let output = verify(dir.path(), &base, &head);
    assert_eq!(output.status.code(), Some(1));
    let json = document(&output);
    assert_eq!(json["trials"][0]["base"]["execution"], "error");
    assert_eq!(json["trials"][0]["base"]["observation"], "no-verdict");
    assert_eq!(json["trials"][0]["head"]["execution"], "error");
    assert_eq!(json["trials"][0]["head"]["observation"], "no-verdict");
    assert_eq!(json["policy"], "not-warranted");
}

#[test]
fn malformed_success_is_not_inferred_to_have_stood() {
    let (dir, base, head) = fixture("#!/usr/bin/env bash\necho 'looks good'\n");
    let output = verify(dir.path(), &base, &head);
    assert_eq!(output.status.code(), Some(1));
    let json = document(&output);
    assert_eq!(json["trials"][0]["base"]["execution"], "completed");
    assert_eq!(json["trials"][0]["base"]["observation"], "no-verdict");
    assert_eq!(json["policy"], "not-warranted");
}

#[test]
fn observation_without_a_structured_witness_is_no_verdict() {
    let (dir, base, head) =
        fixture("#!/usr/bin/env bash\nprintf '%s\\n' '{\"observation\":\"stood\"}'\n");
    let output = verify(dir.path(), &base, &head);
    assert_eq!(output.status.code(), Some(1));
    let json = document(&output);
    assert_eq!(json["trials"][0]["base"]["execution"], "completed");
    assert_eq!(json["trials"][0]["base"]["observation"], "no-verdict");
    assert_eq!(json["policy"], "not-warranted");
}
