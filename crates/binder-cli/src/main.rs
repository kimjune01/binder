use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

use binder_core::{Evidence, EvidenceVerdict, ReceiptBundle, evaluate, load_claim, render_report};

fn main() {
    if let Err(error) = run() {
        eprintln!("binder: {error}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), String> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let [command, claim_path] = args.as_slice() else {
        return Err("usage: binder-cli <verify|status> <claim.yaml>".into());
    };

    let root = env::current_dir().map_err(|error| format!("find workspace root: {error}"))?;
    let loaded = load_claim(&root, Path::new(claim_path))?;
    match command.as_str() {
        "verify" => verify(&root, loaded),
        "status" => status(&root, loaded),
        _ => Err(format!("unknown command: {command}")),
    }
}

fn verify(root: &Path, loaded: binder_core::LoadedClaim) -> Result<(), String> {
    let mut base = Vec::new();
    let mut head = Vec::new();

    for trial in &loaded.trials {
        let base_passed = execute(root, &trial.command, "vulnerable")?;
        base.push(Evidence::new(
            &trial.id,
            if base_passed {
                EvidenceVerdict::Fail
            } else {
                EvidenceVerdict::ExpectedFail
            },
            loaded.snapshot.clone(),
        ));

        let head_passed = execute(root, &trial.command, "fixed")?;
        head.push(Evidence::new(
            &trial.id,
            if head_passed {
                EvidenceVerdict::Pass
            } else {
                EvidenceVerdict::Fail
            },
            loaded.snapshot.clone(),
        ));
    }

    let warrant = evaluate(&loaded.claim, &loaded.snapshot, &base, &head);
    let bundle = ReceiptBundle {
        claim_id: loaded.claim.id.clone(),
        dependencies: loaded.snapshot.clone(),
        base: base.clone(),
        head: head.clone(),
        status: warrant.status,
    };
    let receipt_dir = root
        .join(".binder/receipts")
        .join(&loaded.snapshot.identity);
    fs::create_dir_all(&receipt_dir)
        .map_err(|error| format!("create {}: {error}", receipt_dir.display()))?;
    let receipt = serde_yaml::to_string(&bundle)
        .map_err(|error| format!("serialize receipt bundle: {error}"))?;
    fs::write(receipt_dir.join("receipt.yaml"), receipt)
        .map_err(|error| format!("write receipt bundle: {error}"))?;
    let claim_dir = root.join(".binder/claims");
    fs::create_dir_all(&claim_dir)
        .map_err(|error| format!("create {}: {error}", claim_dir.display()))?;
    let latest = serde_yaml::to_string(&bundle)
        .map_err(|error| format!("serialize latest receipt: {error}"))?;
    fs::write(claim_dir.join(format!("{}.yaml", loaded.claim.id)), latest)
        .map_err(|error| format!("write latest claim receipt: {error}"))?;

    let mut report = render_report(&loaded.claim, &warrant, &head);
    report.insert_str(
        report.find("\nEvidence").unwrap_or(report.len()),
        &format!(
            "\nBase  {}\nHead  {}\n",
            aggregate_label(&base, EvidenceVerdict::ExpectedFail),
            aggregate_label(&head, EvidenceVerdict::Pass),
        ),
    );
    report.push_str(&format!(
        "\nReplay bundle  {}\n",
        receipt_dir
            .strip_prefix(root)
            .unwrap_or(&receipt_dir)
            .display()
    ));
    print!("{report}");

    if !matches!(warrant.status, binder_core::WarrantStatus::Warranted) {
        std::process::exit(1);
    }
    Ok(())
}

fn status(root: &Path, loaded: binder_core::LoadedClaim) -> Result<(), String> {
    let receipt_path = root
        .join(".binder/claims")
        .join(format!("{}.yaml", loaded.claim.id));
    let bytes = fs::read(&receipt_path).map_err(|error| {
        format!(
            "no recorded warrant for {} ({}): {error}",
            loaded.claim.id,
            receipt_path.display()
        )
    })?;
    let bundle: ReceiptBundle = serde_yaml::from_slice(&bytes)
        .map_err(|error| format!("parse {}: {error}", receipt_path.display()))?;
    if bundle.claim_id != loaded.claim.id {
        return Err("receipt claim id does not match requested claim".into());
    }
    let warrant = evaluate(&loaded.claim, &loaded.snapshot, &bundle.base, &bundle.head);
    let report = render_report(&loaded.claim, &warrant, &bundle.head);
    print!("{report}");
    if !matches!(warrant.status, binder_core::WarrantStatus::Warranted) {
        std::process::exit(1);
    }
    Ok(())
}

fn execute(root: &Path, command: &[String], revision: &str) -> Result<bool, String> {
    let args = command
        .iter()
        .map(|arg| arg.replace("{revision}", revision))
        .collect::<Vec<_>>();
    let (program, arguments) = args
        .split_first()
        .ok_or_else(|| "trial command is empty".to_owned())?;
    let status = Command::new(program)
        .args(arguments)
        .current_dir(root)
        .status()
        .map_err(|error| format!("execute {program}: {error}"))?;
    Ok(status.success())
}

fn aggregate_label(evidence: &[Evidence], expected: EvidenceVerdict) -> &'static str {
    if evidence.iter().all(|item| item.verdict == expected) {
        match expected {
            EvidenceVerdict::ExpectedFail => "FAIL (expected; bug reproduced)",
            EvidenceVerdict::Pass => "PASS",
            EvidenceVerdict::Fail => "FAIL",
        }
    } else {
        "UNEXPECTED"
    }
}
