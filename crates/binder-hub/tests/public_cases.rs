use std::{collections::BTreeSet, path::PathBuf};

use binder_hub::{CaseClass, load_cases, render_hub};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn five_public_cases_cover_the_roadmap_classes() {
    let cases = load_cases(&repo_root().join("hub/cases")).expect("valid public cases");
    assert_eq!(cases.len(), 5);

    let classes = cases.iter().map(|case| case.class).collect::<BTreeSet<_>>();
    assert_eq!(
        classes,
        BTreeSet::from([
            CaseClass::AuthorizationReplay,
            CaseClass::UpgradeMigration,
            CaseClass::BuildDeploymentIdentity,
            CaseClass::InvariantScope,
            CaseClass::PostmortemRemediation,
        ])
    );
}

#[test]
fn factual_edges_are_cited_and_inferences_are_visible() {
    let cases = load_cases(&repo_root().join("hub/cases")).expect("valid public cases");

    for case in cases {
        assert!(!case.decision.question.trim().is_empty());
        assert!(!case.decision.expected.trim().is_empty());
        assert!(
            !case.missing_edges.is_empty(),
            "{} needs a known gap",
            case.id
        );

        for edge in case.edges {
            match edge.status.as_str() {
                "sourced" => assert!(
                    !edge.citations.is_empty(),
                    "{} sourced edge {} needs a citation",
                    case.id,
                    edge.id
                ),
                "inferred" | "missing" => {}
                other => panic!("{} edge {} has unknown status {other}", case.id, edge.id),
            }
        }
    }
}

#[test]
fn every_case_has_the_four_validation_packets() {
    let root = repo_root();
    let cases = load_cases(&root.join("hub/cases")).expect("valid public cases");

    for case in cases {
        for packet in [
            case.packets.control,
            case.packets.curated,
            case.packets.questions,
            case.packets.answer_key,
        ] {
            assert!(root.join(packet).is_file(), "{} packet is missing", case.id);
        }
    }
}

#[test]
fn renderer_emits_an_index_and_one_page_per_case() {
    let root = repo_root();
    let cases = load_cases(&root.join("hub/cases")).expect("valid public cases");
    let output = tempfile::tempdir().unwrap();

    render_hub(&root, &cases, output.path()).expect("hub renders");

    assert!(output.path().join("index.html").is_file());
    for case in cases {
        let page = output.path().join(format!("cases/{}/index.html", case.id));
        assert!(page.is_file(), "{} page is missing", case.id);
        let html = std::fs::read_to_string(page).unwrap();
        assert!(html.contains(&case.title));
        assert!(html.contains("Evidence boundary"));
        assert!(html.contains("Missing edges"));
        for packet in ["control.md", "curated.md", "questions.md", "answer-key.md"] {
            assert!(
                output
                    .path()
                    .join("cases")
                    .join(&case.id)
                    .join(packet)
                    .is_file(),
                "{} rendered packet {packet} is missing",
                case.id
            );
            assert!(html.contains(packet), "{} does not link {packet}", case.id);
        }
    }
}
