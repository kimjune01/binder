use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

use binder_core::{
    BINDER_VERSION, Evidence, EvidenceVerdict, RECEIPT_SCHEMA_VERSION, ReceiptBundle,
    WarrantStatus, changed_artifacts, evaluate, load_claim, receipt_identity, render_report,
    validate_receipt,
};

fn main() {
    if let Err(error) = run() {
        eprintln!("binder: {error}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), String> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let (command, claim_path, format) = match args.as_slice() {
        [command, claim_path] => (command, claim_path, OutputFormat::Text),
        [command, claim_path, flag, value] if flag == "--format" && value == "json" => {
            (command, claim_path, OutputFormat::Json)
        }
        _ => {
            return Err("usage: binder-cli <verify|status> <claim.yaml> [--format json]".into());
        }
    };

    let root = env::current_dir().map_err(|error| format!("find workspace root: {error}"))?;
    let loaded = load_claim(&root, Path::new(claim_path))?;
    match command.as_str() {
        "verify" => verify(&root, loaded, format),
        "status" => status(&root, loaded, format),
        _ => Err(format!("unknown command: {command}")),
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum OutputFormat {
    Text,
    Json,
}

fn verify(
    root: &Path,
    loaded: binder_core::LoadedClaim,
    format: OutputFormat,
) -> Result<(), String> {
    let mut base = Vec::new();
    let mut head = Vec::new();
    let mut raw_outputs = Vec::new();

    for trial in &loaded.trials {
        let base_result = execute(root, trial, "vulnerable", format)?;
        raw_outputs.push((base_result.stdout.clone(), base_result.stderr.clone()));
        base.push(
            Evidence::new(
                &trial.id,
                if base_result.passed {
                    EvidenceVerdict::Fail
                } else {
                    EvidenceVerdict::ExpectedFail
                },
                loaded.snapshot.clone(),
            )
            .with_observation(
                base_result.command,
                &base_result.stdout,
                &base_result.stderr,
                base_result.artifacts,
            ),
        );

        let head_result = execute(root, trial, "fixed", format)?;
        raw_outputs.push((head_result.stdout.clone(), head_result.stderr.clone()));
        head.push(
            Evidence::new(
                &trial.id,
                if head_result.passed {
                    EvidenceVerdict::Pass
                } else {
                    EvidenceVerdict::Fail
                },
                loaded.snapshot.clone(),
            )
            .with_observation(
                head_result.command,
                &head_result.stdout,
                &head_result.stderr,
                head_result.artifacts,
            ),
        );
    }

    let warrant = evaluate(&loaded.claim, &loaded.snapshot, &base, &head);
    let bundle = ReceiptBundle {
        schema_version: RECEIPT_SCHEMA_VERSION,
        binder_version: BINDER_VERSION.into(),
        claim_id: loaded.claim.id.clone(),
        dependencies: loaded.snapshot.clone(),
        base: base.clone(),
        head: head.clone(),
        status: warrant.status,
    };
    let receipt_digest = receipt_identity(&bundle)?;
    let receipt_dir = root.join(".binder/receipts").join(&receipt_digest);
    fs::create_dir_all(&receipt_dir)
        .map_err(|error| format!("create {}: {error}", receipt_dir.display()))?;
    let receipt = serde_yaml::to_string(&bundle)
        .map_err(|error| format!("serialize receipt bundle: {error}"))?;
    fs::write(receipt_dir.join("receipt.yaml"), receipt)
        .map_err(|error| format!("write receipt bundle: {error}"))?;
    package_replay_bundle(root, &receipt_dir, &loaded.dependency_paths, &raw_outputs)?;
    let claim_dir = root.join(".binder/claims");
    fs::create_dir_all(&claim_dir)
        .map_err(|error| format!("create {}: {error}", claim_dir.display()))?;
    fs::write(
        claim_dir.join(&loaded.claim.id),
        format!("{receipt_digest}\n"),
    )
    .map_err(|error| format!("write latest claim receipt pointer: {error}"))?;

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
    write_github_summary(warrant.status, &report)?;
    emit_output(
        format,
        "verify",
        &loaded.claim,
        warrant.status,
        Some(&receipt_digest),
        &report,
    )?;

    if !matches!(warrant.status, binder_core::WarrantStatus::Warranted) {
        std::process::exit(1);
    }
    Ok(())
}

fn package_replay_bundle(
    root: &Path,
    receipt_dir: &Path,
    dependency_paths: &[String],
    raw_outputs: &[(Vec<u8>, Vec<u8>)],
) -> Result<(), String> {
    for relative in dependency_paths {
        let destination = receipt_dir.join("inputs").join(relative);
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| format!("create {}: {error}", parent.display()))?;
        }
        fs::copy(root.join(relative), &destination)
            .map_err(|error| format!("package {relative}: {error}"))?;
    }

    let outputs = receipt_dir.join("outputs");
    fs::create_dir_all(&outputs)
        .map_err(|error| format!("create {}: {error}", outputs.display()))?;
    for (pair_index, pair) in raw_outputs.chunks_exact(2).enumerate() {
        for (revision, trial) in [("base", &pair[0]), ("head", &pair[1])] {
            fs::write(
                outputs.join(format!("{revision}-{pair_index}.stdout")),
                &trial.0,
            )
            .map_err(|error| format!("write replay stdout: {error}"))?;
            fs::write(
                outputs.join(format!("{revision}-{pair_index}.stderr")),
                &trial.1,
            )
            .map_err(|error| format!("write replay stderr: {error}"))?;
        }
    }
    fs::write(
        receipt_dir.join("REPLAY.md"),
        "# Binder replay bundle\n\nThe `inputs/` tree contains every declared input. Raw captured trial streams are in `outputs/`; their SHA-256 digests are recorded in `receipt.yaml`. External crates are pinned by `inputs/Cargo.lock`; `cargo-build-sbf` is pinned by `inputs/scripts/replay.sh`. To reconstruct and verify from this bundle, run:\n\n```sh\ncd inputs\nbash scripts/replay.sh\n```\n",
    )
    .map_err(|error| format!("write replay instructions: {error}"))?;
    Ok(())
}

fn status(
    root: &Path,
    loaded: binder_core::LoadedClaim,
    format: OutputFormat,
) -> Result<(), String> {
    let receipt_path = root.join(".binder/claims").join(&loaded.claim.id);
    let pointer = fs::read_to_string(&receipt_path).map_err(|error| {
        format!(
            "no recorded warrant for {} ({}): {error}",
            loaded.claim.id,
            receipt_path.display()
        )
    })?;
    let receipt_digest = pointer.trim();
    if receipt_digest.len() != 64 || !receipt_digest.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err("legacy or malformed receipt pointer; rerun binder verify".into());
    }
    let canonical_path = root
        .join(".binder/receipts")
        .join(receipt_digest)
        .join("receipt.yaml");
    let bytes = fs::read(&canonical_path).map_err(|error| {
        format!(
            "receipt pointer has no content-addressed bundle ({}): {error}",
            canonical_path.display()
        )
    })?;
    let bundle: ReceiptBundle = serde_yaml::from_slice(&bytes)
        .map_err(|error| format!("parse {}: {error}", canonical_path.display()))?;
    validate_receipt(&loaded.claim, &bundle)?;
    if receipt_identity(&bundle)? != receipt_digest {
        return Err("receipt contents do not match their content address".into());
    }
    let mut warrant = evaluate(&loaded.claim, &loaded.snapshot, &bundle.base, &bundle.head);
    let artifacts = bundle
        .base
        .iter()
        .chain(&bundle.head)
        .cloned()
        .collect::<Vec<_>>();
    let changed = changed_artifacts(root, &artifacts)
        .map_err(|error| format!("validate receipt artifacts: {error}"))?;
    if !changed.is_empty() {
        warrant.status = WarrantStatus::Stale;
        warrant.changed_dependencies.extend(changed);
        warrant.changed_dependencies.sort();
        warrant.changed_dependencies.dedup();
    }
    let report = render_report(&loaded.claim, &warrant, &bundle.head);
    write_github_summary(warrant.status, &report)?;
    emit_output(
        format,
        "status",
        &loaded.claim,
        warrant.status,
        Some(receipt_digest),
        &report,
    )?;
    if !matches!(warrant.status, binder_core::WarrantStatus::Warranted) {
        std::process::exit(1);
    }
    Ok(())
}

