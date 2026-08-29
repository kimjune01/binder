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
    assert!(
        manifest
            .dependency_paths
            .contains(&"rust-toolchain.toml".to_owned())
    );
    assert_eq!(manifest.trials.len(), 2);

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
