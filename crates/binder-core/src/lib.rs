//! Binder's local warrant evaluator.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs;
use std::io;
use std::path::{Component, Path};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Claim {
    pub id: String,
    pub statement: String,
    pub required_trials: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoadedClaim {
    pub claim: Claim,
    pub dependency_paths: Vec<String>,
    pub snapshot: DependencySnapshot,
    pub trials: Vec<Trial>,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Trial {
    pub id: String,
    pub command: Vec<String>,
    #[serde(default)]
    pub artifacts: Vec<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ClaimFile {
    version: u32,
    id: String,
    claim: String,
    dependencies: Vec<String>,
    required_trials: Vec<String>,
    trials: Vec<Trial>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DependencySnapshot {
    pub identity: String,
    pub files: BTreeMap<String, String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EvidenceVerdict {
    Pass,
    Fail,
    ExpectedFail,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Evidence {
    pub trial_id: String,
    pub verdict: EvidenceVerdict,
    pub dependencies: DependencySnapshot,
    #[serde(default)]
    pub command: Vec<String>,
    #[serde(default)]
    pub stdout_sha256: String,
    #[serde(default)]
    pub stderr_sha256: String,
    #[serde(default)]
    pub artifacts: BTreeMap<String, String>,
    #[serde(default)]
    pub observation: String,
}

impl Evidence {
    pub fn new(
        trial_id: impl Into<String>,
        verdict: EvidenceVerdict,
        dependencies: DependencySnapshot,
    ) -> Self {
        Self {
            trial_id: trial_id.into(),
            verdict,
            dependencies,
            command: Vec::new(),
            stdout_sha256: String::new(),
            stderr_sha256: String::new(),
            artifacts: BTreeMap::new(),
            observation: String::new(),
        }
    }

    pub fn with_observation(
        mut self,
        command: Vec<String>,
        stdout: &[u8],
        stderr: &[u8],
        artifacts: BTreeMap<String, String>,
    ) -> Self {
        self.command = command;
        self.stdout_sha256 = hex_digest(stdout);
        self.stderr_sha256 = hex_digest(stderr);
        self.artifacts = artifacts;
        self.observation = first_observation(stderr)
            .or_else(|| first_observation(stdout))
            .unwrap_or_default();
        self
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WarrantStatus {
    Warranted,
    Failed,
    Stale,
    Unsupported,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReceiptBundle {
    pub claim_id: String,
    pub dependencies: DependencySnapshot,
    pub base: Vec<Evidence>,
    pub head: Vec<Evidence>,
    pub status: WarrantStatus,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Warrant {
    pub status: WarrantStatus,
    pub changed_dependencies: Vec<String>,
    pub missing_trials: Vec<String>,
}

pub fn validate_receipt(claim: &Claim, bundle: &ReceiptBundle) -> Result<(), String> {
    if bundle.claim_id != claim.id {
        return Err("receipt claim id does not match requested claim".into());
    }
    validate_evidence_set(claim, &bundle.dependencies, &bundle.base, "base")?;
    validate_evidence_set(claim, &bundle.dependencies, &bundle.head, "head")?;

    let evaluated = evaluate(claim, &bundle.dependencies, &bundle.base, &bundle.head);
    if evaluated.status != bundle.status {
        return Err("receipt status does not match its evidence".into());
    }
    Ok(())
}

fn validate_evidence_set(
    claim: &Claim,
    dependencies: &DependencySnapshot,
    evidence: &[Evidence],
    revision: &str,
) -> Result<(), String> {
    let mut seen = BTreeSet::new();
    for item in evidence {
        if !seen.insert(item.trial_id.as_str()) {
            return Err(format!("duplicate {revision} trial: {}", item.trial_id));
        }
        if item.dependencies != *dependencies {
            return Err(format!(
                "{} {revision} evidence has a mismatched dependency snapshot",
                item.trial_id
            ));
        }
        if item.command.is_empty() || item.command.iter().any(String::is_empty) {
            return Err(format!(
                "{} {revision} evidence has no command",
                item.trial_id
            ));
        }
        if !is_sha256(&item.stdout_sha256) || !is_sha256(&item.stderr_sha256) {
            return Err(format!(
                "{} {revision} evidence has a malformed output digest",
                item.trial_id
            ));
        }
        if item.artifacts.values().any(|digest| !is_sha256(digest)) {
            return Err(format!(
                "{} {revision} evidence has a malformed artifact digest",
                item.trial_id
            ));
        }
    }
    let expected = claim
        .required_trials
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    if seen != expected {
        return Err(format!(
            "receipt {revision} trials do not exactly match the claim"
        ));
    }
    Ok(())
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

pub fn load_claim(root: &Path, path: &Path) -> Result<LoadedClaim, String> {
    let bytes = fs::read(path).map_err(|error| format!("read {}: {error}", path.display()))?;
    let parsed: ClaimFile = serde_yaml::from_slice(&bytes)
        .map_err(|error| format!("parse {}: {error}", path.display()))?;
    if parsed.version != 1 {
        return Err(format!("unsupported claim version: {}", parsed.version));
    }
    if parsed.id.trim().is_empty() || parsed.claim.trim().is_empty() {
        return Err("claim id and statement must not be empty".into());
    }
    if !parsed
        .id
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
    {
        return Err("claim id may contain only ASCII letters, digits, '-' and '_'".into());
    }
    if parsed.required_trials.is_empty() {
        return Err("claim must require at least one trial".into());
    }
    let defined_trials = parsed
        .trials
        .iter()
        .map(|trial| trial.id.as_str())
        .collect::<BTreeSet<_>>();
    for required in &parsed.required_trials {
        if !defined_trials.contains(required.as_str()) {
            return Err(format!("required trial has no definition: {required}"));
        }
    }
    if parsed
        .trials
        .iter()
        .any(|trial| trial.command.is_empty() || trial.command.iter().any(String::is_empty))
    {
        return Err("trial commands must contain non-empty arguments".into());
    }
    let path_refs = parsed
        .dependencies
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    let snapshot = snapshot_dependencies(root, &path_refs)
        .map_err(|error| format!("snapshot claim dependencies: {error}"))?;

    Ok(LoadedClaim {
        claim: Claim {
            id: parsed.id,
            statement: parsed.claim,
            required_trials: parsed.required_trials,
        },
        dependency_paths: parsed.dependencies,
        snapshot,
        trials: parsed.trials,
    })
}

pub fn snapshot_dependencies(root: &Path, paths: &[&str]) -> io::Result<DependencySnapshot> {
    let mut normalized = BTreeSet::new();
    for path in paths {
        let candidate = Path::new(path);
        if candidate.is_absolute()
            || candidate
                .components()
                .any(|component| matches!(component, Component::ParentDir | Component::RootDir))
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("dependency path must stay below the claim root: {path}"),
            ));
        }
        normalized.insert((*path).to_owned());
    }

    let mut files = BTreeMap::new();
    for relative in normalized {
        let bytes = fs::read(root.join(&relative))?;
        files.insert(relative, hex_digest(&bytes));
    }

    let mut preimage = Vec::new();
    for (path, digest) in &files {
        preimage.extend_from_slice(&(path.len() as u64).to_be_bytes());
        preimage.extend_from_slice(path.as_bytes());
        preimage.extend_from_slice(digest.as_bytes());
    }

    Ok(DependencySnapshot {
        identity: hex_digest(&preimage),
        files,
    })
}

pub fn changed_artifacts(root: &Path, evidence: &[Evidence]) -> io::Result<Vec<String>> {
    let mut changed = BTreeSet::new();
    for item in evidence {
        for (path, recorded_digest) in &item.artifacts {
            match fs::read(root.join(path)) {
                Ok(bytes) if hex_digest(&bytes) == *recorded_digest => {}
                Ok(_) | Err(_) => {
                    changed.insert(path.clone());
                }
            }
        }
    }
    Ok(changed.into_iter().collect())
}

pub fn evaluate(
    claim: &Claim,
    current: &DependencySnapshot,
    base_evidence: &[Evidence],
    head_evidence: &[Evidence],
) -> Warrant {
    let changed_dependencies = changed_dependencies(current, head_evidence);
    if !changed_dependencies.is_empty() {
        return Warrant {
            status: WarrantStatus::Stale,
            changed_dependencies,
            missing_trials: Vec::new(),
        };
    }

    if !base_evidence.is_empty()
        && base_evidence
            .iter()
            .any(|item| item.verdict != EvidenceVerdict::ExpectedFail)
    {
        return Warrant {
            status: WarrantStatus::Failed,
            changed_dependencies: Vec::new(),
            missing_trials: Vec::new(),
        };
    }

    let by_trial: BTreeMap<&str, &Evidence> = head_evidence
        .iter()
        .map(|evidence| (evidence.trial_id.as_str(), evidence))
        .collect();
    let missing_trials: Vec<_> = claim
        .required_trials
        .iter()
        .filter(|trial| !by_trial.contains_key(trial.as_str()))
        .cloned()
        .collect();
    if !missing_trials.is_empty() {
        return Warrant {
            status: WarrantStatus::Unsupported,
            changed_dependencies: Vec::new(),
            missing_trials,
        };
    }

    let failed = claim.required_trials.iter().any(|trial| {
        by_trial
            .get(trial.as_str())
            .is_some_and(|evidence| !matches!(evidence.verdict, EvidenceVerdict::Pass))
    });

    Warrant {
        status: if failed {
            WarrantStatus::Failed
        } else {
            WarrantStatus::Warranted
        },
        changed_dependencies: Vec::new(),
        missing_trials: Vec::new(),
    }
}

pub fn render_report(claim: &Claim, warrant: &Warrant, evidence: &[Evidence]) -> String {
    let mut report = format!("{}  {}\n", status_label(warrant.status), claim.id);
    writeln!(&mut report, "{}", claim.statement).expect("writing to a string cannot fail");
    if !warrant.changed_dependencies.is_empty() {
        report.push_str("\nChanged\n");
        for path in &warrant.changed_dependencies {
            writeln!(&mut report, "  {path}").expect("writing to a string cannot fail");
        }
    }
    if !warrant.missing_trials.is_empty() {
        report.push_str("\nRequired\n");
        for trial in &warrant.missing_trials {
            writeln!(&mut report, "  MISSING  {trial}").expect("writing to a string cannot fail");
        }
    }
    if !evidence.is_empty() {
        report.push_str("\nEvidence\n");
        for item in evidence {
            writeln!(
                &mut report,
                "  {:<6}{}  inputs {}",
                verdict_label(item.verdict),
                item.trial_id,
                &item.dependencies.identity[..12]
            )
            .expect("writing to a string cannot fail");
            if item.verdict == EvidenceVerdict::Fail && !item.observation.is_empty() {
                writeln!(&mut report, "        {}", item.observation)
                    .expect("writing to a string cannot fail");
            }
        }
    }
    report
}

fn first_observation(bytes: &[u8]) -> Option<String> {
    String::from_utf8_lossy(bytes)
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .map(|line| line.strip_prefix("FAIL: ").unwrap_or(line).to_owned())
}

fn status_label(status: WarrantStatus) -> &'static str {
    match status {
        WarrantStatus::Warranted => "WARRANTED",
        WarrantStatus::Failed => "FAILED",
        WarrantStatus::Stale => "STALE",
        WarrantStatus::Unsupported => "UNSUPPORTED",
    }
}

fn verdict_label(verdict: EvidenceVerdict) -> &'static str {
    match verdict {
        EvidenceVerdict::Pass => "PASS",
        EvidenceVerdict::Fail => "FAIL",
        EvidenceVerdict::ExpectedFail => "XFAIL",
    }
}

fn changed_dependencies(current: &DependencySnapshot, evidence: &[Evidence]) -> Vec<String> {
    let Some(recorded) = evidence.first().map(|item| &item.dependencies) else {
        return Vec::new();
    };
    if evidence
        .iter()
        .all(|item| item.dependencies.identity == current.identity)
    {
        return Vec::new();
    }

    current
        .files
        .keys()
        .chain(recorded.files.keys())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .filter(|path| current.files.get(*path) != recorded.files.get(*path))
        .cloned()
        .collect()
}

fn hex_digest(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(64);
    for byte in Sha256::digest(bytes) {
        write!(&mut output, "{byte:02x}").expect("writing to a string cannot fail");
    }
    output
}