fn emit_output(
    format: OutputFormat,
    command: &str,
    claim: &binder_core::Claim,
    status: WarrantStatus,
    receipt_digest: Option<&str>,
    report: &str,
) -> Result<(), String> {
    match format {
        OutputFormat::Text => print!("{report}"),
        OutputFormat::Json => {
            let document = serde_json::json!({
                "schema_version": 1,
                "binder_version": BINDER_VERSION,
                "command": command,
                "claim_id": claim.id,
                "statement": claim.statement,
                "status": format!("{status:?}").to_ascii_uppercase(),
                "receipt_digest": receipt_digest,
            });
            println!(
                "{}",
                serde_json::to_string(&document)
                    .map_err(|error| format!("serialize JSON output: {error}"))?
            );
        }
    }
    Ok(())
}

fn write_github_summary(status: WarrantStatus, report: &str) -> Result<(), String> {
    let Ok(path) = env::var("GITHUB_STEP_SUMMARY") else {
        return Ok(());
    };
    let label = match status {
        WarrantStatus::Warranted => "WARRANTED",
        WarrantStatus::Failed => "FAILED",
        WarrantStatus::Stale => "STALE",
        WarrantStatus::Unsupported => "UNSUPPORTED",
    };
    let mut summary = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|error| format!("open GitHub step summary {path}: {error}"))?;
    writeln!(summary, "## Binder: {label}\n\n```text\n{report}```\n")
        .map_err(|error| format!("write GitHub step summary {path}: {error}"))
}

