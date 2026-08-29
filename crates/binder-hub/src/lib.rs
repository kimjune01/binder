//! Static research hub for Binder public case fixtures.

use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CaseClass {
    AuthorizationReplay,
    UpgradeMigration,
    BuildDeploymentIdentity,
    InvariantScope,
    PostmortemRemediation,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PublicCase {
    pub schema_version: u8,
    pub id: String,
    pub title: String,
    pub ecosystem: String,
    pub class: CaseClass,
    pub summary: String,
    pub claim: String,
    pub evidence_boundary: String,
    pub decision: Decision,
    pub artifacts: Vec<Artifact>,
    pub edges: Vec<Edge>,
    pub missing_edges: Vec<String>,
    pub packets: Packets,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Decision {
    pub question: String,
    pub expected: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Artifact {
    pub id: String,
    pub kind: String,
    pub title: String,
    pub url: String,
    #[serde(default)]
    pub revision: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Edge {
    pub id: String,
    pub relation: String,
    pub from: String,
    pub to: String,
    pub status: String,
    #[serde(default)]
    pub citations: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Packets {
    pub control: String,
    pub curated: String,
    pub questions: String,
    pub answer_key: String,
}

pub fn load_cases(directory: &Path) -> Result<Vec<PublicCase>, String> {
    let entries =
        fs::read_dir(directory).map_err(|e| format!("read {}: {e}", directory.display()))?;
    let mut paths = entries
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|v| v.to_str()) == Some("json"))
        .collect::<Vec<_>>();
    paths.sort();
    paths
        .into_iter()
        .map(|path| {
            let bytes = fs::read(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
            let case: PublicCase = serde_json::from_slice(&bytes)
                .map_err(|e| format!("parse {}: {e}", path.display()))?;
            if case.schema_version != 1 {
                return Err(format!(
                    "{} uses unsupported schema version",
                    path.display()
                ));
            }
            Ok(case)
        })
        .collect()
}

pub fn render_hub(repo_root: &Path, cases: &[PublicCase], output: &Path) -> Result<(), String> {
    fs::create_dir_all(output).map_err(|e| format!("create output: {e}"))?;
    fs::write(output.join("index.html"), render_index(cases))
        .map_err(|e| format!("write index: {e}"))?;
    for case in cases {
        let dir = output.join("cases").join(&case.id);
        fs::create_dir_all(&dir).map_err(|e| format!("create case output: {e}"))?;
        fs::write(dir.join("index.html"), render_case(case))
            .map_err(|e| format!("write case: {e}"))?;
        fs::write(
            dir.join("case.json"),
            serde_json::to_vec_pretty(case).map_err(|e| e.to_string())?,
        )
        .map_err(|e| format!("write JSON: {e}"))?;
        for (packet, name) in [
            (&case.packets.control, "control.md"),
            (&case.packets.curated, "curated.md"),
            (&case.packets.questions, "questions.md"),
            (&case.packets.answer_key, "answer-key.md"),
        ] {
            let source = repo_root.join(packet);
            if !source.is_file() {
                return Err(format!("missing packet {packet}"));
            }
            fs::copy(source, dir.join(name)).map_err(|e| format!("copy packet: {e}"))?;
        }
    }
    fs::write(output.join("style.css"), STYLE).map_err(|e| format!("write style: {e}"))?;
    Ok(())
}

fn render_index(cases: &[PublicCase]) -> String {
    let cards = cases.iter().map(|c| format!(
        "<article><p class=\"eyebrow\">{} · {}</p><h2><a href=\"cases/{}/\">{}</a></h2><p>{}</p><p class=\"gap\">{} missing edge{}</p></article>",
        esc(&c.ecosystem), class_name(c.class), esc(&c.id), esc(&c.title), esc(&c.summary),
        c.missing_edges.len(), if c.missing_edges.len() == 1 { "" } else { "s" }
    )).collect::<String>();
    page(
        "Binder public cases",
        &format!(
            "<header><p class=\"eyebrow\">Research hub · sourced public artifacts</p><h1>Contract claims, joined without the trust theater.</h1><p class=\"lede\">Five consequential cases connect findings, revisions, checks, builds, and deployments while showing what remains unknown.</p></header><main class=\"grid\">{cards}</main>"
        ),
        "style.css",
    )
}

fn render_case(c: &PublicCase) -> String {
    let artifacts = c
        .artifacts
        .iter()
        .map(|a| {
            format!(
                "<li><span>{}</span><a href=\"{}\">{}</a>{}</li>",
                esc(&a.kind),
                esc(&a.url),
                esc(&a.title),
                a.revision
                    .as_ref()
                    .map(|r| format!(" <code>{}</code>", esc(r)))
                    .unwrap_or_default()
            )
        })
        .collect::<String>();
    let edges = c
        .edges
        .iter()
        .map(|e| {
            format!(
                "<li><strong>{}</strong>: {} → {} <mark class=\"{}\">{}</mark></li>",
                esc(&e.relation),
                esc(&e.from),
                esc(&e.to),
                esc(&e.status),
                esc(&e.status)
            )
        })
        .collect::<String>();
    let gaps = c
        .missing_edges
        .iter()
        .map(|g| format!("<li>{}</li>", esc(g)))
        .collect::<String>();
    let body = format!(
        "<nav><a href=\"../../\">← All cases</a><a href=\"case.json\">JSON</a></nav><header><p class=\"eyebrow\">{} · {}</p><h1>{}</h1><p class=\"lede\">{}</p></header><main><section><h2>Claim</h2><p>{}</p></section><section><h2>Decision</h2><p>{}</p><p><strong>Expected:</strong> {}</p></section><section><h2>Evidence boundary</h2><p>{}</p></section><section><h2>Public artifacts</h2><ul class=\"artifacts\">{}</ul></section><section><h2>Edges</h2><ul>{}</ul></section><section><h2>Missing edges</h2><ul>{}</ul></section><section><h2>Validation packets</h2><p><a href=\"control.md\">Control</a> · <a href=\"curated.md\">Curated</a> · <a href=\"questions.md\">Questions</a> · <a href=\"answer-key.md\">Answer key</a></p></section></main>",
        esc(&c.ecosystem),
        class_name(c.class),
        esc(&c.title),
        esc(&c.summary),
        esc(&c.claim),
        esc(&c.decision.question),
        esc(&c.decision.expected),
        esc(&c.evidence_boundary),
        artifacts,
        edges,
        gaps
    );
    page(&c.title, &body, "../../style.css")
}

fn page(title: &str, body: &str, css: &str) -> String {
    format!(
        "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><title>{}</title><link rel=\"stylesheet\" href=\"{}\"></head><body>{}</body></html>",
        esc(title),
        css,
        body
    )
}
fn class_name(c: CaseClass) -> &'static str {
    match c {
        CaseClass::AuthorizationReplay => "authorization / replay",
        CaseClass::UpgradeMigration => "upgrade / migration",
        CaseClass::BuildDeploymentIdentity => "build / deployment identity",
        CaseClass::InvariantScope => "invariant / proof scope",
        CaseClass::PostmortemRemediation => "postmortem / remediation",
    }
}
fn esc(v: &str) -> String {
    v.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

const STYLE: &str = r#":root{--ink:#17211c;--muted:#5d6962;--paper:#f4f1e8;--line:#d6d2c7;--accent:#155d3a}*{box-sizing:border-box}body{max-width:960px;margin:auto;padding:56px 24px 96px;background:var(--paper);color:var(--ink);font:17px/1.55 system-ui,sans-serif}h1{max-width:780px;margin:.15em 0 .35em;font:700 clamp(2.5rem,7vw,5.5rem)/.96 Georgia,serif;letter-spacing:-.045em}h2{font:700 1.45rem/1.15 Georgia,serif}a{color:var(--accent);text-underline-offset:3px}nav{display:flex;justify-content:space-between;margin-bottom:64px}.eyebrow{color:var(--muted);font-size:.78rem;font-weight:750;letter-spacing:.11em;text-transform:uppercase}.lede{max-width:700px;color:#39463f;font-size:1.15rem}.grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(280px,1fr));gap:16px;margin-top:64px}article,section{border-top:1px solid var(--line);padding:20px 0 28px}article{min-height:240px}.gap{color:#8c3b2d;font-size:.88rem}main:not(.grid){max-width:760px;margin-top:56px}li{margin:.55rem 0}.artifacts span{display:inline-block;min-width:120px;color:var(--muted);font-size:.8rem;text-transform:uppercase}code{font-size:.78em;overflow-wrap:anywhere}mark{margin-left:.4rem;padding:.12rem .35rem;border-radius:3px;background:#e2ddd0;font-size:.72rem;text-transform:uppercase}mark.sourced{background:#dcebdd;color:#174e2f}mark.inferred{background:#f1e3bd}mark.missing{background:#efd5cf;color:#7a2f22}"#;
