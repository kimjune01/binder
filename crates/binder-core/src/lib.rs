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
    pub required_trials: Vec<String>,
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
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WarrantStatus {
    Warranted,
    Failed,
    Stale,
    Unsupported,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Warrant {
    pub status: WarrantStatus,
    pub changed_dependencies: Vec<String>,
    pub missing_trials: Vec<String>,
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

pub fn evaluate(
    claim: &Claim,
    current: &DependencySnapshot,
    _base_evidence: &[Evidence],
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