struct Execution {
    passed: bool,
    command: Vec<String>,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
    artifacts: BTreeMap<String, String>,
}

fn execute(
    root: &Path,
    trial: &binder_core::Trial,
    revision: &str,
    format: OutputFormat,
) -> Result<Execution, String> {
    let args = trial
        .command
        .iter()
        .map(|arg| arg.replace("{revision}", revision))
        .collect::<Vec<_>>();
    let (program, arguments) = args
        .split_first()
        .ok_or_else(|| "trial command is empty".to_owned())?;
    let output = Command::new(program)
        .args(arguments)
        .current_dir(root)
        .output()
        .map_err(|error| format!("execute {program}: {error}"))?;
    if format == OutputFormat::Text {
        io::stdout()
            .write_all(&output.stdout)
            .map_err(|error| format!("write trial stdout: {error}"))?;
    } else {
        io::stderr()
            .write_all(&output.stdout)
            .map_err(|error| format!("write trial stdout diagnostics: {error}"))?;
    }
    io::stderr()
        .write_all(&output.stderr)
        .map_err(|error| format!("write trial stderr: {error}"))?;
    let artifact_paths = trial
        .artifacts
        .iter()
        .map(|path| path.replace("{revision}", revision))
        .collect::<Vec<_>>();
    let path_refs = artifact_paths
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    let artifacts = binder_core::snapshot_dependencies(root, &path_refs)
        .map_err(|error| format!("snapshot trial artifacts: {error}"))?
        .files;
    Ok(Execution {
        passed: output.status.success(),
        command: args,
        stdout: output.stdout,
        stderr: output.stderr,
        artifacts,
    })
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
