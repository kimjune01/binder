use std::fs;

use binder_core::{
    Claim, DependencySnapshot, Evidence, EvidenceVerdict, ReceiptBundle, WarrantStatus,
    changed_artifacts, evaluate, snapshot_dependencies, validate_receipt,
};
use tempfile::tempdir;

fn claim() -> Claim {
    Claim {
        id: "failed-withdrawal-preserves-balances".into(),
        required_trials: vec!["rust-transition-proof".into(), "runtime-replay".into()],
    }
}

#[test]
fn changed_runtime_artifact_is_reported_stale() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("program.so"), "first").unwrap();
    let artifacts = snapshot_dependencies(dir.path(), &["program.so"])
        .unwrap()
        .files;
    let evidence = Evidence::new(
        "runtime-replay",
        EvidenceVerdict::Pass,
        DependencySnapshot {
            identity: "inputs".into(),
            files: Default::default(),
        },
    )
    .with_observation(Vec::new(), b"", b"", artifacts);

    assert!(
        changed_artifacts(dir.path(), &[evidence.clone()])
            .unwrap()
            .is_empty()
    );
    fs::write(dir.path().join("program.so"), "tampered").unwrap();
    assert_eq!(
        changed_artifacts(dir.path(), &[evidence]).unwrap(),
        vec!["program.so"]
    );
}

fn passing_evidence(snapshot: &DependencySnapshot) -> Vec<Evidence> {
    vec![
        Evidence::new(
            "rust-transition-proof",
            EvidenceVerdict::Pass,
            snapshot.clone(),
        ),
        Evidence::new("runtime-replay", EvidenceVerdict::Pass, snapshot.clone()),
    ]
}

#[test]
fn warrants_head_only_when_base_fails_and_every_required_trial_passes() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("transition.rs"), "fixed").unwrap();
    let snapshot = snapshot_dependencies(dir.path(), &["transition.rs"]).unwrap();

    let result = evaluate(
        &claim(),
        &snapshot,
        &[Evidence::new(
            "base-regression",
            EvidenceVerdict::ExpectedFail,
            snapshot.clone(),
        )],
        &passing_evidence(&snapshot),
    );

    assert_eq!(result.status, WarrantStatus::Warranted);
}

#[test]
fn fails_closed_when_required_evidence_is_missing_or_failed() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("transition.rs"), "fixed").unwrap();
    let snapshot = snapshot_dependencies(dir.path(), &["transition.rs"]).unwrap();

    let missing = evaluate(&claim(), &snapshot, &[], &passing_evidence(&snapshot)[..1]);
    assert_eq!(missing.status, WarrantStatus::Unsupported);

    let failed = vec![
        Evidence::new(
            "rust-transition-proof",
            EvidenceVerdict::Fail,
            snapshot.clone(),
        ),
        Evidence::new("runtime-replay", EvidenceVerdict::Pass, snapshot.clone()),
    ];
    assert_eq!(
        evaluate(&claim(), &snapshot, &[], &failed).status,
        WarrantStatus::Failed
    );
}

#[test]
fn dependency_change_makes_evidence_stale_but_unrelated_change_does_not() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("transition.rs"), "fixed").unwrap();
    fs::write(dir.path().join("notes.md"), "one").unwrap();
    let original = snapshot_dependencies(dir.path(), &["transition.rs"]).unwrap();
    let evidence = passing_evidence(&original);

    fs::write(dir.path().join("notes.md"), "two").unwrap();
    let unrelated = snapshot_dependencies(dir.path(), &["transition.rs"]).unwrap();
    assert_eq!(
        evaluate(&claim(), &unrelated, &[], &evidence).status,
        WarrantStatus::Warranted
    );

    fs::write(dir.path().join("transition.rs"), "changed").unwrap();
    let changed = snapshot_dependencies(dir.path(), &["transition.rs"]).unwrap();
    let result = evaluate(&claim(), &changed, &[], &evidence);
    assert_eq!(result.status, WarrantStatus::Stale);
    assert_eq!(result.changed_dependencies, vec!["transition.rs"]);
}

#[test]
fn snapshot_identity_is_deterministic_and_path_sensitive() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("a"), "same").unwrap();
    fs::write(dir.path().join("b"), "same").unwrap();

    let first = snapshot_dependencies(dir.path(), &["a", "b"]).unwrap();
    let second = snapshot_dependencies(dir.path(), &["b", "a"]).unwrap();
    let only_a = snapshot_dependencies(dir.path(), &["a"]).unwrap();
    let only_b = snapshot_dependencies(dir.path(), &["b"]).unwrap();

    assert_eq!(first.identity, second.identity);
    assert_ne!(only_a.identity, only_b.identity);
}

fn valid_receipt() -> ReceiptBundle {
    let snapshot = DependencySnapshot {
        identity: "inputs".into(),
        files: [("source.rs".into(), "digest".into())].into(),
    };
    let observed = |trial: &str, verdict| {
        Evidence::new(trial, verdict, snapshot.clone()).with_observation(
            vec!["trial".into()],
            b"stdout",
            b"stderr",
            Default::default(),
        )
    };
    ReceiptBundle {
        claim_id: claim().id,
        dependencies: snapshot.clone(),
        base: vec![
            observed("rust-transition-proof", EvidenceVerdict::ExpectedFail),
            observed("runtime-replay", EvidenceVerdict::ExpectedFail),
        ],
        head: vec![
            observed("rust-transition-proof", EvidenceVerdict::Pass),
            observed("runtime-replay", EvidenceVerdict::Pass),
        ],
        status: WarrantStatus::Warranted,
    }
}

#[test]
fn rejects_receipts_with_tampered_dependency_snapshots() {
    let mut receipt = valid_receipt();
    receipt.head[0].dependencies.identity = "forged".into();

    assert!(validate_receipt(&claim(), &receipt).is_err());
}

#[test]
fn rejects_receipts_with_missing_commands_or_duplicate_trials() {
    let mut missing_command = valid_receipt();
    missing_command.head[0].command.clear();
    assert!(validate_receipt(&claim(), &missing_command).is_err());

    let mut duplicate = valid_receipt();
    duplicate.head[1].trial_id = duplicate.head[0].trial_id.clone();
    assert!(validate_receipt(&claim(), &duplicate).is_err());
}

#[test]
fn rejects_receipts_whose_recorded_status_does_not_match_the_evidence() {
    let mut receipt = valid_receipt();
    receipt.head[0].verdict = EvidenceVerdict::Fail;

    assert!(validate_receipt(&claim(), &receipt).is_err());
}
