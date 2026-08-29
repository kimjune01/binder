use std::fs;
use std::process::Command;

use binder_core::{ReceiptBundle, receipt_identity};

#[test]
fn verify_packages_inputs_and_raw_trial_outputs() {
    let dir = tempfile::tempdir().unwrap();
    let summary = dir.path().join("step-summary.md");
    fs::write(dir.path().join("input.txt"), "declared input").unwrap();
    fs::write(
        dir.path().join("claim.yaml"),
        "version: 1\nid: portable-demo\nclaim: fixed passes\ndependencies: [claim.yaml, input.txt]\nrequired_trials: [trial]\ntrials:\n  - id: trial\n    command: [sh, trial.sh, \"{revision}\"]\n",
    )
    .unwrap();
    fs::write(dir.path().join("trial.sh"), "test \"$1\" = fixed\n").unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_binder-cli"))
        .args(["verify", "claim.yaml"])
        .env("GITHUB_STEP_SUMMARY", &summary)
        .current_dir(dir.path())
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let receipts = dir.path().join(".binder/receipts");
    let bundle = fs::read_dir(receipts)
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    let receipt: ReceiptBundle =
        serde_yaml::from_slice(&fs::read(bundle.join("receipt.yaml")).unwrap()).unwrap();
    let digest = receipt_identity(&receipt).unwrap();
    assert_eq!(bundle.file_name().unwrap(), digest.as_str());
    assert_eq!(
        fs::read_to_string(dir.path().join(".binder/claims/portable-demo")).unwrap(),
        format!("{digest}\n")
    );
    assert_eq!(
        fs::read_to_string(bundle.join("inputs/input.txt")).unwrap(),
        "declared input"
    );
    assert!(bundle.join("outputs/base-0.stdout").is_file());
    assert!(bundle.join("outputs/base-0.stderr").is_file());
    assert!(bundle.join("outputs/head-0.stdout").is_file());
    assert!(bundle.join("outputs/head-0.stderr").is_file());
    let replay = fs::read_to_string(bundle.join("REPLAY.md")).unwrap();
    assert!(replay.contains("cd inputs"));
    assert!(replay.contains("bash scripts/replay.sh"));

    let summary = fs::read_to_string(summary).unwrap();
    assert!(summary.contains("## Binder: WARRANTED"));
    assert!(summary.contains("fixed passes"));
    assert!(summary.contains("Base  FAIL (expected; bug reproduced)"));
    assert!(summary.contains("Head  PASS"));
}
