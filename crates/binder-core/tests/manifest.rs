use std::fs;

use binder_core::{Evidence, EvidenceVerdict, WarrantStatus, evaluate, load_claim, render_report};

#[test]
fn parses_the_demo_claim_and_renders_a_reviewer_sized_warrant() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let manifest = load_claim(
        &root,
        &root.join("demo/claims/failed-withdrawal-preserves-balances.yaml"),
    )
    .unwrap();
    assert_eq!(manifest.claim.id, "failed-withdrawal-preserves-balances");
    assert_eq!(
        manifest.claim.statement,
        "An unsuccessful withdrawal cannot reduce the vault balance or increase the recipient balance."
    );
    assert!(
        manifest
            .dependency_paths
            .contains(&"rust-toolchain.toml".to_owned())
    );
    assert_eq!(manifest.trials.len(), 2);
    assert_eq!(
        manifest.trials[1].artifacts,
        vec!["target/deploy/{revision}/vault_program.so"]
    );

    let evidence = manifest
        .claim
        .required_trials
        .iter()
        .map(|trial| Evidence::new(trial, EvidenceVerdict::Pass, manifest.snapshot.clone()))
        .collect::<Vec<_>>();
    let warrant = evaluate(&manifest.claim, &manifest.snapshot, &[], &evidence);
    let report = render_report(&manifest.claim, &warrant, &evidence);

    assert_eq!(warrant.status, WarrantStatus::Warranted);
    assert!(report.starts_with("WARRANTED  failed-withdrawal-preserves-balances"));
    assert!(report.contains(
        "An unsuccessful withdrawal cannot reduce the vault balance or increase the recipient balance."
    ));
    assert!(report.contains("PASS  rust-transition-proof"));
    assert!(report.contains("PASS  runtime-replay"));
    assert!(!report.contains(&manifest.snapshot.identity));
    assert!(report.contains(&manifest.snapshot.identity[..12]));
}

#[test]
fn rejects_unknown_manifest_fields() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("claim.yaml");
    fs::write(
        &path,
        "version: 1\nid: x\nclaim: x\ndependencies: []\nrequired_trials: []\ntrials: []\nsurprise: true\n",
    )
    .unwrap();

    assert!(load_claim(dir.path(), &path).is_err());
}

#[test]
fn rejects_claim_ids_that_can_escape_the_receipt_directory() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("claim.yaml");
    fs::write(
        &path,
        "version: 1\nid: ../../outside\nclaim: x\ndependencies: []\nrequired_trials: [x]\ntrials:\n  - id: x\n    command: [true]\n",
    )
    .unwrap();

    assert!(load_claim(dir.path(), &path).is_err());
}

#[test]
fn rejects_v2_trials_without_an_evidence_kind() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("claim.yaml");
    fs::write(
        &path,
        "version: 2\nid: x\nclaim: x\nentitlement: {authored_by: maintainer, base: refuted, head: stood}\ndependencies: [claim.yaml]\nrequired_trials: [x]\ntrials:\n  - id: x\n    command: [true]\n",
    )
    .unwrap();

    assert!(load_claim(dir.path(), &path).is_err());
}

#[test]
fn rejects_overlay_paths_that_escape_the_worktree() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("claim.yaml");
    fs::write(
        &path,
        "version: 2\nid: x\nclaim: x\nentitlement: {authored_by: maintainer, base: refuted, head: stood}\ndependencies: [claim.yaml]\nrequired_trials: [x]\ntrials:\n  - id: x\n    evidence_kind: empirical\n    command: [true]\n    overlay_from_head: [../outside]\n",
    )
    .unwrap();

    assert!(load_claim(dir.path(), &path).is_err());
}

#[test]
fn rejects_v2_claims_without_an_authored_entitlement_rule() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("claim.yaml");
    fs::write(
        &path,
        "version: 2\nid: x\nclaim: x\ndependencies: [claim.yaml]\nrequired_trials: [x]\ntrials:\n  - id: x\n    evidence_kind: empirical\n    command: [true]\n",
    )
    .unwrap();

    assert!(load_claim(dir.path(), &path).is_err());
}
